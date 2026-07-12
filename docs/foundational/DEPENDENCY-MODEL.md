# CyborgHeart Dependency Model

**Status:** Initial immutable-fact contract
**Matrix baseline:** v1.19
**Initial room version:** 12 only
**Last reviewed:** 2026-07-12

---

## 1. Purpose

This document defines the immutable facts that CyborgHeart event application may consume, how missing facts are requested, how repeated evaluation works, and which related information is deliberately not a dependency.

The central rule is:

> The engine names the fact it needs. The host decides how to obtain it.

A dependency is never a database query, federation request, retry instruction, or transport operation.

---

## 2. Goals

The dependency model must make event evaluation:

- deterministic;
- storage-neutral;
- network-neutral;
- resumable by restart;
- safe under concurrent room activity;
- explicit about missing information;
- testable from fixtures;
- suitable for future durable execution.

It must prevent:

- hidden I/O;
- missing local data being classified as protocol invalidity;
- mutable current state changing during one evaluation sequence;
- opaque “fetch everything” requests;
- storage schemas leaking into the event engine;
- deferred consequences blocking a completed event decision.

---

## 3. Fact versus operation

Correct dependency:

```text
Event { event_id }
CurrentRoomState { room_id }
ServerSigningKeys { server, key_ids, event_origin_server_ts, key_validity_reference_ts }
```

Incorrect dependency:

```text
FetchEventFromFederation
QueryEventsTable
CallPolicyServer
RetryAfterDelay
ResolveStateRemotely
```

The latter are host operations.

The same fact may be supplied by:

- a fixture;
- memory;
- an embedded store;
- PostgreSQL;
- a cache;
- federation;
- an import archive.

The engine must not observe the source.

---

## 4. Dependency snapshot

One evaluation receives an immutable dependency snapshot.

Conceptually:

```rust
struct DependencySnapshot<'a> {
    events: &'a dyn EventView,
    states: &'a dyn StateView,
    signing_keys: &'a dyn SigningKeyView,
    policy_recommendations: &'a dyn PolicyRecommendationView,
}
```

The exact traits may change, but the snapshot must have these properties:

1. lookups are side-effect free;
2. repeated lookup of the same fact yields the same answer;
3. returned collections have deterministic ordering or are normalized;
4. facts cannot change during one evaluation call;
5. a dependency-completion sequence uses one frozen current-state revision;
6. conflicting facts are rejected before evaluation rather than converted into an event disposition.

---

## 5. Dependency identity

Dependencies need stable identity so they can be deduplicated, ordered, logged, tested, and later persisted by the transaction runtime.

Initial conceptual model:

```rust
enum Dependency {
    Event {
        event_id: OwnedEventId,
    },

    StateAfter {
        event_id: OwnedEventId,
    },

    CurrentRoomState {
        room_id: OwnedRoomId,
        expected_revision: Option<CurrentStateRevision>,
    },

    ServerSigningKeys {
        event_id: OwnedEventId,
        server: OwnedServerName,
        key_ids: BTreeSet<OwnedServerSigningKeyId>,
        event_origin_server_ts: MilliSecondsSinceUnixEpoch,
        key_validity_reference_ts: MilliSecondsSinceUnixEpoch,
    },

    PolicyRecommendation {
        event_id: OwnedEventId,
        policy_event_id: OwnedEventId,
        policy_fingerprint: PolicyFingerprint,
    },
}
```

This is a domain sketch, not a frozen Rust signature.

Dependencies should be returned in a deterministic ordered set.

---

## 6. Event facts

### Identity

```text
Event { event_id }
```

### Required content

An event fact must contain enough information to implement Ruma’s server-side `Event` abstraction and CyborgHeart lifecycle:

- canonical event representation;
- authoritative event ID;
- room ID when present;
- sender;
- origin timestamp;
- event type;
- raw content;
- state key;
- predecessor IDs;
- auth-event IDs;
- redaction target;
- prior disposition metadata required by Matrix algorithms.

### Disposition metadata

Referenced event facts must distinguish at least:

- accepted;
- rejected;
- soft-failed;
- unknown or not yet evaluated.

Ruma’s current `Event` trait requires a rejected flag. CyborgHeart may retain a richer internal disposition and adapt the exact value Ruma needs.

An unevaluated event is not silently treated as accepted.

### Identity validation

The snapshot must reject an event fact when:

- its derived or authoritative event ID differs from the lookup key;
- its room identity conflicts with the room being evaluated;
- the same event ID maps to different canonical events;
- its stored disposition metadata is internally contradictory.

These are fact-integrity faults, not Matrix dispositions for the candidate event.

### Missing event

If a required event fact is absent:

```text
NeedsDependencies(need.event)
```

The engine should request all directly discoverable missing event IDs for the current stage in one deterministic batch.

---

## 7. State-after facts

### Identity

```text
StateAfter { event_id }
```

### Meaning

A state-after fact is the authoritative room state immediately after the identified event under the previously committed Matrix decision.

It is not:

- current room state;
- client-side state;
- a database snapshot format;
- an arbitrary state map supplied by a remote server without validation.

### Use

For one predecessor:

```text
state_before(candidate) = state_after(predecessor)
```

For multiple predecessors, the engine resolves all predecessor state-after maps.

### Validation

A state-after fact must:

- be associated with the same room;
- use canonical state tuple keys;
- reference event IDs consistently;
- have a stable fact identity;
- be immutable during evaluation.

The snapshot may retain an internal digest or version to detect contradictory duplicates, but the event engine does not prescribe the storage representation.

### Missing state after

```text
NeedsDependencies(need.state_after)
```

### State before is derived

CyborgHeart does not initially accept `StateBefore(candidate)` as an opaque dependency.

State before is derived from:

- the create-event empty state;
- a predecessor’s state after;
- or room-version-12 state resolution across predecessor states.

This prevents a host from bypassing the lifecycle by injecting an unexplained historical state.

A future optimization may admit a cached state-before fact only if it is cryptographically or transactionally bound to the same predecessor set and can be verified without weakening the model.

---

## 8. Current-room-state facts

### Identity

```text
CurrentRoomState { room_id, expected_revision }
```

### Meaning

Current room state is the server’s frozen resolved state used for:

- current-state authorization;
- Policy Server enablement and key selection.

It is distinct from historical state before the candidate.

### Revision

The fact must include an opaque revision or snapshot token.

The token need not have Matrix meaning. It exists to ensure one dependency-completion sequence does not silently cross current-state versions.

Example:

```rust
struct CurrentRoomStateFact {
    room_id: OwnedRoomId,
    revision: CurrentStateRevision,
    state: StateMap<OwnedEventId>,
}
```

### Concurrency rule

During one evaluation sequence:

```text
candidate + room version + current-state revision
```

must remain fixed.

If the transaction runtime detects that current state changed before commit, it starts a new evaluation sequence against a new frozen revision. It does not extend the old snapshot and pretend dependency monotonicity still applies.

### Missing current state

```text
NeedsDependencies(need.current_room_state)
```

### Current state and local commands

The future command runtime may already have current state while constructing a local event. It must still supply that state explicitly to the authoritative event path rather than bypassing the current-state check.

---

## 9. Signing-key facts

### Dependency identity

```text
ServerSigningKeys {
    event_id,
    server,
    key_ids,
    event_origin_server_ts,
    key_validity_reference_ts,
}
```

### Discovery

The engine first uses Ruma to determine required server identities.

For each required server it then inspects the event’s signature map and determines candidate key IDs that:

- are present on the event;
- use a supported signing algorithm;
- belong to the required server identity.

### Fact content

A key fact should contain:

- server name;
- signing-key ID;
- public-key bytes;
- `valid_until_ts`, or normalized equivalent validity metadata;
- source-independent verification metadata where required.

One evaluation sequence also carries an explicit frozen `key_validity_reference_ts`. The host chooses this reference time when beginning the sequence and retains it for retries and replay evidence. The engine does not read the wall clock.

For room version 12, a supplied key is eligible only when:

```text
origin_server_ts <= min(valid_until_ts, key_validity_reference_ts + 7 days)
```

CyborgHeart performs this metadata check before providing eligible key bytes to Ruma. Ruma verifies cryptographic signatures; it does not acquire keys or establish this external validity metadata.

The engine does not own:

- key discovery;
- notary policy;
- cache expiry;
- network retries;
- key persistence.

### Missing versus invalid

**Missing locally:**

```text
NeedsDependencies(need.signing_keys)
```

**No required signature present on the event:**

```text
Dropped(drop.missing_required_signature)
```

**All relevant eligible key facts resolved and no required signature verifies:**

```text
Dropped(drop.invalid_required_signature)
```

A single unavailable, unknown, unsupported, or expired key does not prove invalidity when another candidate signature may still verify. Unknown and expired keys are ignored according to Matrix rules; the engine reaches a drop only after the required entity's usable candidate set is complete.

**Key acquisition ultimately fails:**

This is a host/runtime failure to complete evaluation. It does not prove that the event’s signature is invalid. The engine remains incomplete unless Matrix defines a specific final semantic outcome for that dependency.

### Deterministic key requests

Requests should include all candidate key IDs for each required server rather than requesting one key at a time in event-map iteration order.

---

## 10. Auth-chain and graph dependencies

CyborgHeart does not define opaque dependencies named:

```text
AuthChain(event_id)
ResolveState(...)
ConflictedStateSubgraph(...)
```

Those names hide algorithms and acquisition scope.

Instead, the engine:

1. traverses known event facts;
2. discovers missing concrete auth-event or predecessor IDs;
3. requests those events;
4. computes full auth-chain sets locally;
5. computes the conflicted state subgraph locally from event facts;
6. invokes Ruma state resolution only after preflight is complete.

This makes every missing unit inspectable and fixture-friendly.

### Batching

The engine requests every missing concrete event discoverable at the current traversal frontier.

It must not continue into later lifecycle stages when the missing facts prevent a sound result.

### Work accounting

Every traversal charges:

- event inspections;
- auth-chain edges;
- graph edges;
- state entries;
- state-resolution steps.

---

## 11. Policy recommendation facts

Policy Server recommendation is the one initial dependency whose final absence can have Matrix meaning.

### Identity

```text
PolicyRecommendation {
    event_id,
    policy_event_id,
    policy_fingerprint,
}
```

The binding prevents a recommendation from being reused against:

- another candidate event;
- another active policy event;
- another policy public key.

### Policy fingerprint

The fingerprint should cover the active policy inputs needed to prevent ambiguity, including:

- policy state event ID;
- `via` server;
- policy public key;
- relevant room-version signing rules.

The exact encoding is internal and versioned.

### Fact states

Conceptually:

```rust
enum PolicyRecommendationFact {
    Signature {
        signer: OwnedServerName,
        key_id: OwnedServerSigningKeyId,
        signature: Base64Signature,
    },

    FinalUnavailable {
        attempted_at_policy_fingerprint: PolicyFingerprint,
    },
}
```

### Initial absence

When policy applies and no valid fact exists:

```text
NeedsDependencies(need.policy_recommendation)
```

The host may call the Policy Server.

### Detached recommendation

The host may supply the recommendation as a detached fact rather than mutating the candidate event object.

This is safe because the additional policy signature does not change the event’s reference hash or event ID.

The engine reconstructs the correct signed representation and verifies the detached signature.

### Final unavailable

After the host performs the required acquisition attempt and records final unavailability:

```text
SoftFailed(soft_fail.policy_recommendation)
```

This sentinel is allowed only because Matrix assigns meaning to final recommendation failure.

It is not a general pattern for turning unavailable dependencies into protocol decisions.

### Same-name Policy Server

A detached fact avoids overwriting or conflating normal origin signatures when the Policy Server uses the same server name as the event origin. Verification is bound to the dedicated policy-server key ID.

---

## 12. Deferred redaction partners are not decision dependencies

An accepted `m.room.redaction` event can receive a complete disposition without its target event being locally available.

Therefore:

```text
missing redaction target
    ≠ NeedsDependencies for event disposition
```

The semantic result contains:

```text
consequence.redaction_pending_partner
```

This covers a missing target, a partner that is not yet valid, or a currently nonqualifying pair. Matrix requires later re-checking rather than a terminal denial.

The future transaction runtime records the pending relation. When relevant partner facts change, it evaluates the redaction consequence separately.

The partner becomes an input to consequence completion, not a dependency of the original event decision.

---

## 13. Missing, known absent, and conflicting facts

### Missing

The snapshot has no authoritative answer.

Result:

```text
NeedsDependencies
```

### Known absent

Use only where the absence itself is authoritative and semantically meaningful.

Initial example:

```text
PolicyRecommendationFact::FinalUnavailable
```

Do not introduce general `EventDoesNotExist` or `SigningKeyUnavailable` facts to force an event decision where Matrix does not define one.

### Conflicting

Two facts claim the same identity but differ materially.

Examples:

- two canonical events for one event ID;
- two public keys for the same server and key ID with overlapping validity;
- two current-state maps for the same revision;
- a state-after fact bound to the wrong room.

Conflicts are snapshot-construction or host-contract faults.

They must not become:

- `Dropped`;
- `Rejected`;
- `SoftFailed`;
- `NeedsDependencies`.

Evaluation should not begin with a conflicting snapshot.

---

## 14. Dependency request

Conceptually:

```rust
struct DependencyNeed {
    requests: DependencyRequestSet,
    evidence: DecisionEvidence,
}

struct DependencyRequest {
    stage: EvaluationStage,
    code: DependencyCode,
    required: NonEmptyBTreeSet<Dependency>,
}
```

### Stable properties

A request must be:

- deterministic;
- deduplicated;
- non-empty;
- homogeneous by dependency code;
- minimally sufficient for the current stage;
- explicit about fact identity;
- independent of the host’s acquisition mechanism.

Duplicate request groups are merged by `(stage, code)`. The stage remains contextual; the dependency code identifies the required fact family directly and is never optional after construction.

### Stage field

The stage is diagnostic and may help the runtime prioritize work.

It is not a serialized continuation and correctness must not depend on it.

### No request loops without progress

If the same candidate, options, budget, and snapshot return the same request after the caller claims to have satisfied it, the runtime must detect lack of progress.

Possible causes:

- the wrong fact was supplied;
- the fact identity was incomplete;
- the engine failed to observe the new fact;
- a dependency cycle was not expanded correctly.

The transaction runtime will eventually own progress detection and retry limits.

---

## 15. Snapshot extension and deterministic restart

The initial resumability model is:

```text
snapshot A
    ↓ evaluate
NeedsDependencies {D1, D2}
    ↓ acquire
snapshot B = A + D1 + D2
    ↓ evaluate from start
Complete | NeedsDependencies | Halted
```

Requirements:

- the candidate is unchanged;
- the supported room version is unchanged;
- evaluation options are unchanged;
- current-state revision is unchanged;
- existing facts are not replaced;
- new facts do not conflict;
- the engine restarts from the beginning.

Opaque internal continuation state is prohibited in the initial public contract.

---

## 16. Dependency monotonicity

For snapshots `A` and `B` where `B` is a valid extension of `A`:

```text
A ⊆ B
```

Allowed:

```text
NeedsDependencies → NeedsDependencies
NeedsDependencies → Complete
NeedsDependencies → Halted
Halted → Complete with a larger budget
```

Forbidden for the same candidate, options, budget, and frozen current-state revision:

```text
Complete(Accepted) → Complete(Rejected)
Complete(Rejected) → Complete(SoftFailed)
Complete(Dropped) → any other disposition
```

A complete decision must only be returned when all facts capable of influencing that decision are present.

### Budget qualification

Dependency monotonicity assumes the same budget.

A larger budget may allow an evaluation that previously halted to complete.

### Current-state qualification

A different current-state revision starts a new evaluation sequence. It is not an extension of the old snapshot.

---

## 17. Fact consistency checks

Before evaluation, the snapshot or its builder should validate:

- unique fact identities;
- event ID to canonical-event consistency;
- room consistency;
- current-state revision uniqueness;
- state tuple validity;
- signing-key identity and encoding;
- policy recommendation binding;
- deterministic collection normalization.

During evaluation, defensive checks may still detect impossible conditions. Those are implementation or host faults and should be surfaced separately from Matrix outcomes.

---

## 18. Security properties

The dependency model supports security by ensuring:

- no hidden network access;
- no ambient database authority;
- no unbounded “load full room” request;
- no silent mutation of current state;
- no key use without event-time context;
- no policy signature reuse across policies or events;
- no unverified redaction target blocking the event path;
- explicit graph and auth-chain work accounting.

Fact providers remain responsible for authenticating and validating externally acquired data before presenting it as authoritative fact.

The event engine still validates all protocol-relevant relationships.

---

## 19. Example — missing signing keys

```text
candidate event
    sender server: example.org
    signature key: ed25519:7
    origin_server_ts: T

snapshot
    key_validity_reference_ts: R
    no matching eligible public key
```

Result:

```text
NeedsDependencies {
    need.signing_keys(
        server = example.org,
        key_ids = {ed25519:7},
        event_origin_server_ts = T,
        key_validity_reference_ts = R
    )
}
```

After the key fact is added, evaluation restarts and verifies the signature.

---

## 20. Example — branching state

```text
candidate.prev_events = {A, B}
```

Snapshot contains:

- events A and B;
- state after A;
- state after B;
- some but not all auth-chain events.

The engine traverses known auth references and returns all concrete missing event IDs at the current frontier.

It does not ask the host to “resolve state.”

After the missing events are added, the engine computes auth chains and the conflicted state subgraph, then calls Ruma `resolve()`.

---

## 21. Example — Policy Server recommendation

Current state enables policy event `P` with fingerprint `F`.

No valid recommendation fact is present.

Result:

```text
NeedsDependencies {
    need.policy_recommendation(
        event_id = E,
        policy_event_id = P,
        policy_fingerprint = F
    )
}
```

If the host supplies a valid detached signature, evaluation continues.

If the host supplies `FinalUnavailable(F)`, evaluation completes as:

```text
SoftFailed(soft_fail.policy_recommendation)
```

---

## 22. Example — missing redaction target

Accepted redaction event `R` targets an unavailable or not-yet-valid event `T`.

Result:

```text
Complete(Accepted) {
    redaction_effect = pending_partner(T)
}
```

No event-decision dependency request is produced for `T`.

---

## 23. Future transaction-runtime obligations

The future transaction runtime will:

- translate dependency identities into store, cache, federation, or Policy Server operations;
- deduplicate acquisition work;
- preserve one frozen current-state revision per evaluation sequence;
- enforce aggregate submission budgets and retry limits;
- detect no-progress loops;
- re-evaluate after dependencies arrive;
- atomically commit final consequences;
- retain pending consequence relationships;
- distinguish transient acquisition failure from final Matrix meaning.

None of those responsibilities move into the event engine.

---

## 24. Non-goals

The initial dependency model does not define:

- repository traits for a database;
- federation endpoints;
- network retry policy;
- cache expiration;
- transaction isolation implementation;
- serialized internal continuations;
- arbitrary remote state snapshots;
- a generic dependency-injection framework.

---

## 25. Final dependency commitment

> CyborgHeart evaluates events only from explicit immutable facts. Missing decision facts produce deterministic dependency requests, conflicting facts prevent evaluation, current state is frozen by revision, and host operations remain outside the engine. Concrete event and state facts—not opaque remote procedures—drive authorization and state resolution.

---

## Companion documents

- [`EVENT-LIFECYCLE.md`](./EVENT-LIFECYCLE.md)
- [`DECISION-CODES.md`](./DECISION-CODES.md)
- [`ARCHITECTURE.md`](./ARCHITECTURE.md)
- [`RUMA-COVERAGE.md`](./RUMA-COVERAGE.md)

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

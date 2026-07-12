# CyborgHeart Event Lifecycle

**Status:** Initial lifecycle contract
**Matrix baseline:** v1.19
**Initial room version:** 12 only
**Last reviewed:** 2026-07-12

---

## 1. Purpose

This document defines the exact stage order for applying one candidate Matrix room event in CyborgHeart.

It owns:

- lifecycle order;
- stage inputs and outputs;
- dependency points;
- drop, rejection, soft-failure, acceptance, and halt points;
- redacted-verification behavior;
- Policy Server recommendation behavior;
- semantic consequence calculation.

It does not define database access, federation fetching, retry schedules, or API transport.

---

## 2. Entry contract

The event engine evaluates one candidate event at a time.

Conceptually:

```rust
fn evaluate_event(
    input: EvaluationInput<'_>,
    facts: &DependencySnapshot<'_>,
    budget: &mut EvaluationBudget,
) -> EvaluationResult;
```

The explicit inputs are:

- a canonical candidate event;
- one `SupportedRoomVersion`;
- event provenance;
- evaluation options;
- a frozen key-validity reference timestamp;
- an immutable dependency snapshot;
- a declared work budget.

The engine performs no I/O and reads no ambient state.

### Room-version admission

An external `RoomVersionId` must first map to:

```rust
enum SupportedRoomVersion {
    V12,
}
```

Failure to map is:

```text
capability.unsupported_room_version
```

It occurs before event evaluation and is not a Matrix `Dropped` disposition.

---

## 3. Result model

```text
EvaluationResult
├── Complete
│   ├── Dropped
│   └── Admitted
│       ├── Accepted
│       ├── Rejected
│       └── SoftFailed
├── NeedsDependencies
└── Halted
    └── BudgetExceeded
```

Definitions:

- **Dropped:** the candidate failed before normal room-history participation.
- **Rejected:** the candidate was admitted to history processing but failed state-independent, claimed-auth, or historical authorization.
- **SoftFailed:** the candidate passed historical authorization but failed current-state authorization or the final required Policy Server recommendation check.
- **Accepted:** all required event-decision checks passed.
- **NeedsDependencies:** required immutable facts are not present.
- **BudgetExceeded:** evaluation stopped without a Matrix disposition.

`Duplicate` and `Committed` belong to the future transaction runtime.

---

## 4. Lifecycle overview

```text
0. supported room-version admission
        ↓
1. canonical candidate intake
        ↓
2. PDU format validation
        ↓
3. event identity and required signer discovery
        ↓
4. signature and content-hash verification
        ↓
5. effective authorization representation
        ↓
6. state-independent authorization
        ↓
7. authorization against claimed auth events
        ↓
8. state-before reconstruction
        ↓
9. authorization against historical state
        ↓
10. authorization against current room state
        ↓
11. Policy Server recommendation validation
        ↓
12. final disposition and semantic consequences
        ↓
13. evidence and work report
```

The stage order is normative for CyborgHeart’s implementation.

---

## 5. Stage 0 — supported room-version admission

### Input

- external room-version identifier.

### Operation

Map the identifier into `SupportedRoomVersion` and obtain one immutable Ruma `RoomVersionRules` bundle.

### Output

- supported room-version value;
- applicable event-format, signature, authorization, redaction, and state-resolution rules.

### Failure

Unsupported version:

```text
capability.unsupported_room_version
```

No event decision exists yet.

### Invariant

Room-version checks must not be scattered throughout the engine. All later stages receive the selected rules.

---

## 6. Stage 1 — canonical candidate intake

### Input

- `CanonicalJsonObject`;
- optional externally supplied expected event identity;
- provenance.

### Operation

Construct the private candidate wrapper and record:

- received canonical representation;
- provenance;
- any externally asserted event ID;
- initial byte cost.

### Output

- `CandidateEvent`.

### Failure boundary

Raw JSON parsing and conversion to canonical JSON occur before this stage. Failure there is an input-adapter error, not a Matrix disposition.

If the canonical object cannot be serialized or inspected consistently after admission, evaluation may drop it as unrecoverable canonical event data.

### Budget

Charge canonical JSON bytes before expensive processing.

---

## 7. Stage 2 — PDU format validation

### Ruma operation

```rust
check_pdu_format(candidate, &rules.event_format)
```

### Checks

The released Ruma function covers total PDU size and a room-version-sensitive subset of the shape, including:

- `type` and `sender`;
- room-version-sensitive `room_id` and `event_id`;
- optional `state_key`;
- identifier lengths;
- `prev_events` and `auth_events` limits;
- non-negative, canonical-integer `depth`;
- room-version-12 create-event relationships encoded by format rules.

Before constructing the infallible Ruma `Event` adapter, CyborgHeart also performs a narrow structural preflight for required fields not covered by the helper, including `content`, `origin_server_ts`, `hashes`, and `signatures`. This preflight validates only protocol-required structure and must not turn strict typed content deserialization into an extra rejection rule.

### Output

- format-checked candidate.

### Final outcome

Any format failure:

```text
Complete(Dropped(drop.invalid_pdu_format))
```

### Dependencies

None.

A malformed event is not made complete by fetching more data.

---

## 8. Stage 3 — event identity and required signer discovery

### Event identity

For room version 12, the event ID is derived from the reference hash rather than trusted from an event-body field.

CyborgHeart must:

1. calculate the reference hash through Ruma;
2. derive the event ID using room-version-12 rules;
3. compare it with any external expected identity;
4. retain the derived ID as the authoritative candidate identity.

An external identity mismatch produces:

```text
drop.event_id_mismatch
```

### Required signers

Use:

```rust
required_server_signatures_to_verify_event(
    candidate,
    &rules.signatures,
)
```

Then inspect the candidate’s signature map for supported key IDs belonging to each required server.

### Dependency decision

If required signatures are present in the event but the corresponding public keys are not in the dependency snapshot:

```text
NeedsDependencies(need.signing_keys)
```

The request includes:

- required server;
- candidate key IDs present on the event;
- event `origin_server_ts`;
- the frozen key-validity reference timestamp;
- candidate event ID.

Before constructing Ruma's public-key map, CyborgHeart filters supplied key facts using Matrix room-version-12 validity rules. A key is eligible only when its effective validity ceiling covers the event timestamp:

```text
origin_server_ts <= min(valid_until_ts, key_validity_reference_ts + 7 days)
```

The reference timestamp is supplied explicitly and frozen for the evaluation sequence. The engine never reads the wall clock. Unknown, unavailable, unsupported, or expired keys are not evidence of an invalid signature while another candidate key can still be acquired or verified.

### Drop conditions

- a required server has no signature in the event;
- all candidate signatures for a required server use unsupported or malformed key IDs;
- signature structure is malformed.

These map to stable cryptographic drop codes, not missing dependencies.

---

## 9. Stage 4 — signature and content-hash verification

### Ruma operation

```rust
verify_event(public_keys, candidate, room_version_rules)
```

### Branch A — all verification succeeds

```rust
Verified::All
```

Meaning:

- required signatures are valid;
- content hash matches.

The effective authorization representation is the received event.

### Branch B — signatures succeed, content hash differs

```rust
Verified::Signatures
```

Meaning:

- required signatures are valid;
- content hash does not match;
- this is not automatically forgery.

CyborgHeart must create or retain the room-version-redacted representation used for all later authorization and semantic interpretation.

The result evidence records:

```text
content_integrity = redacted_after_hash_mismatch
```

Unverified content must not influence later protocol decisions.

### Drop conditions

After all required key facts are present:

- invalid required signature;
- malformed signature encoding;
- malformed integrity fields;
- unrecoverable verification error.

### Invariant

A locally missing public key must be detected before `verify_event()` and must not be misclassified as an invalid event.

---

## 10. Stage 5 — effective authorization representation

The lifecycle carries two representations when necessary:

```text
received representation
    exact canonical event supplied to CyborgHeart

effective representation
    event representation permitted to influence authorization
```

Normally they are the same.

After `Verified::Signatures`, the effective representation is the room-version-redacted form.

The event ID remains stable because signatures and redaction-sensitive fields are handled according to the reference-hash rules.

The eventual durable runtime must not treat unverified received content as authoritative simply because it was retained for evidence or transport fidelity.

---

## 11. Stage 6 — state-independent authorization

### Preflight dependencies

Before calling Ruma, CyborgHeart must resolve:

- every directly referenced `auth_event`;
- the room-version-12 create event derived from the room ID when required;
- each referenced event’s known rejected status.

Any absent event fact produces:

```text
NeedsDependencies(need.event)
```

### Create-event room-version preflight

If an `m.room.create` event contains `content.room_version`, CyborgHeart checks that the value is a recognized Matrix room version before calling Ruma. An unrecognized value is a state-independent authorization rejection. A recognized value is not required by the Matrix authorization rule to equal the externally selected room version and must not be rejected for that reason alone. The released Ruma function assumes this recognition check has already occurred.

### Ruma operation

```rust
check_state_independent_auth_rules(
    &rules.authorization,
    effective_event,
    fetch_event,
)
```

### Final outcome

Failure after complete dependency preflight:

```text
Complete(Admitted(Rejected(
    reject.state_independent_auth
)))
```

### Why rejected, not dropped

Matrix and Ruma classify this stage as authorization after format and cryptographic admission. The event has entered room-history processing even though it cannot affect ordinary state or visibility.

---

## 12. Stage 7 — authorization against claimed auth events

### Input

The candidate’s claimed `auth_events`, represented as the exact state tuple map required by the authorization rules.

### Preflight

- all claimed auth events are present;
- duplicates and unexpected tuples have already been checked state-independently;
- the state view is deterministic and complete.

### Ruma operation

```rust
check_state_dependent_auth_rules(
    &rules.authorization,
    effective_event,
    claimed_auth_state,
)
```

### Final outcome

Failure:

```text
Complete(Admitted(Rejected(
    reject.claimed_auth
)))
```

### Invariant

The claimed auth-event check is not the historical-state check. Both are required.

---

## 13. Stage 8 — state-before reconstruction

State before the candidate is derived from its predecessors.

### Create event

For the room’s create event:

```text
state_before = empty
```

subject to the room-version create rules already checked.

### No predecessors on a non-create event

This is invalid under the relevant format or authorization path and must not be treated as an empty ordinary state.

### One predecessor

```text
state_before = state_after(predecessor)
```

If the predecessor or its state-after fact is absent:

```text
NeedsDependencies(need.event | need.state_after)
```

### Multiple predecessors

1. load every predecessor;
2. load the state after every predecessor;
3. construct the full auth chain for each predecessor state;
4. discover any missing concrete auth events;
5. calculate the room-version-12 conflicted state subgraph;
6. discover any missing graph events;
7. call Ruma `resolve()`.

### Ruma operation

```rust
resolve(
    &rules.authorization,
    &rules.state_res,
    predecessor_states,
    auth_chains,
    fetch_event,
    fetch_conflicted_state_subgraph,
)
```

### Dependency rules

The engine requests concrete facts:

- events by ID;
- state after predecessor events.

It does not request an opaque “auth chain” or “resolved state” operation.

### Budget

Charge:

- state entries inspected;
- events loaded;
- auth-chain edges;
- graph edges;
- state-resolution steps.

### Halt

Budget exhaustion:

```text
Halted(halt.budget_exceeded)
```

### Internal fault

If Ruma’s fetch callbacks fail after CyborgHeart declared preflight complete, the snapshot or traversal implementation violated its contract. This is not a Matrix rejection.

---

## 14. Stage 9 — authorization against historical state

### Input

- effective event representation;
- reconstructed state before.

### Ruma operation

```rust
check_state_dependent_auth_rules(
    &rules.authorization,
    effective_event,
    state_before,
)
```

### Final outcome

Failure:

```text
Complete(Admitted(Rejected(
    reject.historical_auth
)))
```

### Consequence

A rejected state event does not update state after the event.

The event may still need to be retained for graph and federation semantics.

---

## 15. Stage 10 — authorization against current room state

### Dependency

A frozen current-room-state fact is required:

```text
NeedsDependencies(need.current_room_state)
```

The fact must identify the room and an opaque current-state revision. It must remain unchanged across one dependency-completion sequence.

### Ruma operation

```rust
check_state_dependent_auth_rules(
    &rules.authorization,
    effective_event,
    current_room_state,
)
```

### Final outcome

Failure:

```text
Complete(Admitted(SoftFailed(
    soft_fail.current_state_auth
)))
```

### Consequence

A soft-failed event:

- is not presented as an ordinary client-visible event;
- is not selected as a local forward extremity;
- remains available for federation and later state-resolution participation.

---

## 16. Stage 11 — Policy Server recommendation validation

This stage occurs after current-state authorization.

### Determine whether a Policy Server is active

Using current room state, determine whether:

- a valid `m.room.policy` state event enables a Policy Server;
- the configured `via` server has the required joined-user relationship;
- the candidate is exempt because it is the empty-state-key `m.room.policy` event.

If no Policy Server is active, continue.

### Validate a supplied recommendation

If an active policy applies, verify the `ed25519:policy_server` recommendation against:

- the candidate event’s signing representation;
- the public key embedded in the active policy state;
- the configured `via` identity.

### Missing or invalid recommendation before host attempt

Return:

```text
NeedsDependencies(need.policy_recommendation)
```

The pure engine does not call the Policy Server.

### Host responsibility

The future host obtains or refreshes a recommendation and re-evaluates with one of:

- a detached recommendation signature fact;
- a final-unavailable recommendation fact after the required attempt.

### Final outcome

If a valid recommendation is supplied, continue to acceptance.

If the host records final unavailability or invalidity after the required attempt:

```text
Complete(Admitted(SoftFailed(
    soft_fail.policy_recommendation
)))
```

### Same-name rule

When the Policy Server identity matches the origin server name, normal origin signatures must remain distinct and preserved. Policy verification must target the dedicated policy-server key ID rather than replacing the origin-signature set.

### Local commands

A future command runtime should normally refuse the local action when it cannot obtain a required recommendation. If a local event is deliberately submitted anyway, the same event lifecycle applies and may soft-fail it.

---

## 17. Stage 12 — final disposition and semantic consequences

### Accepted event

An event reaches `Accepted` only after all required decision stages succeed.

The engine then calculates storage-neutral consequences.

### Historical state effect

The engine records the state at the candidate's historical position.

For an admitted message event that passes historical authorization:

```text
state_after = state_before
```

For an admitted state event that passes historical authorization, including an event that is later soft-failed by the current-state or Policy Server check:

```text
state_after[(type, state_key)] = candidate_event_id
```

For a rejected event:

```text
state_after = state_before
```

This distinction is necessary because a soft-failed event remains part of room history and may participate in later state resolution even though it is not immediately applied to the server's current resolved room state.

### Current-room application

- An accepted event may contribute to the current resolved room state through ordinary graph processing.
- A soft-failed event does not immediately update current room state and does not become a local forward extremity, but its retained historical state-after remains available if later events reference it.
- A rejected event does not update historical or current room state with its state tuple.

### Graph effect

The result describes:

- predecessor references;
- forward-extremity removals;
- whether the candidate becomes a forward extremity;
- whether it is eligible as a local predecessor.

Accepted ordinary events may become forward extremities.

Rejected and soft-failed events are retained as required by Matrix history semantics but are not ordinary local predecessor candidates.

### Visibility class

The semantic result uses a constrained class:

```text
normal
rejected
soft_failed
```

Product-specific visibility and moderation remain outside the engine.

---

## 18. Redaction consequences

Authorization of an `m.room.redaction` event and application of its target effect are separate.

### Event disposition

The redaction event passes through the normal lifecycle and may be accepted, rejected, soft-failed, or dropped.

### Partner evaluation

If the accepted redaction event and its target are both known and valid, apply the room-version-12 target-effect rules.

Return:

```text
consequence.redaction_applied
```

only when the sender has the required redact power level or shares the original sender's domain.

In every other currently unresolved case—including a missing target, a target that is not yet valid, or a pair that does not currently qualify—return:

```text
consequence.redaction_pending_partner
```

Matrix requires the server to wait for a valid partner event and re-check. CyborgHeart therefore does not record a terminal `not_permitted` effect. The redaction event's own disposition remains complete, so the partner is not a `NeedsDependencies` requirement for that original decision.

The future durable runtime records the pending relation and re-evaluates the consequence when relevant partner facts change.

---

## 19. Stage 13 — evidence and work report

Every final or incomplete result includes structured evidence appropriate to the reached stage.

Possible evidence:

- supported room version;
- derived event ID;
- completed lifecycle stages;
- required signers;
- key-validity reference timestamp;
- eligible and checked key IDs;
- content integrity status;
- authorization contexts completed;
- state-before source or resolution summary;
- current-state revision;
- Policy Server state and recommendation result;
- disposition code;
- consequence status;
- work consumed.

Diagnostic messages may be attached, but they are not stable API contracts.

---

## 20. Budget behavior

Budget checks occur before work that would exceed the declared limit.

Example dimensions:

- canonical JSON bytes;
- event facts inspected;
- state entries inspected;
- auth events and auth-chain edges;
- graph edges;
- signature verifications;
- state-resolution steps.

The same inputs and limits must halt at the same point with the same work report.

A larger budget may change:

```text
Halted → Complete
```

It must not change one completed Matrix decision into another.

---

## 21. Dependency restart behavior

`NeedsDependencies` does not serialize an internal coroutine.

The caller:

1. preserves the candidate, room version, options, and frozen current-state revision;
2. extends the immutable fact snapshot;
3. invokes evaluation again from Stage 1;
4. may use the reported stage only as a diagnostic optimization hint.

Correctness must not depend on resuming internal mutable state.

---

## 22. Provenance behavior

Provenance is recorded as:

- federated;
- locally constructed;
- replay;
- import.

It may affect evidence and host behavior.

It must not alter Matrix authorization when all protocol facts are equal.

Transport-level server ACL rejection occurs before the event engine and is not represented as an event disposition.

---

## 23. Lifecycle pseudocode

```rust
fn evaluate_event(input, facts, budget) -> EvaluationResult {
    let rules = admit_room_version(input.room_version)?;
    let candidate = admit_canonical_candidate(input.event, budget)?;

    check_pdu_format(candidate.json(), &rules.event_format)
        .or_drop("drop.invalid_pdu_format")?;

    let event_id = derive_event_id(candidate, &rules, budget)?;
    if let Some(key_request) = discover_required_signing_keys(candidate, facts, &rules)? {
        return NeedsDependencies(DependencyNeed::new(
            key_request,
            evidence_at("signer_discovery"),
        ));
    }

    let verified = verify_event(facts.keys(), candidate.json(), &rules)
        .or_drop_by_verification_cause()?;
    let effective = effective_authorization_event(candidate, verified, &rules)?;

    if let Some(auth_event_request) = preflight_claimed_auth_events(effective, facts, &rules)? {
        return NeedsDependencies(DependencyNeed::new(
            auth_event_request,
            evidence_at("claimed_auth"),
        ));
    }

    check_state_independent_auth_rules(...)
        .or_reject("reject.state_independent_auth")?;

    check_state_dependent_auth_rules(...claimed_auth...)
        .or_reject("reject.claimed_auth")?;

    let state_before = reconstruct_state_before(effective, facts, budget)?;

    check_state_dependent_auth_rules(...state_before...)
        .or_reject("reject.historical_auth")?;

    let current_state = facts.current_state_or_request(...)?;

    check_state_dependent_auth_rules(...current_state...)
        .or_soft_fail("soft_fail.current_state_auth")?;

    validate_policy_recommendation_or_request(...)?;

    let consequences = calculate_semantic_consequences(...)?;
    Complete(Accepted(consequences))
}
```

This is an architectural sketch, not a frozen implementation signature.

---

## 24. Lifecycle invariants

1. Unsupported room versions fail before evaluation.
2. Format and required-signature failures drop.
3. State-independent, claimed-auth, and historical-auth failures reject.
4. Current-state and final Policy Server failures soft-fail.
5. Missing decision facts request dependencies.
6. Missing redaction targets create pending consequences, not incomplete event decisions.
7. Budget exhaustion creates no Matrix disposition.
8. Accepted does not mean committed.
9. Local and federated events share the same normative path.
10. All protocol-influencing inputs are explicit and immutable.

---

## 25. Fixture obligations

Each stage requires positive, negative, dependency, and budget cases where applicable.

Minimum families:

```text
canonical-json/
room-v12/format/
signing/required-signers/
signing/key-validity/
signing/hash-mismatch/
authorization/state-independent/
authorization/claimed-auth/
authorization/historical-state/
soft-failure/current-state/
policy-server/
state-resolution/
redaction/
rejection/
adversarial/
regressions/
```

No stage is complete because only its happy path works.

---

## 26. Final lifecycle commitment

> CyborgHeart applies one candidate room event through an explicit room-version gate, format and cryptographic admission, all required authorization contexts, Policy Server validation, and storage-neutral consequence calculation. Missing facts, bounded halts, and protocol dispositions remain distinct at every stage.

---

## Companion documents

- [`ARCHITECTURE.md`](./ARCHITECTURE.md)
- [`DEPENDENCY-MODEL.md`](./DEPENDENCY-MODEL.md)
- [`DECISION-CODES.md`](./DECISION-CODES.md)
- [`RUMA-COVERAGE.md`](./RUMA-COVERAGE.md)
- [`SPECIFICATION-MAP.md`](./SPECIFICATION-MAP.md)

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

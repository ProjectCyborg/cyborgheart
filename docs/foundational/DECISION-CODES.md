# CyborgHeart Decision Codes

**Status:** Initial machine-readable outcome taxonomy
**Matrix baseline:** v1.19
**Initial room version:** 12 only
**Last reviewed:** 2026-07-12

---

## 1. Purpose

This document defines the stable machine-readable codes emitted by CyborgHeart event application and its immediate capability boundary.

The codes preserve the distinctions between:

- unsupported capability;
- malformed or cryptographically invalid input;
- authorization rejection;
- soft failure;
- missing dependencies;
- bounded halt;
- accepted event;
- deferred semantic consequence.

They do not mirror every Ruma error variant or message.

> Stable codes describe CyborgHeart domain meaning. Upstream messages remain diagnostic detail.

---

## 2. Taxonomy

```text
capability.*
input.*
accepted.*
drop.*
reject.*
soft_fail.*
need.*
halt.*
consequence.*
fault.*
```

### Namespace ownership

| Namespace | Owner | Matrix disposition? |
|---|---|---:|
| `capability.*` | boundary before evaluation | No |
| `input.*` | raw/canonical input adapter | No |
| `accepted.*` | complete event decision | Yes: accepted |
| `drop.*` | complete pre-history event decision | Yes: dropped |
| `reject.*` | complete historical authorization decision | Yes: rejected |
| `soft_fail.*` | complete current-policy decision | Yes: soft-failed |
| `need.*` | incomplete evaluation | No |
| `halt.*` | bounded incomplete evaluation | No |
| `consequence.*` | semantic effect status | No separate disposition |
| `fault.*` | host or implementation contract failure | No |

---

## 3. Code format

Codes are lowercase ASCII strings:

```text
<namespace>.<specific_reason>
```

Examples:

```text
drop.invalid_pdu_format
reject.historical_auth
soft_fail.current_state_auth
need.signing_keys
halt.budget_exceeded
```

Rules:

- namespace and reason use lowercase snake case;
- code meaning is stable once released;
- new codes may be added compatibly;
- renaming or changing meaning is a breaking API change;
- diagnostic messages are not part of the code;
- codes must not contain event IDs, field names, or user data.

---

## 4. Result envelope

Conceptual serialized shape:

```json
{
  "schema_version": 1,
  "kind": "complete",
  "disposition": "rejected",
  "code": "reject.historical_auth",
  "stage": "historical_auth",
  "event_id": "$event",
  "room_version": "12",
  "details": {
    "rule": "authorization",
    "field": null
  },
  "diagnostic": "unstable human-readable detail",
  "evidence": {},
  "work": {}
}
```

This is a semantic example, not a frozen wire format.

### Stable fields

- schema version;
- result kind;
- disposition where applicable;
- decision or request code;
- lifecycle stage;
- supported room version;
- event ID when derivable;
- structured detail keys declared stable by the relevant code;
- work dimension for budget halts.

### Diagnostic fields

- Ruma error text;
- debug messages;
- internal type names;
- stack traces;
- implementation-specific context.

Diagnostic fields may change without an API compatibility promise.

---

## 5. Lifecycle stage names

Stable stage identifiers:

| Stage | Meaning |
|---|---|
| `room_version_admission` | External room version mapped into supported rules |
| `canonical_input` | Candidate canonical representation admitted |
| `pdu_format` | PDU shape and size checks |
| `event_identity` | Reference hash and event ID derivation |
| `signer_discovery` | Required server and key discovery |
| `cryptographic_verification` | Signature and content-hash verification |
| `state_independent_auth` | State-independent authorization |
| `claimed_auth` | Authorization against claimed auth events |
| `state_before` | Historical state reconstruction |
| `historical_auth` | Authorization against state before |
| `current_state_auth` | Authorization against current room state |
| `policy_server` | Policy Server recommendation validation |
| `consequences` | Graph, state, redaction, and visibility effects |

A code must identify the stage where its meaning became final.

---

## 6. Capability codes

These occur before event evaluation.

### `capability.unsupported_room_version`

Meaning:

- the external room-version identifier does not map to a CyborgHeart-supported version.

This code does not cover an `m.room.create` event whose optional `content.room_version` is unrecognized; that is a state-independent authorization rejection. A recognized content value must not be treated as unsupported, or rejected solely because it differs from the externally selected room version.

Disposition:

- none.

Stage:

```text
room_version_admission
```

Stable details:

- requested room-version identifier;
- list or set of supported versions may be included as metadata.

Must not be mapped to `Dropped`.

---

## 7. Input-adapter codes

These occur before a canonical candidate reaches the event engine.

### `input.invalid_json`

Raw input is not valid JSON.

### `input.non_canonical_value`

Input cannot be represented by Matrix canonical JSON constraints.

### `input.not_an_object`

The candidate root is not a JSON object.

These are adapter errors, not Matrix event dispositions. If the engine API accepts only `CanonicalJsonObject`, these codes may exist only in a higher input adapter or testkit.

---

## 8. Accepted code

### `accepted.authorized`

Meaning:

- all required event-decision stages completed successfully;
- the event is accepted under the supported Matrix rules.

Disposition:

```text
accepted
```

Stage:

```text
consequences
```

Accepted does not mean durably committed.

Additional semantic effects are expressed through consequence codes rather than alternative accepted dispositions.

---

## 9. Drop codes

A drop means the candidate failed before normal room-history participation.

### `drop.invalid_pdu_format`

Use when `check_pdu_format()` or equivalent room-version format validation fails.

Possible diagnostics:

- missing required field;
- wrong field type;
- PDU too large;
- identifier too long;
- too many predecessors;
- too many auth events;
- invalid depth;
- invalid room-version-specific field relationship.

Do not create a public code for every current Ruma error string.

### `drop.event_id_mismatch`

Use when an externally asserted event ID does not match the room-version-derived reference-hash event ID.

Stable details:

- expected event ID;
- derived event ID.

Avoid including full event content.

### `drop.invalid_event_identity`

Use for an unrecoverable event-identity construction failure not more precisely covered by `event_id_mismatch`.

Examples:

- invalid reference-hash encoding;
- impossible room-version identity composition.

### `drop.missing_required_signature`

Use when the event itself does not contain a required server signature.

Do not use when the event contains the signature but the local server has not yet acquired its public key. That is `need.signing_keys`.

### `drop.unsupported_required_signature`

Use when all signatures presented for a required entity use unsupported or unusable algorithms or key identifiers.

### `drop.invalid_signature_encoding`

Use when a required signature value cannot be parsed under the Matrix encoding rules.

### `drop.invalid_required_signature`

Use when all required key facts are present and cryptographic verification fails.

### `drop.invalid_integrity_fields`

Use when required event integrity fields such as `hashes` or `signatures` are structurally malformed in a way not already classified by PDU format validation.

### `drop.unrecoverable_canonical_event`

Use when the canonical event cannot be processed consistently after canonical input admission.

This should be rare and fixture-backed.

### Content-hash mismatch is not a drop code

When signatures verify but the content hash differs:

```text
content_integrity = redacted_after_hash_mismatch
```

Evaluation continues with the redacted authorization representation.

---

## 10. Rejection codes

Rejection means the event passed format and required cryptographic admission but failed an authorization stage relevant to its historical position.

### `reject.state_independent_auth`

Use for failure of Ruma state-independent authorization after all required event facts are present.

Examples include:

- invalid create semantics;
- duplicate auth-event state tuple;
- unexpected auth-event tuple;
- rejected auth event;
- auth event from another room;
- room-version-12 create-event relationship failure.

A locally missing auth event must be `need.event`, not this rejection code.

### `reject.claimed_auth`

Use when the event fails state-dependent authorization against its claimed auth-event state.

### `reject.historical_auth`

Use when the event fails state-dependent authorization against reconstructed state before the event.

### Rejection detail policy

Stable top-level meaning comes from the lifecycle call context.

Optional stable details may include:

- `auth_context`: `state_independent`, `claimed`, or `historical`;
- event type;
- state key presence;
- a future stable CyborgHeart rule identifier.

Ruma text remains diagnostic and must not be parsed into logic.

---

## 11. Soft-failure codes

### `soft_fail.current_state_auth`

Use when the event passed historical authorization but fails authorization against the frozen current room state.

Stage:

```text
current_state_auth
```

### `soft_fail.policy_recommendation`

Use when:

- an active Matrix Policy Server applies;
- the event is not exempt;
- the host performed the required recommendation acquisition attempt;
- no valid recommendation was available;
- Matrix requires the event to be soft-failed rather than treated as incomplete.

Stage:

```text
policy_server
```

Do not use this code before the host records final recommendation unavailability. Initial absence is `need.policy_recommendation`.

---

## 12. Dependency request codes

Dependency requests are incomplete evaluations, not decisions.

### `need.event`

Stable details:

- ordered event IDs;
- lifecycle stage that discovered them.

Used for:

- directly referenced auth events;
- room-version-12 create event;
- predecessor events;
- auth-chain traversal;
- conflicted-state-subgraph traversal.

### `need.state_after`

Stable details:

- ordered predecessor event IDs whose state-after maps are required.

### `need.current_room_state`

Stable details:

- room ID;
- expected revision when one has already been established.

### `need.signing_keys`

Stable details per request item:

- candidate event ID;
- server name;
- ordered candidate key IDs;
- event `origin_server_ts`;
- frozen key-validity reference timestamp.

### `need.policy_recommendation`

Stable details:

- candidate event ID;
- active policy state event ID;
- policy fingerprint.

The request means the host should attempt acquisition. It does not prescribe an HTTP operation.

---

## 13. Halt codes

### `halt.budget_exceeded`

Meaning:

- the declared evaluation budget was exhausted;
- no Matrix disposition was invented.

Stable details:

- budget dimension;
- configured limit;
- consumed amount at halt;
- lifecycle stage.

Initial budget dimensions:

```text
canonical_json_bytes
events_inspected
state_entries_inspected
auth_events_inspected
auth_chain_edges
graph_edges
signature_verifications
state_resolution_steps
```

The exact same inputs and budget must halt reproducibly.

No generic timeout code belongs in the pure engine.

---

## 14. Consequence codes

Consequence codes describe semantic effects associated with a completed event disposition.

### Historical state consequences

```text
consequence.state_unchanged
consequence.state_updated
```

These codes describe state at the candidate's historical position. A soft-failed state event may use `consequence.state_updated` while remaining unapplied to current resolved room state at receipt. A rejected state event uses `consequence.state_unchanged`.

Current-room application remains a structured field derived from disposition and graph processing rather than a separate combinatorial code.

### Graph consequences

```text
consequence.forward_extremity_added
consequence.forward_extremity_not_added
```

Graph details should remain structured rather than encoded into many combinatorial code strings.

### Redaction consequences

#### `consequence.redaction_applied`

The accepted redaction event’s target is known and the room-version target-effect rules permit the redaction.

#### `consequence.redaction_pending_partner`

The redaction event's disposition is complete, but no currently valid qualifying partner pair is available.

This covers:

- a missing target event;
- a target or redaction event that is not yet valid for partner processing;
- a currently nonqualifying sender/power relationship.

Matrix requires later re-checking rather than a terminal `not_permitted` result. This code does not produce `NeedsDependencies` for the original event decision.

### Visibility class

Visibility remains a structured enum:

```text
normal
rejected
soft_failed
```

Avoid creating a separate code for every derived visibility boolean.

---

## 15. Fault codes

Faults indicate a violated host or implementation contract. They are not Matrix dispositions and should be rare.

### `fault.inconsistent_facts`

Examples:

- one event ID maps to different canonical events;
- current-state revision maps to different states;
- state fact belongs to another room;
- policy recommendation is bound to another policy fingerprint;
- public-key identity conflicts.

The preferred behavior is to reject the snapshot before evaluation.

### `fault.internal_invariant`

Use when CyborgHeart reaches a state its own preconditions declared impossible.

Examples:

- a Ruma fetch callback returns missing after dependency preflight completed;
- a completed decision lacks a derived event ID where one is mandatory;
- a semantic consequence contradicts the disposition.

Fault codes should not become a catch-all for malformed Matrix input.

---

## 16. Ruma mapping

### PDU format

```text
check_pdu_format() Err
    → drop.invalid_pdu_format
```

Ruma error string:

```text
diagnostic only
```

### Required signer discovery

Malformed event signature structure:

```text
drop.missing_required_signature
or
drop.unsupported_required_signature
or
drop.invalid_signature_encoding
```

Locally absent keys:

```text
need.signing_keys
```

### `verify_event()`

```text
Verified::All
    → continue

Verified::Signatures
    → continue with redacted effective representation

VerificationError after complete key preflight
    → appropriate drop.* code
```

`NoPublicKeysForEntity` after preflight is normally `fault.internal_invariant`, not proof that the event is invalid.

### Authorization

```text
check_state_independent_auth_rules() Err
    → reject.state_independent_auth

check_state_dependent_auth_rules() Err against claimed auth
    → reject.claimed_auth

check_state_dependent_auth_rules() Err against state before
    → reject.historical_auth

check_state_dependent_auth_rules() Err against current state
    → soft_fail.current_state_auth
```

The same Ruma function maps differently because the Matrix lifecycle context is different.

### State resolution

Missing concrete facts discovered before `resolve()`:

```text
need.event
need.state_after
```

Budget exhaustion:

```text
halt.budget_exceeded
```

Unexpected fetch failure after preflight:

```text
fault.internal_invariant
```

---

## 17. Code-to-specification map

| Code | Primary specification-map IDs |
|---|---|
| `drop.invalid_pdu_format` | `MX-PDU-001` through `MX-PDU-006`, `MX-DISP-001` |
| `drop.event_id_mismatch` | `MX-ID-001` |
| `drop.missing_required_signature` | `MX-SIG-002`, `MX-SIG-003`, `MX-SIG-007` |
| `drop.invalid_required_signature` | `MX-SIG-002`, `MX-SIG-007` |
| `drop.invalid_integrity_fields` | `MX-SIG-002`, `MX-SIG-005` through `MX-SIG-008` |
| `reject.state_independent_auth` | `MX-AUTH-002`, `MX-AUTH-006` |
| `reject.claimed_auth` | `MX-AUTH-003` |
| `reject.historical_auth` | `MX-AUTH-004` |
| `soft_fail.current_state_auth` | `MX-AUTH-005`, `MX-DISP-004`, `MX-DISP-005` |
| `soft_fail.policy_recommendation` | `MX-PS-001` through `MX-PS-006` |
| `need.event` | `MX-AUTH-001` through `MX-AUTH-004`, `MX-STATE-003` through `MX-STATE-007` |
| `need.state_after` | `MX-STATE-003`, `MX-STATE-004` |
| `need.signing_keys` | `MX-SIG-002` through `MX-SIG-004` |
| `need.policy_recommendation` | `MX-PS-003` through `MX-PS-006` |
| `consequence.redaction_*` | `MX-RED-002` through `MX-RED-005` |
| `halt.budget_exceeded` | `CH-ARCH-004`, `CH-ARCH-005` |

The table is traceability, not a substitute for fixtures.

---

## 18. Fixture obligations

Every stable decision code must have at least one direct fixture before public release.

Each negative code should normally have:

- minimal positive control;
- direct negative case;
- malformed boundary case;
- deterministic replay;
- evidence assertion;
- work-ceiling assertion.

Suggested paths:

```text
fixtures/matrix-v1.19/room-v12/format/
fixtures/matrix-v1.19/signing/
fixtures/matrix-v1.19/room-v12/authorization/
fixtures/matrix-v1.19/room-v12/rejection/
fixtures/matrix-v1.19/room-v12/soft-failure/
fixtures/matrix-v1.19/room-v12/policy-server/
fixtures/matrix-v1.19/room-v12/redaction/
fixtures/adversarial/
fixtures/regressions/
```

A code without a fixture remains internal and unsupported.

---

## 19. Rust representation

Conceptual enums and result envelopes:

```rust
#[non_exhaustive]
enum CapabilityCode {
    UnsupportedRoomVersion,
}

#[non_exhaustive]
enum InputCode {
    InvalidJson,
    NonCanonicalValue,
    NotAnObject,
}

#[non_exhaustive]
enum AcceptedCode {
    Authorized,
}

#[non_exhaustive]
enum DropCode {
    InvalidPduFormat,
    EventIdMismatch,
    InvalidEventIdentity,
    MissingRequiredSignature,
    UnsupportedRequiredSignature,
    InvalidSignatureEncoding,
    InvalidRequiredSignature,
    InvalidIntegrityFields,
    UnrecoverableCanonicalEvent,
}

#[non_exhaustive]
enum RejectionCode {
    StateIndependentAuth,
    ClaimedAuth,
    HistoricalAuth,
}

#[non_exhaustive]
enum SoftFailureCode {
    CurrentStateAuth,
    PolicyRecommendation,
}

#[non_exhaustive]
enum DependencyCode {
    Event,
    StateAfter,
    CurrentRoomState,
    SigningKeys,
    PolicyRecommendation,
}

#[non_exhaustive]
enum HaltCode {
    BudgetExceeded,
}

#[non_exhaustive]
enum ConsequenceCode {
    StateUnchanged,
    StateUpdated,
    ForwardExtremityAdded,
    ForwardExtremityNotAdded,
    RedactionApplied,
    RedactionPendingPartner,
}

#[non_exhaustive]
enum FaultCode {
    InconsistentFacts,
    InternalInvariant,
}

#[non_exhaustive]
enum DecisionCode {
    Capability(CapabilityCode),
    Input(InputCode),
    Accepted(AcceptedCode),
    Drop(DropCode),
    Rejection(RejectionCode),
    SoftFailure(SoftFailureCode),
    Dependency(DependencyCode),
    Halt(HaltCode),
    Consequence(ConsequenceCode),
    Fault(FaultCode),
}

#[non_exhaustive]
enum AdmittedOutcome {
    Accepted(AcceptedCode),
    Rejected(RejectionCode),
    SoftFailed(SoftFailureCode),
}

struct AdmittedDecision {
    outcome: AdmittedOutcome,
    evidence: DecisionEvidence,
    consequences: SemanticConsequences,
}
```

Serialized string values remain the dotted codes defined in this document.

Use `#[non_exhaustive]` so new codes and admitted outcomes can be added without implying exhaustive downstream matching. Code families maintain `ALL` or `all()` registries, and families with a stable owning stage expose that stage directly rather than accepting it from callers.

---

## 20. Versioning policy

### Compatible changes

- adding a new code;
- adding an optional diagnostic field;
- adding a new optional evidence field;
- refining human-readable messages.

### Breaking changes

- renaming a code;
- changing its lifecycle stage;
- changing its disposition;
- broadening or narrowing its meaning materially;
- changing a stable detail field’s semantics;
- reusing a retired code for another purpose.

### Retirement

A retired code remains documented for the relevant major version and is never reassigned.

### Schema version

If a serialized envelope is published, it carries an explicit schema version independent of the Matrix room version.

---

## 21. Terms deliberately avoided

Do not emit these as stable codes:

```text
invalid_event
auth_failed
processing_error
unknown_error
temporary_failure
bad_state
not_allowed
failed
```

They erase lifecycle meaning and produce unstable downstream behavior.

Use the narrow code that names the owning stage and outcome.

---

## 22. Decision evidence requirements

Each complete event decision should carry:

- code;
- disposition;
- lifecycle stage;
- event ID when derivable;
- supported room version;
- completed-stage set;
- cryptographic integrity status;
- key-validity reference timestamp and eligible key IDs where relevant;
- authorization contexts completed;
- relevant current-state revision;
- work report.

Additional evidence by code:

| Code family | Additional evidence |
|---|---|
| `drop.*signature*` | required server, candidate key IDs, verified key IDs where safe |
| `reject.*auth` | authorization context and candidate event type |
| `soft_fail.current_state_auth` | frozen current-state revision |
| `soft_fail.policy_recommendation` | policy event ID and policy fingerprint |
| `halt.budget_exceeded` | dimension, limit, consumed amount |
| `consequence.redaction_*` | redaction event ID, target ID when known, and partner status |

Evidence must avoid secrets and unnecessary event content.

---

## 23. Final decision-code commitment

> CyborgHeart codes describe stable lifecycle meaning rather than upstream implementation detail. A caller can distinguish unsupported capability, incomplete facts, bounded halt, dropped input, historical rejection, current-policy soft failure, acceptance, and deferred consequence without parsing logs or Ruma error strings.

---

## Companion documents

- [`EVENT-LIFECYCLE.md`](./EVENT-LIFECYCLE.md)
- [`DEPENDENCY-MODEL.md`](./DEPENDENCY-MODEL.md)
- [`SPECIFICATION-MAP.md`](./SPECIFICATION-MAP.md)
- [`SUPPORTED-SURFACE.md`](./SUPPORTED-SURFACE.md)
- [`WORKING-LEXICON.md`](./WORKING-LEXICON.md)

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

# CyborgHeart Supported Surface

**Status:** Living compatibility contract
**Protocol baseline:** Matrix v1.19
**Initial target room version:** 12 only
**Primary dependency:** Ruma
**Last reviewed:** 2026-07-12

---

## 1. Purpose

This document defines exactly what CyborgHeart currently supports, what it is actively targeting, and what remains outside scope.

It exists to prevent accidental compatibility claims.

> A protocol feature is supported only when the implementation, documentation, specification mapping, Ruma coverage, conformance fixtures, adversarial tests, and public API all agree.

The existence of a Ruma type, enum variant, or algorithm does not by itself make the corresponding Matrix behavior supported by CyborgHeart.

---

## 2. Support states

Every item in this document uses one of the following states.

| State | Meaning |
|---|---|
| **Unsupported** | Not implemented and not claimed. |
| **Targeted** | Included in the current implementation plan but not yet supported. |
| **Partial** | Some required behavior exists, but the full claim is not proven. |
| **Supported** | Deliberately implemented and proven by the required evidence. |
| **Deferred** | Intentionally excluded from the current phase. |
| **Out of scope** | Not part of the relevant CyborgHeart layer. |

No item may be marked **Supported** merely because happy-path tests pass.

---

## 3. Current compatibility claim

At repository initialization:

```text
Supported Matrix room versions: none
Supported event-application lifecycle: none
Supported homeserver APIs: none
Supported federation behavior: none
```

CyborgHeart is currently establishing its first supported surface.

The initial target is:

> Deterministic Matrix v1.19 room-event application for room version 12, using canonical JSON and explicit immutable dependencies, with bounded work and complete outcome classification.

Until the promotion gates in this document are satisfied, this remains a target rather than a support claim.

---

## 4. Initial target surface

### 4.1 Protocol baseline

| Surface | Status | Target |
|---|---:|---|
| Matrix specification v1.19 | Targeted | Normative baseline for initial implementation |
| Matrix room version 12 | Targeted | Only admitted room version |
| Earlier room versions | Deferred | No compatibility claim |
| Future room versions | Deferred | Added only through explicit review |
| Unstable MSC behavior | Unsupported | Not enabled in the initial surface |

### 4.2 Input model

| Surface | Status | Target |
|---|---:|---|
| One candidate room event per evaluation | Targeted | Primary engine unit |
| Canonical JSON event input | Targeted | Required untrusted representation |
| Lazy PDU field inspection | Targeted | Avoid strict full-event typing for untrusted PDUs |
| Typed local event content | Deferred | Belongs primarily to the future command runtime |
| Raw HTTP request input | Out of scope | Transport adapter concern |
| Raw federation transaction input | Out of scope | Federation ingress concern |
| Batch event application | Deferred | May be built above single-event evaluation |

The event engine accepts a canonical candidate event and does not perform network or storage I/O.

### 4.3 Provenance

The engine may record how a candidate entered the system:

| Provenance | Status | Meaning |
|---|---:|---|
| Federated | Targeted | Received from a remote Matrix server |
| Locally constructed | Targeted | Created by a future command runtime |
| Replay | Targeted | Re-evaluated from retained data |
| Import | Targeted | Supplied from an import or migration path |

Provenance may affect evidence or permitted entry points. It must not silently change Matrix authorization semantics.

---

## 5. Room-version-12 lifecycle

The initial target lifecycle is:

```text
supported room-version admission
        ↓
canonical candidate event
        ↓
PDU format check
        ↓
required signer discovery
        ↓
event ID, hash, and signature processing
        ↓
redacted authorization representation when required
        ↓
state-independent authorization
        ↓
claimed auth-event resolution
        ↓
authorization against claimed auth events
        ↓
state-before reconstruction
        ↓
authorization against historical state
        ↓
authorization against current room state
        ↓
Policy Server recommendation validation when enabled
        ↓
accepted / rejected / soft-failed
        ↓
semantic graph, state, redaction, and visibility consequences
```

### 5.1 Lifecycle support matrix

| Lifecycle responsibility | Status |
|---|---:|
| Supported-room-version admission | Targeted |
| PDU format validation, including CyborgHeart preflight for fields not covered by released Ruma helper | Targeted |
| Create-event `content.room_version` recognition without a non-normative equality check | Targeted |
| Event ID and room/create identity handling | Targeted |
| Required signer discovery | Targeted |
| Restricted-join authorizing-server signature requirement | Targeted |
| Signing-key event-time and effective-validity checks | Targeted |
| Reference hashing | Targeted |
| Content hashing | Targeted |
| Signature verification | Targeted |
| Valid-signature/content-hash-mismatch distinction | Targeted |
| Room-version-appropriate redacted verification representation | Targeted |
| State-independent authorization | Targeted |
| Claimed auth-event dependency preflight | Targeted |
| Authorization against claimed auth events | Targeted |
| State-before reconstruction | Targeted |
| Authorization against historical state | Targeted |
| Authorization against frozen current room state | Targeted |
| Detect Policy Server enablement from current room state | Targeted |
| Validate supplied Policy Server recommendation material | Targeted |
| Request host acquisition when a recommendation is missing or invalid | Targeted |
| Soft-fail after the required acquisition attempt remains unsuccessful | Targeted |
| Linear room history | Targeted |
| Branching room history | Targeted |
| Room-version-12 state resolution | Targeted |
| Semantic graph effects | Targeted |
| Historical state effects, including soft-failed state events | Targeted |
| Redaction effect: applied or pending partner | Targeted |
| Deterministic replay | Targeted |
| Explicit work accounting | Targeted |

No lifecycle item becomes supported independently if an incomplete lifecycle could produce a misleading final decision.

## 6. Event outcomes

The initial engine result surface is:

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

| Outcome | Status | Meaning |
|---|---:|---|
| `Dropped` | Targeted | Failed before normal room-history participation |
| `Accepted` | Targeted | Passed required Matrix authorization |
| `Rejected` | Targeted | Admitted but failed state-independent, claimed-auth, or historical-state authorization |
| `SoftFailed` | Targeted | Historically valid but failed against current state, or lacked a valid required Policy Server recommendation after the host attempt |
| `NeedsDependencies` | Targeted | No final decision possible with current facts |
| `BudgetExceeded` | Targeted | Evaluation halted without a protocol decision |
| `Duplicate` | Out of scope | Future transaction-runtime result |
| `Committed` | Out of scope | Future durable-runtime result |

Important distinctions:

- accepted does not mean committed;
- incomplete does not mean rejected;
- budget exhaustion does not mean invalid;
- duplicate is not an event-engine disposition;
- an unsupported room version is a capability-boundary error, not a dropped event;
- dropped events do not produce ordinary room graph or state effects.

---

## 7. Dependency surface

The engine may request immutable facts required to continue evaluation.

### 7.1 Initial dependency categories

| Dependency | Status |
|---|---:|
| Event by event ID | Targeted |
| State before an event | Targeted |
| State after an event | Targeted |
| Current room state | Targeted |
| Server signing keys | Targeted |
| Policy Server recommendation fact | Targeted |
| Opaque persisted continuation | Unsupported |
| Database query instruction | Out of scope |
| Federation fetch instruction | Out of scope |

A dependency request describes **what fact is missing**, not **how to obtain it**.

The host may later satisfy dependencies from memory, storage, cache, federation, imports, or fixtures.

### 7.2 Resumability

Initial resumability is:

```text
evaluate with snapshot A
        ↓
NeedsDependencies
        ↓
construct enriched snapshot B
        ↓
evaluate again from the beginning
```

Persisted internal continuations are not supported.

### 7.3 Dependency monotonicity

The target contract requires:

```text
NeedsDependencies → Complete
```

when valid missing facts are added.

It forbids:

```text
Complete(Accepted) → Complete(Rejected)
```

after adding valid facts.

This property must be covered by dedicated tests before support is claimed.

---

## 8. Semantic consequence surface

For an admitted event, the engine targets storage-neutral consequences.

### 8.1 Graph consequences

| Surface | Status |
|---|---:|
| Referenced predecessors | Targeted |
| Forward-extremity removal | Targeted |
| Forward-extremity addition | Targeted |
| Database graph schema | Out of scope |
| Federation delivery scheduling | Out of scope |

### 8.2 State consequences

| Surface | Status |
|---|---:|
| Historical state before | Targeted |
| Historical state after | Targeted |
| Message-event state unchanged | Targeted |
| Accepted state-event tuple update | Targeted |
| Soft-failed state-event historical tuple update | Targeted |
| Rejected state-event state unchanged | Targeted |
| State delta and state reference | Targeted |
| Immediate current-room application as distinct from historical state | Targeted |
| Database state encoding | Out of scope |
| Snapshot compaction policy | Out of scope |

A soft-failed state event can update historical state after while remaining excluded from immediate current forward extremities and current-room application.

### 8.3 Deferred redaction consequences

| Surface | Status |
|---|---:|
| Apply a redaction when both valid partner events are known and the additional checks pass | Targeted |
| Return `pending_partner` when no currently valid qualifying partner is available | Targeted |
| Re-evaluate a pending redaction when a valid partner later arrives | Targeted |
| Block the original event disposition solely because the redaction partner is unavailable | Unsupported |
| Treat a currently nonqualifying pair as a permanent `not_permitted` result | Unsupported |

A pending redaction consequence is not `NeedsDependencies`: the event decision is complete, while the related effect and client visibility wait for a valid partner event.

### 8.4 Evidence

| Evidence | Status |
|---|---:|
| Completed lifecycle stages | Targeted |
| Signers and key IDs checked | Targeted |
| Content-integrity status | Targeted |
| Authorization contexts | Targeted |
| State-resolution evidence | Targeted |
| Work report | Targeted |
| Free-form logs as correctness evidence | Unsupported |

Evidence must be structured enough to explain and reproduce a result. Logs remain supplementary.

---

## 9. Evaluation-budget surface

The initial target includes explicit accounting for:

| Budget dimension | Status |
|---|---:|
| Canonical JSON bytes | Targeted |
| Events inspected | Targeted |
| State entries inspected | Targeted |
| Auth events inspected | Targeted |
| Auth-chain edges | Targeted |
| Graph edges | Targeted |
| Signature verifications | Targeted |
| State-resolution steps | Targeted |
| Wall-clock timeout as protocol result | Unsupported |

A budget determines whether evaluation completes. It must not alter the completed Matrix decision.

Budget exhaustion must be deterministic and reproducible for the same inputs and limits.

---

## 10. Determinism surface

The supported contract will require identical observable results for identical:

- canonical event;
- supported room version;
- dependency snapshot;
- frozen key-validity reference timestamp;
- evaluation options;
- evaluation budget.

Observable outputs include:

- dependency requests;
- final decisions;
- graph consequences;
- state consequences;
- evidence;
- work reports.

The implementation must not allow these to depend on:

- wall-clock time;
- randomness;
- process identity;
- hidden caches;
- network state;
- database state not present in the dependency snapshot;
- unordered map iteration.

Stable ordering or explicit normalization is required for returned collections.

---

## 11. Ruma usage surface

CyborgHeart targets Ruma-provided behavior for:

| Ruma responsibility | Status |
|---|---:|
| Matrix identifiers | Targeted |
| Canonical JSON | Targeted |
| Room-version rules | Targeted |
| Event ID and reference-hash behavior | Targeted |
| Content hashes | Targeted |
| Signing | Targeted |
| Signature verification | Targeted |
| Event authorization | Targeted |
| `ruma-events` public event-type interoperability without strict untrusted-PDU validation | Targeted |
| State resolution | Targeted |
| Complete Policy Server lifecycle | CyborgHeart gap; not assumed from Ruma |
| Client–Server API models | Deferred |
| Server–Server API models | Deferred |
| Unstable MSC APIs | Unsupported |

CyborgHeart does not claim support for every capability exposed by Ruma.

Any local replacement for Ruma-owned behavior requires:

- a reproduced defect;
- a specification-backed expected result;
- a regression fixture;
- an ADR;
- an isolated and removable workaround.

---

## 12. Conformance surface

Support requires evidence at multiple levels.

### 12.1 Required test categories

| Category | Required for support |
|---|---:|
| Unit tests for local invariants | Yes |
| Specification-traceable conformance fixtures | Yes |
| Deterministic replay tests | Yes |
| Dependency-monotonicity tests | Yes |
| Input-order-independence tests where applicable | Yes |
| Malformed-input tests | Yes |
| Adversarial graph and budget tests | Yes |
| Regression fixtures for discovered defects | Yes |
| Fuzzing of untrusted parsing boundaries | Yes |
| End-to-end homeserver tests | Not for the event engine |
| Matrix Complement | Later homeserver surface |

### 12.2 Fixture requirements

Each conformance fixture must identify:

- fixture ID;
- Matrix specification version;
- room version;
- normative section;
- provenance and license;
- candidate event;
- available dependencies;
- expected dependency requests;
- expected outcome;
- expected graph and state consequences;
- expected evidence;
- work ceiling.

A feature without this evidence cannot be marked supported.

---

## 13. Explicitly unsupported early surface

The following are not part of the initial event-application support claim.

### 13.1 Protocol versions

- room versions other than 12;
- unstable Matrix room versions;
- MSC-only behavior;
- automatic fallback between room versions.

### 13.2 Network and API behavior

- HTTP;
- Client–Server API endpoints;
- Server–Server API endpoints;
- federation transactions;
- federation retries;
- calling `POST /_matrix/policy/v1/sign` from the event engine;
- serving as a Policy Server;
- media APIs;
- push gateway APIs;
- application service APIs;
- identity service APIs.

### 13.3 Durable execution

- database persistence;
- duplicate detection;
- idempotency;
- retries;
- atomic commits;
- crash recovery;
- durable outbox;
- storage migrations.

### 13.4 User and device behavior

- accounts;
- login;
- access tokens;
- OIDC;
- devices;
- one-time keys;
- cross-signing;
- key backup;
- to-device messaging;
- Olm or Megolm implementation.

### 13.5 Client behavior

- `/sync`;
- room timelines for clients;
- local echo;
- client-side room models;
- E2EE decryption;
- calling;
- push action presentation.

These belong to client SDKs or future server capabilities, not the initial event engine.

### 13.6 Product and operations

- product moderation;
- operator administration;
- spam policy;
- server allowlists or blocklists;
- media quarantine;
- telemetry;
- deployment automation;
- multi-process orchestration;
- dynamic plugins.

---

## 14. Promotion to supported

An item may move from **Targeted** or **Partial** to **Supported** only when all applicable conditions are met.

### Required promotion gate

1. The public API admits the behavior deliberately.
2. The relevant Matrix rule is cited in `SPECIFICATION-MAP.md`.
3. The relied-on Ruma behavior is recorded in `RUMA-COVERAGE.md`.
4. The implementation is complete for the claimed scope.
5. Positive conformance fixtures pass.
6. Negative and malformed-input fixtures pass.
7. Adversarial and budget cases pass where relevant.
8. Deterministic replay passes.
9. Dependency monotonicity passes where relevant.
10. Fuzz regressions are retained.
11. `WORKING-LEXICON.md`, `ARCHITECTURE.md`, and this document use consistent terminology.
12. No known failing normative case remains inside the claimed scope.

A support claim should be narrow enough to be true without qualifications hidden elsewhere.

---

## 15. Partial support policy

Use **Partial** only when a clear sub-surface can be described precisely.

Example:

```text
Partial:
PDU format and cryptographic verification for room version 12

Not yet supported:
authorization, state reconstruction, state resolution,
or final event disposition
```

Do not use **Partial** to imply that the full event lifecycle is usable.

A partial implementation must not return a final `Accepted`, `Rejected`, or `SoftFailed` decision unless all required lifecycle stages are complete.

---

## 16. Versioning and updates

This document changes whenever the supported surface changes.

A support change must update, in the same pull request:

- implementation;
- conformance fixtures;
- `SUPPORTED-SURFACE.md`;
- `SPECIFICATION-MAP.md`;
- `RUMA-COVERAGE.md`;
- decision codes where applicable;
- an ADR when layer ownership or compatibility policy changes.

Dependency updates do not automatically change the supported surface.

A newer Matrix specification or Ruma release must be reviewed before the baseline in this document changes.

---

## 17. Initial support milestones

The first support claim should be promoted in stages.

### Milestone A — verified candidate intake

Potential first partial claim:

- room version 12 support gate;
- canonical PDU inspection;
- PDU format checking;
- signer discovery;
- signing-key dependency requests;
- event ID, reference hash, content hash, and signature verification;
- redacted authorization representation after content-hash mismatch;
- deterministic work reporting.

This milestone does not support final room-event application.

### Milestone B — linear room-event application

Potential first complete but deliberately narrow claim:

- room version 12;
- linear room history;
- create, creator membership, power levels, invite, join, and message sequence;
- all required authorization contexts;
- accepted, rejected, soft-failed, and dropped outcomes;
- historical state and graph consequences;
- explicit dependencies and bounded work;
- rooms without an active Matrix Policy Server;
- no claim of complete redaction-partner handling.

This milestone proves the core lifecycle without implying the complete v1.19 target.

### Milestone C — complete initial target

The first full initial claim requires:

- branching histories and predecessor-state reconstruction;
- room-version-12 state resolution and input-order independence;
- adversarial graph budgets;
- event-time and effective signing-key validity;
- restricted-join authorizing-server signatures;
- complete Policy Server recommendation handling;
- applied and pending-partner redaction consequences with later re-evaluation;
- soft-failed historical state-after semantics;
- deterministic restart after dependency requests;
- complete specification and Ruma traceability.

Only Milestone C completes the initial target surface defined by this document.

## 18. Current status table

At document creation:

| Surface | Current state |
|---|---:|
| Repository and workspace baseline | Targeted |
| Event-application public contract | Targeted |
| Canonical PDU intake | Targeted |
| Cryptographic verification | Targeted |
| Authorization lifecycle | Targeted |
| State reconstruction | Targeted |
| State resolution | Targeted |
| Semantic consequences | Targeted |
| Deterministic replay | Targeted |
| Evaluation budgets | Targeted |
| Conformance corpus | Targeted |
| Room version 12 support claim | Unsupported |
| Complete Matrix event-application claim | Unsupported |
| Homeserver claim | Unsupported |

This table must be updated as implementation evidence is merged.

---

## 19. Final support commitment

> CyborgHeart will claim only the Matrix behavior it has deliberately implemented and proven. Its initial target is deterministic, bounded, storage-neutral room-event application for Matrix v1.19 room version 12. Until the full evidence gate is satisfied, that target must not be presented as supported compatibility.

---

## Companion documents

- [`MANIFESTO.md`](./MANIFESTO.md) — enduring principles.
- [`WORKING-THESIS.md`](./WORKING-THESIS.md) — working strategic and falsifiable argument.
- [`WORKING-LEXICON.md`](./WORKING-LEXICON.md) — working terminology.
- [`ARCHITECTURE.md`](./ARCHITECTURE.md) — system structure and boundaries.
- [`SPECIFICATION-MAP.md`](./SPECIFICATION-MAP.md) — Matrix obligations mapped to implementation and fixtures.
- [`RUMA-COVERAGE.md`](./RUMA-COVERAGE.md) — Ruma behavior relied upon and known gaps.
- [`EVENT-LIFECYCLE.md`](./EVENT-LIFECYCLE.md) — exact stage order and disposition points.
- [`DEPENDENCY-MODEL.md`](./DEPENDENCY-MODEL.md) — immutable facts and resumability.
- [`DECISION-CODES.md`](./DECISION-CODES.md) — stable machine-readable outcome taxonomy.

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

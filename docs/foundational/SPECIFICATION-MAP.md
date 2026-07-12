# CyborgHeart Specification Map

**Status:** Initial traceability baseline
**Matrix baseline:** v1.19
**Initial target room version:** 12 only
**Candidate Ruma baseline:** `ruma 0.16.0`, `ruma-events 0.34.0`, `ruma-state-res 0.17.0`, `ruma-signatures 0.21.0`
**Last reviewed:** 2026-07-12

---

## 1. Purpose

This document maps CyborgHeart’s initial event-application surface from:

```text
Matrix obligation
        ↓
Ruma capability or known gap
        ↓
CyborgHeart owning module
        ↓
conformance fixture family
        ↓
support status
```

It is the traceability ledger behind `SUPPORTED-SURFACE.md`.

This document does not claim that any mapped item is implemented. At repository initialization, all initial-surface items are targets or identified gaps.

---

## 2. Authority and interpretation

The order of authority is:

1. Matrix v1.19 specification.
2. Applicable room-version-12 rules.
3. Verified Ruma behavior.
4. CyborgHeart architecture and public contract.
5. Implementation and fixtures.

When sources appear to disagree:

- Matrix determines intended protocol behavior;
- Ruma behavior is reproduced and inspected;
- CyborgHeart records the mismatch;
- no compatibility claim is made until the mismatch is resolved or isolated.

Matrix requirement words such as **MUST**, **SHOULD**, and **MAY** retain their RFC 2119 meanings.

---

## 3. Mapping status

| Status | Meaning |
|---|---|
| **Targeted** | Required by the initial implementation plan; not yet supported. |
| **Gap** | Required behavior whose Ruma or CyborgHeart mapping needs explicit implementation or verification. |
| **Deferred** | Deliberately excluded from the initial event-engine phase. |
| **Outside engine** | Required elsewhere in a complete homeserver, but not owned by event application. |
| **Supported** | Implemented and proven under the support gate. |

A row may move to **Supported** only when its code, fixtures, Ruma mapping, and documentation are complete.

---

## 4. Baseline assessment

Matrix v1.19 was released on **2026-07-08**. Its changes relevant to the initial CyborgHeart surface are primarily clarifications:

- handling multiple signatures during verification;
- preserving origin signatures when a Policy Server shares the origin name;
- canonical JSON grammar clarification;
- explicit URL-safe unpadded Base64 specification.

Room version 12 itself is carried forward from the prior specification and remains the initial room-version target.

The candidate Ruma baseline documents broad Matrix v1.18 coverage rather than a blanket v1.19 claim. CyborgHeart must audit the v1.19 delta for every relied-on behavior. The versions resolved into the first reviewed `Cargo.lock` become the operational baseline.

---

## 5. Primary traceability map

### 5.1 Event model, canonical JSON, and identifiers

| ID | Matrix obligation | Matrix source | Ruma mapping | CyborgHeart owner | Fixture family | Status |
|---|---|---|---|---|---|---:|
| `MX-EVT-001` | Treat event bodies as untrusted and validate fields before use. | [Events](https://spec.matrix.org/v1.19/#events) | Canonical JSON plus lazy `ruma-state-res` event helpers | `input.rs`, `internal/pdu.rs` | `room-v12/format/` | Targeted |
| `MX-EVT-002` | Model room history as a DAG linked by predecessor events. | [Event Graphs](https://spec.matrix.org/v1.19/#event-graphs) | Ruma IDs and state-resolution event abstraction | `internal/state.rs`, `consequences.rs` | `linear-history/`, `state-resolution/` | Targeted |
| `MX-JSON-001` | Strictly enforce Matrix canonical JSON. | [Canonical JSON](https://spec.matrix.org/v1.19/appendices/#canonical-json), [RV12 Canonical JSON](https://spec.matrix.org/v1.19/rooms/v12/#canonical-json) | Ruma canonical JSON types and utilities | `input.rs`, `internal/pdu.rs` | `canonical-json/` | Targeted |
| `MX-JSON-002` | Use unpadded and URL-safe unpadded Base64 where specified. | [Base64](https://spec.matrix.org/v1.19/appendices/#unpadded-base64), [URL-safe Base64](https://spec.matrix.org/v1.19/appendices/#url-safe-unpadded-base64) | Ruma Base64 wrappers and signature utilities | `internal/verification.rs` | `signing/base64/` | Targeted |
| `MX-ID-001` | Derive room-version-12 event IDs from the reference hash with `$` prefix. | [RV12 Event IDs](https://spec.matrix.org/v1.19/rooms/v12/#event-ids) | Ruma signature/reference-hash support | `internal/verification.rs` | `room-v12/event-ids/` | Targeted |
| `MX-ID-002` | Derive a room-version-12 room ID from the create event’s reference hash; omit `room_id` from the create event. | [RV12 Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format) | Ruma room-version rules and identifier types; integration path must be verified | `room_version.rs`, `internal/pdu.rs` | `room-v12/create/` | Gap |

### 5.2 Room-version-12 PDU format

| ID | Matrix obligation | Matrix source | Ruma mapping | CyborgHeart owner | Fixture family | Status |
|---|---|---|---|---|---|---:|
| `MX-PDU-001` | Enforce required PDU fields and their room-version-specific types. | [RV12 Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format) | `check_pdu_format()` | `internal/pdu.rs` | `room-v12/format/required-fields/` | Targeted |
| `MX-PDU-002` | Limit `auth_events` to at most 10 event IDs. | [RV12 Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format) | `check_pdu_format()`; verify exact coverage | `internal/pdu.rs` | `room-v12/format/auth-events/` | Targeted |
| `MX-PDU-003` | Limit `prev_events` to at most 20 event IDs. | [RV12 Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format) | `check_pdu_format()`; verify exact coverage | `internal/pdu.rs` | `room-v12/format/prev-events/` | Targeted |
| `MX-PDU-004` | Validate `depth` as a non-negative canonical integer within Matrix’s integer range. | [RV12 Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format) | Canonical integer representation plus `check_pdu_format()` | `internal/pdu.rs` | `room-v12/format/depth/` | Targeted |
| `MX-PDU-005` | Interpret `state_key` presence as a state event and identify its state tuple. | [RV12 Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format) | Ruma event abstraction | `internal/pdu.rs`, `internal/state.rs` | `room-v12/format/state-key/` | Targeted |
| `MX-PDU-006` | Drop a PDU that does not conform to its room-version format. | [PDU receipt checks](https://spec.matrix.org/v1.19/server-server-api/#checks-performed-on-receipt-of-a-pdu) | `check_pdu_format()` plus CyborgHeart structural preflight | `evaluation.rs`, `decision.rs` | `room-v12/format/` | Targeted |
| `MX-PDU-007` | Require structurally valid `content`, `origin_server_ts`, `hashes`, and `signatures` fields even though the released `check_pdu_format()` helper does not validate all of them. | [RV12 Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format) | Ruma canonical field access and `verify_event()` supplemented by CyborgHeart preflight | `internal/pdu.rs`, `internal/verification.rs` | `room-v12/format/required-fields/` | Gap |

### 5.3 Hashes, signatures, and redacted verification

| ID | Matrix obligation | Matrix source | Ruma mapping | CyborgHeart owner | Fixture family | Status |
|---|---|---|---|---|---|---:|
| `MX-SIG-001` | Sign canonical JSON without `signatures` and `unsigned`. | [Signing JSON](https://spec.matrix.org/v1.19/appendices/#signing-json) | Ruma signing utilities | Future command runtime; test vectors live in testkit | `signing/json/` | Deferred |
| `MX-SIG-002` | Validate required event signatures before content-hash processing. | [Validating received events](https://spec.matrix.org/v1.19/server-server-api/#validating-hashes-and-signatures-on-received-events) | `ruma_signatures::verify_event()` | `internal/verification.rs` | `signing/signatures/` | Targeted |
| `MX-SIG-003` | Require the sender’s server signature for room version 12, subject to specified exceptions. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules), [Received signatures](https://spec.matrix.org/v1.19/server-server-api/#validating-hashes-and-signatures-on-received-events) | Ruma required-signer and verification behavior; exact public API path to lock during implementation | `internal/verification.rs`, `dependencies.rs` | `signing/required-signers/` | Gap |
| `MX-SIG-004` | Require `valid_until_ts >= origin_server_ts` and use the lesser of `valid_until_ts` and seven days after the explicit validity-reference time when deciding key eligibility. | [RV12 Signing key validity](https://spec.matrix.org/v1.19/rooms/v12/#signing-key-validity-period) | Ruma verifies signatures; CyborgHeart validates caller-supplied key metadata and explicit reference time | `dependencies.rs`, `internal/verification.rs` | `signing/key-validity/` | Gap |
| `MX-SIG-005` | Calculate the content hash over the complete unredacted event after removing `unsigned`, `signatures`, and `hashes`. | [Content hash](https://spec.matrix.org/v1.19/server-server-api/#calculating-the-content-hash-for-an-event) | Ruma signature/hash utilities | `internal/verification.rs` | `signing/content-hash/` | Targeted |
| `MX-SIG-006` | Calculate the reference hash over the redacted event without `signatures` and `unsigned`. | [Reference hash](https://spec.matrix.org/v1.19/server-server-api/#calculating-the-reference-hash-for-an-event) | Ruma signature/hash utilities | `internal/verification.rs` | `signing/reference-hash/` | Targeted |
| `MX-SIG-007` | Drop the event if required signature checks fail. | [PDU receipt checks](https://spec.matrix.org/v1.19/server-server-api/#checks-performed-on-receipt-of-a-pdu) | `verify_event()` error | `evaluation.rs`, `decision.rs` | `signing/invalid-signature/` | Targeted |
| `MX-SIG-008` | If signatures pass but the content hash fails, continue with the locally calculated redacted representation. | [PDU receipt checks](https://spec.matrix.org/v1.19/server-server-api/#checks-performed-on-receipt-of-a-pdu), [Received hashes](https://spec.matrix.org/v1.19/server-server-api/#validating-hashes-and-signatures-on-received-events) | `verify_event()` returning signature-only verification | `internal/verification.rs` | `signing/hash-mismatch/` | Targeted |
| `MX-SIG-009` | Preserve multiple valid signatures correctly, including when a Policy Server shares the event-origin server name. | [v1.19 changelog](https://spec.matrix.org/v1.19/changelog/v1.19/#server-server-api) | Ruma behavior requires explicit regression verification | `internal/verification.rs`, `internal/policy_server.rs` | `signing/multiple-signatures/` | Gap |
| `MX-SIG-010` | For a restricted join with `join_authorised_via_users_server`, require a valid signature from that user’s server in addition to the sender-server requirement. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Required-signer discovery and `verify_event()`; exact released behavior requires fixtures | `internal/verification.rs`, `dependencies.rs` | `signing/restricted-join-authorizer/` | Gap |

### 5.4 Redaction behavior

Verification redaction and an `m.room.redaction` event’s later effect are separate responsibilities.

| ID | Matrix obligation | Matrix source | Ruma mapping | CyborgHeart owner | Fixture family | Status |
|---|---|---|---|---|---|---:|
| `MX-RED-001` | Produce the room-version-12 redacted representation used for hashing, signatures, and continued processing. | [RV12 Redactions](https://spec.matrix.org/v1.19/rooms/v12/#redactions) | Ruma room-version redaction support | `internal/verification.rs` | `room-v12/redaction/representation/` | Targeted |
| `MX-RED-002` | Authorize `m.room.redaction` through the ordinary event rules; applying its effect is a separate later check. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules), [Handling redactions](https://spec.matrix.org/v1.19/rooms/v12/#handling-redactions) | Authorization via Ruma; effect orchestration is CyborgHeart-owned | `internal/authorization.rs`, `internal/redaction.rs`, `consequences.rs` | `room-v12/redaction/events/` | Gap |
| `MX-RED-003` | Apply a redaction only when the redaction sender meets the redact level or shares the original sender’s domain. | [Handling redactions](https://spec.matrix.org/v1.19/rooms/v12/#handling-redactions) | No complete released-Ruma lifecycle mapping assumed | `internal/redaction.rs`, `consequences.rs` | `room-v12/redaction/application/` | Gap |
| `MX-RED-004` | Do not send the redaction to clients until both events are known and valid. | [Handling redactions](https://spec.matrix.org/v1.19/rooms/v12/#handling-redactions) | CyborgHeart pending-consequence model | `internal/redaction.rs`, `consequences.rs` | `room-v12/redaction/pending-partner/` | Gap |
| `MX-RED-005` | If the current pair does not permit application, wait for a valid partner event and re-check instead of recording a terminal denial. | [Handling redactions](https://spec.matrix.org/v1.19/rooms/v12/#handling-redactions) | CyborgHeart `pending_partner` consequence | `internal/redaction.rs`, `consequences.rs` | `room-v12/redaction/recheck/` | Gap |

### 5.5 Authorization

Ruma documents the required incoming-PDU order:

```text
format check
    → signature/hash verification
    → state-independent authorization
    → authorization against claimed auth events
    → authorization against state before
    → authorization against current state
```

| ID | Matrix obligation | Matrix source | Ruma mapping | CyborgHeart owner | Fixture family | Status |
|---|---|---|---|---|---|---:|
| `MX-AUTH-001` | Select the required authorization-event subset for a candidate event. | [Auth events selection](https://spec.matrix.org/v1.19/server-server-api/#auth-events-selection) | Ruma auth-event selection support | `internal/authorization.rs`, `dependencies.rs` | `room-v12/authorization/auth-event-selection/` | Targeted |
| `MX-AUTH-002` | Perform state-independent authorization checks. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | `check_state_independent_auth_rules()` | `internal/authorization.rs` | `room-v12/authorization/state-independent/` | Targeted |
| `MX-AUTH-003` | Authorize against the event’s claimed auth events; reject on failure. | [PDU receipt checks](https://spec.matrix.org/v1.19/server-server-api/#checks-performed-on-receipt-of-a-pdu) | `check_state_dependent_auth_rules()` | `internal/authorization.rs`, `decision.rs` | `room-v12/authorization/claimed-auth/` | Targeted |
| `MX-AUTH-004` | Reconstruct state before the event and authorize against it; reject on failure. | [PDU receipt checks](https://spec.matrix.org/v1.19/server-server-api/#checks-performed-on-receipt-of-a-pdu), [RV12 state before](https://spec.matrix.org/v1.19/rooms/v12/#state-resolution) | Ruma state resolution plus dependent event/state views | `internal/state.rs`, `internal/authorization.rs` | `room-v12/authorization/historical-state/` | Targeted |
| `MX-AUTH-005` | Authorize against current resolved room state; soft-fail on failure. | [Soft failure](https://spec.matrix.org/v1.19/server-server-api/#soft-failure) | `check_state_dependent_auth_rules()` | `internal/authorization.rs`, `decision.rs` | `room-v12/soft-failure/current-state/` | Targeted |
| `MX-AUTH-006` | Enforce create-event rules, including no predecessors, omitted `room_id`, room-version-12 creator fields, and the room/create identity relationship. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Ruma state-independent authorization plus CyborgHeart identity preflight | `internal/authorization.rs`, `room_version.rs` | `room-v12/authorization/create/` | Targeted |
| `MX-AUTH-007` | Enforce membership transitions and target/sender membership requirements. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Ruma state resolution authorization | `internal/authorization.rs` | `room-v12/authorization/membership/` | Targeted |
| `MX-AUTH-008` | Enforce join-rule and restricted-room authorization. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Ruma state resolution authorization | `internal/authorization.rs` | `room-v12/authorization/join-rules/` | Targeted |
| `MX-AUTH-009` | Enforce power-level rules and room-version-12 infinite creator power. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Ruma room-version-12 authorization; verify creator edge cases | `internal/authorization.rs` | `room-v12/authorization/power-levels/` | Targeted |
| `MX-AUTH-010` | Enforce third-party invite authorization where applicable. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Ruma state resolution authorization | `internal/authorization.rs` | `room-v12/authorization/third-party-invite/` | Targeted |
| `MX-AUTH-011` | Apply the general sender membership and required power-level rules to other events. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Ruma state resolution authorization | `internal/authorization.rs` | `room-v12/authorization/general/` | Targeted |
| `MX-AUTH-012` | For `m.room.create`, reject an optional `content.room_version` value only when it is not a recognized room version; do not add an equality requirement absent from the Matrix rule. | [RV12 Authorisation](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules) | Released Ruma assumes recognition occurred before state-independent auth | `room_version.rs`, `internal/authorization.rs` | `room-v12/authorization/create-room-version/` | Gap |

### 5.6 State before, state after, and state resolution

| ID | Matrix obligation | Matrix source | Ruma mapping | CyborgHeart owner | Fixture family | Status |
|---|---|---|---|---|---|---:|
| `MX-STATE-001` | For a message event, state after equals state before. | [RV12 State resolution](https://spec.matrix.org/v1.19/rooms/v12/#state-resolution) | CyborgHeart state consequence over Ruma IDs | `internal/state.rs`, `consequences.rs` | `room-v12/linear-history/message/` | Targeted |
| `MX-STATE-002` | For a state event, replace the matching `(type, state_key)` entry in state after. | [RV12 State resolution](https://spec.matrix.org/v1.19/rooms/v12/#state-resolution) | CyborgHeart consequence calculation | `internal/state.rs`, `consequences.rs` | `room-v12/linear-history/state/` | Targeted |
| `MX-STATE-003` | State before an event is the resolution of the states after its predecessors. | [RV12 State resolution](https://spec.matrix.org/v1.19/rooms/v12/#state-resolution) | `resolve()` | `internal/state.rs`, `dependencies.rs` | `room-v12/state-resolution/predecessors/` | Targeted |
| `MX-STATE-004` | Use room-version-12 state resolution, including its empty initial state for iterative auth and conflicted-state-subgraph changes. | [RV12 State resolution](https://spec.matrix.org/v1.19/rooms/v12/#state-resolution) | `resolve()` with room-version-12 rules | `internal/state.rs` | `room-v12/state-resolution/v12/` | Targeted |
| `MX-STATE-005` | Produce the same resolved state for the same state sets regardless of receipt order. | [Architecture: Room structure](https://spec.matrix.org/v1.19/#room-structure), [RV12 State resolution](https://spec.matrix.org/v1.19/rooms/v12/#state-resolution) | `resolve()` | `internal/state.rs` | `room-v12/state-resolution/order-independence/` | Targeted |
| `MX-STATE-006` | Handle rejected events in state resolution according to room-version rules. | [RV12 Rejected events](https://spec.matrix.org/v1.19/rooms/v12/#rejected-events) | Ruma resolution behavior; explicit fixtures required | `internal/state.rs` | `room-v12/state-resolution/rejected-events/` | Targeted |
| `MX-STATE-007` | Retain soft-failed events and their historical state-after for later graph and state-resolution participation, while excluding them from immediate local forward-extremity selection. | [Soft failure](https://spec.matrix.org/v1.19/server-server-api/#soft-failure) | Ruma resolves supplied states; lifecycle and consequence ownership are CyborgHeart’s | `decision.rs`, `consequences.rs`, `internal/state.rs` | `room-v12/soft-failure/later-reference/` | Targeted |

### 5.7 Dispositions and graph consequences

| ID | Matrix obligation | Matrix source | Ruma mapping | CyborgHeart owner | Fixture family | Status |
|---|---|---|---|---|---|---:|
| `MX-DISP-001` | Drop malformed or signature-invalid events before normal room-history processing. | [PDU receipt checks](https://spec.matrix.org/v1.19/server-server-api/#checks-performed-on-receipt-of-a-pdu) | Ruma format and verification errors | `decision.rs`, `evaluation.rs` | `room-v12/format/`, `signing/` | Targeted |
| `MX-DISP-002` | Rejected events are not sent to clients or selected as predecessors for local events; their state event does not update state. | [Rejection](https://spec.matrix.org/v1.19/server-server-api/#rejection) | Authorization result from Ruma; consequences owned by CyborgHeart | `decision.rs`, `consequences.rs` | `room-v12/rejection/` | Targeted |
| `MX-DISP-003` | Events referencing rejected predecessors may still be processed if they pass authorization. | [Rejection](https://spec.matrix.org/v1.19/server-server-api/#rejection) | Ruma authorization with supplied facts | `internal/state.rs`, `consequences.rs` | `room-v12/rejection/referenced/` | Targeted |
| `MX-DISP-004` | Soft-failed events are not sent normally to clients or added as forward extremities for local construction. | [Soft failure](https://spec.matrix.org/v1.19/server-server-api/#soft-failure) | Current-state auth failure from Ruma; consequences owned by CyborgHeart | `decision.rs`, `consequences.rs` | `room-v12/soft-failure/` | Targeted |
| `MX-DISP-005` | Soft-failed events remain available to federation and later state resolution. | [Soft failure](https://spec.matrix.org/v1.19/server-server-api/#soft-failure) | CyborgHeart lifecycle and later runtime storage policy | `consequences.rs` | `room-v12/soft-failure/later-reference/` | Targeted |
| `MX-GRAPH-001` | Interpret `prev_events` as DAG parents and update forward-extremity consequences consistently. | [PDUs](https://spec.matrix.org/v1.19/server-server-api/#pdus), [Event Graphs](https://spec.matrix.org/v1.19/#event-graphs) | Ruma event IDs; CyborgHeart consequence calculation | `consequences.rs`, `internal/state.rs` | `room-v12/linear-history/`, `state-resolution/` | Targeted |

---

## 6. Policy Server gap and ownership

Matrix v1.19 requires a final incoming-PDU check for rooms using a Matrix Policy Server.

### 6.1 Required behavior

| ID | Matrix obligation | Source | Owner | Status |
|---|---|---|---|---:|
| `MX-PS-001` | Determine whether current state enables a Policy Server through a valid `m.room.policy` event and a joined user from the configured `via` server. | [Determining Policy Server use](https://spec.matrix.org/v1.19/server-server-api/#determining-if-a-policy-server-is-enabled-in-a-room) | Event application | Gap |
| `MX-PS-002` | Exempt `m.room.policy` events with empty state key from requiring the Policy Server signature. | [Validating Policy Server signatures](https://spec.matrix.org/v1.19/server-server-api/#validating-policy-server-signatures) | Event application | Gap |
| `MX-PS-003` | Validate `ed25519:policy_server` with the public key embedded in active room policy state. | [Validating Policy Server signatures](https://spec.matrix.org/v1.19/server-server-api/#validating-policy-server-signatures) | Event application | Gap |
| `MX-PS-004` | Preserve ordinary origin signatures when the Policy Server name equals the event-origin server name. | [v1.19 changelog](https://spec.matrix.org/v1.19/changelog/v1.19/#server-server-api) | Event application and host | Gap |
| `MX-PS-005` | If a required recommendation is missing or invalid, attempt to obtain a fresh Policy Server signature. | [Asking for a Policy Server signature](https://spec.matrix.org/v1.19/server-server-api/#asking-for-a-policy-server-signature-on-an-event) | Future transaction/federation or command host | Outside engine |
| `MX-PS-006` | If no valid recommendation is available after the required host attempt, soft-fail a federated event or a local event deliberately processed anyway. | [Validating Policy Server signatures](https://spec.matrix.org/v1.19/server-server-api/#validating-policy-server-signatures) | Event application from an explicit terminal recommendation fact | Gap |

### 6.2 Architectural resolution

The pure engine never calls `POST /_matrix/policy/v1/sign`.

```text
event application
    detects active room policy
    validates supplied recommendation material
    requests a Policy Server recommendation fact if needed
        ↓
host/runtime
    obtains or refreshes a Policy Server signature
        ↓
event application
    re-evaluates with signature material or a terminal acquisition outcome
    accepts normal processing or soft-fails
```

The Policy Server recommendation fact is bound to:

- the candidate event;
- the active `m.room.policy` state event;
- the configured server and embedded public key;
- the room-version signing rules.

The engine validates supplied signature material itself. It does not trust a host-provided validity flag. A Policy Server recommendation does not replace ordinary Matrix authorization.

Required fixture families:

```text
room-v12/policy-server/
├── disabled/
├── enabled-valid/
├── empty-state-key-exemption/
├── missing-signature/
├── invalid-signature/
├── final-soft-failure/
└── same-name-as-origin/
```

## 7. Server ACL boundary

Per-PDU server ACL checks are required for federation ingress, but they are not part of room-event application.

| ID | Obligation | Source | Owner | Status |
|---|---|---|---|---:|
| `MX-ACL-001` | Apply room server ACLs to the requesting server on each PDU in a federation transaction. | [Server ACLs](https://spec.matrix.org/v1.19/server-server-api/#server-access-control-lists-acls) | Future federation ingress | Outside engine |
| `MX-ACL-002` | Ignore denied PDUs and report an event-specific transaction error. | [Server ACLs](https://spec.matrix.org/v1.19/server-server-api/#server-access-control-lists-acls) | Future federation ingress | Outside engine |

The event engine may evaluate an event independently of its transport sender. Federation ingress decides whether that candidate is admitted to the engine at all.

---

## 8. CyborgHeart architecture requirements

The following are CyborgHeart contracts, not direct Matrix specification requirements.

| ID | Requirement | Source | Owner | Fixture or test |
|---|---|---|---|---|
| `CH-ARCH-001` | Identical explicit inputs produce identical observable outputs. | `MANIFESTO.md`, `ARCHITECTURE.md` | Entire event engine | deterministic replay |
| `CH-ARCH-002` | Facts required to decide an event produce `NeedsDependencies`; facts needed only for deferred consequences do not. | `WORKING-THESIS.md`, `ARCHITECTURE.md` | `dependencies.rs`, `evaluation.rs` | dependency scenarios |
| `CH-ARCH-003` | Adding valid facts cannot change one complete decision into another. | `ARCHITECTURE.md` | Event engine | dependency-monotonicity property tests |
| `CH-ARCH-004` | Untrusted-work dimensions are explicitly budgeted. | `MANIFESTO.md`, `ARCHITECTURE.md` | `budget.rs` | adversarial work ceilings |
| `CH-ARCH-005` | Budget exhaustion halts without inventing a Matrix disposition. | `WORKING-LEXICON.md`, `ARCHITECTURE.md` | `evaluation.rs`, `decision.rs` | budget-exceeded fixtures |
| `CH-ARCH-006` | Event application performs no I/O or durable duplicate detection. | `ARCHITECTURE.md` | Crate boundary | dependency and architecture tests |
| `CH-ARCH-007` | Public results carry structured evidence rather than relying on logs. | `MANIFESTO.md`, `WORKING-LEXICON.md` | `decision.rs` | expected evidence |
| `CH-ARCH-008` | Observable collection ordering is stable. | `ARCHITECTURE.md` | Public result construction | order-randomization property tests |
| `CH-ARCH-009` | Unsupported room versions fail at the capability boundary rather than becoming `Dropped`. | `SUPPORTED-SURFACE.md`, `ARCHITECTURE.md` | Room-version admission | unsupported-version boundary tests |
| `CH-ARCH-010` | A missing, not-yet-valid, or currently nonqualifying redaction partner yields `pending_partner` rather than `NeedsDependencies` or a terminal denial. | `ARCHITECTURE.md`, `WORKING-LEXICON.md` | `internal/redaction.rs`, `consequences.rs` | pending-partner and recheck fixtures |

These rows must not be cited as though Matrix mandates CyborgHeart’s internal architecture.

---

## 9. Ruma coverage and gap register

`RUMA-COVERAGE.md` records exact crate paths, functions, versions, assumptions, and upstream gaps. The initial register is:

| Gap ID | Finding | Required action |
|---|---|---|
| `GAP-RUMA-001` | The candidate `ruma 0.16.0` baseline documents Matrix v1.18 coverage while CyborgHeart targets v1.19. | Audit the v1.19 delta and retain regression fixtures for every relevant clarification. |
| `GAP-RUMA-002` | Released `ruma-state-res` does not own the Matrix Policy Server lifecycle. | Implement explicit recommendation facts and an isolated verifier until a suitable released helper is adopted. |
| `GAP-RUMA-003` | Room-version-12 room/create identity composition is not exposed as one complete public Ruma operation. | Isolate the composition in `room_version.rs` and prove it with vectors. |
| `GAP-RUMA-004` | Required-signer discovery returns server identities, while CyborgHeart must distinguish absent signatures, absent local keys, and invalid signatures. | Preflight candidate signature key IDs and map missing local key facts to `NeedsDependencies`. |
| `GAP-RUMA-005` | Ruma authorization does not complete `m.room.redaction` partner-effect handling. | Implement `applied` and `pending_partner` consequences with re-evaluation. |
| `GAP-RUMA-006` | Matrix v1.19 clarifies multiple-signature handling and Policy Server/origin name collision. | Add dedicated verification regressions. |
| `GAP-RUMA-007` | `verify_event()` verifies supplied keys but does not acquire keys or establish Matrix signing-key validity metadata. | Model explicit key metadata and validity-reference time; enforce event-time and seven-day effective validity. |
| `GAP-RUMA-008` | Ruma authorization callbacks can report missing referenced auth or create events as errors. | Preflight those events and return `NeedsDependencies` before invoking authorization. |
| `GAP-RUMA-009` | Released `check_pdu_format()` validates a defined subset, not every required room-version-12 PDU field. | Add a narrow canonical-field preflight without strict typed-content overvalidation. |
| `GAP-RUMA-010` | Released state-independent authorization assumes `content.room_version` recognition was performed before the call. | Validate recognition explicitly; fixture unrecognized rejection and ensure a recognized differing value is not rejected for mismatch alone. |

No gap is permission to build a broad shadow implementation of Ruma.

## 10. Fixture coverage matrix

The initial corpus must contain these families before the complete room-version-12 target can be marked supported:

```text
fixtures/matrix-v1.19/
├── canonical-json/
├── signing/
│   ├── base64/
│   ├── content-hash/
│   ├── reference-hash/
│   ├── signatures/
│   ├── required-signers/
│   ├── key-validity/
│   ├── hash-mismatch/
│   └── multiple-signatures/
└── room-v12/
    ├── create/
    ├── event-ids/
    ├── format/
    ├── authorization/
    │   ├── state-independent/
    │   ├── claimed-auth/
    │   ├── historical-state/
    │   ├── current-state/
    │   ├── create/
    │   ├── membership/
    │   ├── join-rules/
    │   ├── power-levels/
    │   ├── third-party-invite/
    │   └── general/
    ├── linear-history/
    ├── redaction/
    ├── rejection/
    ├── soft-failure/
    ├── policy-server/
    └── state-resolution/
```

Directories should be created when their first fixture is added, not as empty placeholders.

---

## 11. Implementation-stage mapping

### Stage 1 — contract skeleton

Maps:

```text
CH-ARCH-001 through CH-ARCH-010
```

No Matrix support claim is made.

### Stage 2 — PDU and cryptographic trust

Maps:

```text
MX-EVT-001
MX-JSON-001..002
MX-ID-001..002
MX-PDU-001..006
MX-SIG-002..010
MX-RED-001
```

Potential support statement remains partial: verified candidate intake only.

### Stage 3 — linear authorization and state

Maps:

```text
MX-AUTH-001..011
MX-STATE-001..003
MX-DISP-001..004
MX-GRAPH-001
```

Potential support statement: narrow linear room-version-12 event application.

### Stage 4 — redaction, Policy Server, and exceptional outcomes

Maps:

```text
MX-RED-002..005
MX-PS-001..006
MX-DISP-002..005
```

A full Matrix v1.19 PDU-lifecycle claim is not valid without resolving these rows.

### Stage 5 — branching histories

Maps:

```text
MX-STATE-003..007
MX-GRAPH-001
```

This completes the initial target only when adversarial and order-independence evidence also passes.

---

## 12. Promotion rule

A map row may move to **Supported** only when:

1. its Matrix source is still current for the declared baseline;
2. its Ruma mapping is confirmed against the pinned release;
3. its CyborgHeart owner exists;
4. positive and negative fixtures exist;
5. malformed or adversarial cases exist where applicable;
6. deterministic replay passes;
7. dependency behavior is explicit;
8. no known normative case in the row remains failing;
9. `SUPPORTED-SURFACE.md` is updated in the same pull request.

A feature group is supported only when every mandatory row in that group is supported.

---

## 13. Maintenance

Update this document when:

- the Matrix baseline changes;
- a room version is added;
- a Ruma dependency changes;
- a lifecycle stage changes owner;
- a new normative obligation is discovered;
- a fixture exposes an incorrect mapping;
- a support claim changes.

Do not delete historical gaps merely because they are resolved. Mark them resolved in `RUMA-COVERAGE.md` or the relevant ADR so the reasoning remains recoverable.

---

## 14. Source index

`REFERENCES.md` is the canonical external source index for this map. Normative links remain inline on individual rows so each obligation is reviewable in isolation.

## 15. Final traceability commitment

> CyborgHeart will not infer support from dependency availability or architectural intent. Every claimed Matrix behavior must remain traceable from the normative specification, through the exact Ruma capability or documented gap, into an owning module, a fixture family, and a passing support gate.

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

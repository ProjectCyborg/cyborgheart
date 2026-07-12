# CyborgHeart Ruma Coverage

**Status:** Initial dependency-coverage baseline
**Matrix baseline:** v1.19
**Initial room version:** 12 only
**Candidate Ruma baseline:** `ruma 0.16.0`, `ruma-events 0.34.0`, `ruma-state-res 0.17.0`, `ruma-signatures 0.21.0`
**Last reviewed:** 2026-07-12

---

## 1. Purpose

This document records the exact Ruma surface CyborgHeart intends to use, what Matrix responsibility each API serves, what CyborgHeart must add around it, and where released Ruma does not yet close the Matrix v1.19 gap.

It is the dependency-level companion to [`SPECIFICATION-MAP.md`](./SPECIFICATION-MAP.md).

> Ruma availability is not a CyborgHeart support claim. Support exists only when the pinned API, lifecycle integration, fixtures, and documented Matrix surface all agree.

The versions above are the reviewed candidate baseline. The first committed `Cargo.toml` and `Cargo.lock` become the operational source of truth.

---

## 2. Coverage states

| State | Meaning |
|---|---|
| **Adopt** | Use the released Ruma API directly as the authoritative implementation for that responsibility. |
| **Wrap** | Use Ruma, but translate its types, errors, or lifecycle shape into CyborgHeart’s public contract. |
| **Verify** | The API appears suitable, but released-version behavior must be confirmed by fixtures before support is claimed. |
| **Gap** | Required Matrix behavior is not completely available through the candidate released Ruma surface. |
| **Deferred** | Useful later, but outside the initial event-application surface. |
| **Rejected** | Deliberately not used because it would broaden scope or weaken the boundary. |

---

## 3. Dependency policy

The initial workspace should prefer the umbrella crate:

```toml
[workspace.dependencies]
ruma = {
    version = "0.16.0",
    default-features = false,
    features = ["events", "signatures", "state-res"],
}
```

This gives CyborgHeart a coherent Ruma release family while keeping the initial feature set narrow.

Initial policy:

- use crates.io releases, not a Git revision;
- commit `Cargo.lock`;
- do not enable `full`;
- do not enable API features;
- do not enable unstable MSC features;
- do not enable compatibility features without a fixture-backed need and ADR;
- do not expose Ruma error strings or temporary helper types as CyborgHeart’s stable public API;
- review every Ruma update against this document, the specification map, and the conformance corpus.

The `events` feature is enabled deliberately, not as permission to strictly deserialize untrusted PDUs. The public `ruma-state-res::Event` trait names `ruma-events::TimelineEventType`, while the umbrella crate re-exports `ruma-events` only when `events` is enabled. CyborgHeart therefore needs the feature to implement the server-side adapter through the umbrella dependency.

Untrusted PDUs still enter as canonical JSON and use Ruma's lazy server-side helpers. Strict typed event-content deserialization is reserved for contexts where Matrix permits it, such as local construction or client-originated content validation.

---

## 4. Version and specification boundary

The candidate releases document Matrix v1.18 links in several crate-level references, while CyborgHeart targets Matrix v1.19.

This does not mean the candidate release is unusable. It means the v1.19 delta must be audited explicitly.

Relevant v1.19 checks include:

- canonical JSON wording clarifications;
- explicit URL-safe unpadded Base64 wording;
- multiple-signature handling clarification;
- preserving origin signatures when a Policy Server shares the origin server name;
- Policy Server recommendation handling in the incoming-PDU lifecycle.

No broad statement such as “Ruma supports Matrix v1.19” should appear in CyborgHeart until each relied-on delta is mapped and tested.

---

## 5. Crate and feature map

| Capability | Released crate | Umbrella feature | Initial use |
|---|---|---|---|
| IDs, canonical JSON, room-version rules | `ruma-common` through `ruma` | always available through `ruma` | Adopt |
| Event signatures and hashes | `ruma-signatures 0.21.0` | `signatures` | Adopt and wrap |
| Event authorization and state resolution | `ruma-state-res 0.17.0` | `state-res` | Adopt and wrap |
| Event type enums and typed content | `ruma-events 0.34.0` | `events` | Adopt required public types; defer strict typed-content validation for untrusted PDUs |
| Client API models | `ruma-client-api` | `client-api-*` | Deferred |
| Federation API models | `ruma-federation-api` | `federation-api-*` | Deferred |

---

## 6. Common types and room-version rules

| Ruma surface | Matrix responsibility | CyborgHeart owner | Coverage | Notes |
|---|---|---|---:|---|
| `CanonicalJsonObject` | Canonical event representation | `input.rs`, `internal/pdu.rs` | Adopt | The event engine accepts canonical objects, not arbitrary `serde_json::Value`. |
| `CanonicalJsonValue` | Canonical field values | PDU inspection and evidence | Adopt | Do not add a second canonical JSON model. |
| `RoomVersionId` | External Matrix room-version identifier | capability admission | Wrap | External IDs must map into `SupportedRoomVersion`; recognition is not support. |
| `RoomVersionRules` | Version-specific rule bundle | `room_version.rs` | Adopt | Resolve once per evaluation and pass the rule bundle to Ruma. |
| `EventFormatRules` | PDU format differences | PDU format stage | Adopt | Used by `check_pdu_format()`. |
| `AuthorizationRules` | Auth-rule differences | authorization stages | Adopt | Used by auth selection and checks. |
| `StateResolutionV2Rules` | State-resolution differences | state reconstruction | Adopt | Room version 12 enables the conflicted-state-subgraph and empty-initial-state changes. |
| `OwnedEventId`, `OwnedRoomId`, `OwnedServerName`, `OwnedUserId` | Matrix identifiers | public domain types | Adopt | Prefer Ruma owned and borrowed identifier types. |
| Base64 wrappers | Matrix Base64 parsing | verification | Adopt | Add v1.19 vectors for standard and URL-safe unpadded forms. |

### Room-version admission

CyborgHeart must expose an explicit support type:

```rust
enum SupportedRoomVersion {
    V12,
}
```

`RoomVersionId::rules()` may provide rules for other versions, but CyborgHeart must not accept them unless the project has deliberately promoted them through `SUPPORTED-SURFACE.md`.

---

## 7. PDU format and server-side event abstraction

### `check_pdu_format()`

**Released surface:** `ruma_state_res::check_pdu_format`

```rust
pub fn check_pdu_format(
    pdu: &CanonicalJsonObject,
    rules: &EventFormatRules,
) -> Result<(), String>
```

Coverage: **Wrap**

It checks total PDU size and a defined server-relevant subset of the event format: `type`, `sender`, room-version-sensitive `room_id` and `event_id`, optional `state_key`, `prev_events`, `auth_events`, and `depth`. It does not by itself validate every required PDU field, including `content`, `origin_server_ts`, `hashes`, and `signatures`.

CyborgHeart responsibilities:

- perform a narrow structural preflight for required fields not covered by the released helper before constructing the infallible Ruma `Event` adapter;
- charge the relevant work budget before or during the call;
- map failure to `drop.invalid_pdu_format`;
- retain the Ruma message only as unstable diagnostic detail;
- never expose the raw `String` as a stable decision code;
- add room-version-12 fixtures for create events, room IDs, depth, array limits, field types, and total size.

### `Event` trait

**Released surface:** `ruma_state_res::Event`

Coverage: **Adopt through an internal adapter**

The trait exposes the server-relevant PDU surface:

- event ID;
- optional room ID;
- sender;
- origin timestamp;
- event type and raw content;
- state key;
- predecessor and auth-event IDs;
- redaction target;
- rejected status.

CyborgHeart should implement this trait for a private lazy PDU adapter over canonical JSON plus evaluated metadata.

The adapter must not be a second public Matrix event model. It exists to provide Ruma exactly the fields its algorithms require.

### Lazy helper event types

Ruma state resolution provides lazy helper types for create, membership, power-level, join-rule, and third-party-invite events.

Coverage: **Adopt indirectly**

These are preferred over strict `ruma-events` deserialization for untrusted federated PDUs because they avoid validating fields that the server is not permitted to use as rejection grounds.

---

## 8. Authorization coverage

### `auth_types_for_event()`

Coverage: **Adopt and wrap**

Purpose:

- determine the required `(event type, state key)` authorization tuples;
- support claimed-auth validation and future local event construction.

CyborgHeart responsibilities:

- resolve directly referenced auth-event IDs before state-independent authorization;
- distinguish missing event facts from an invalid auth-event set;
- map content-format failures into the relevant authorization decision context rather than a generic parser error.

### `check_state_independent_auth_rules()`

Coverage: **Adopt and wrap**

Purpose:

- validate create-event rules that do not require room state;
- validate the claimed `auth_events` list, including room consistency, duplicate tuples, unexpected tuples, rejected auth events, and the room-version-12 create-event relationship.

CyborgHeart responsibilities:

- preflight all referenced event dependencies so a missing local fact does not become a rejection;
- validate a create event's optional `content.room_version` before the call because Ruma assumes recognition already occurred: reject an unrecognized value, but do not impose a non-normative equality check against the externally selected room version;
- call the function once per candidate;
- map failure to `reject.state_independent_auth`;
- preserve a structured stage and optional diagnostic detail.

### `check_state_dependent_auth_rules()`

Coverage: **Adopt and wrap**

The same released function is called in three different Matrix contexts:

1. claimed auth events — failure means `Rejected`;
2. state before the event — failure means `Rejected`;
3. current room state — failure means `SoftFailed`.

CyborgHeart must not map the Ruma error without the call context. The context determines the stable decision code:

```text
reject.claimed_auth
reject.historical_auth
soft_fail.current_state_auth
```

### Error stability

The released authorization functions return `Result<(), String>`.

Coverage: **Gap in stable diagnostics; not a correctness gap**

CyborgHeart must:

- define its own stable top-level codes;
- treat Ruma’s text as diagnostic only;
- avoid parsing error strings to infer protocol meaning;
- use the lifecycle call site as the stable classification boundary.

---

## 9. State-resolution coverage

### `StateMap<T>`

Coverage: **Adopt internally**

Ruma defines state as a map from `(StateEventType, state_key)` to a value, normally an event ID.

CyborgHeart may expose a storage-neutral state map or wrapper publicly, but it must not expose database layout through this type.

### `resolve()`

Coverage: **Adopt and orchestrate**

The released function requires:

- authorization rules;
- state-resolution-v2 rules;
- predecessor state maps;
- full auth-chain sets for each state map;
- event lookup;
- a conflicted-state-subgraph callback for room versions that require it.

CyborgHeart responsibilities:

- construct complete predecessor state inputs;
- discover concrete missing event dependencies;
- derive full auth chains from event facts;
- calculate or supply the room-version-12 conflicted state subgraph;
- ensure all events belong to the same room;
- charge graph, auth-chain, state-entry, and iteration budgets;
- map missing facts to `NeedsDependencies` before invoking `resolve()`;
- map an unexpected Ruma fetch failure after preflight to an internal contract fault, not a Matrix disposition.

### Room-version-12 behavior

The released state-resolution implementation includes room-version-12 switches for:

- adding the conflicted state subgraph to the full conflicted set;
- beginning the first iterative-auth phase with an empty state map.

Coverage: **Verify with dedicated fixtures**

Do not infer complete room-version-12 support from the existence of these switches alone.

---

## 10. Signature and hash coverage

### `required_server_signatures_to_verify_event()`

Coverage: **Adopt and wrap**

Purpose:

- identify the server names whose signatures must be verified for the event under the selected signature rules.

It returns required server names, not a complete key-acquisition plan.

CyborgHeart must additionally:

- inspect the candidate's signature map for key IDs belonging to each required server;
- request all candidate signing-key facts for those key IDs;
- freeze an explicit key-validity reference timestamp for the evaluation sequence;
- require `origin_server_ts <= min(valid_until_ts, key_validity_reference_ts + 7 days)` before a key is eligible;
- provide Ruma all locally known eligible keys for required entities so v1.19 multiple-signature behavior is preserved;
- distinguish an event with no required signature from a locally missing or ineligible public key;
- return `NeedsDependencies` for absent key facts;
- return a drop only when the event's required signature is absent, malformed, unsupported, or invalid after the necessary key facts are complete.

Ruma verifies the supplied key bytes but does not acquire key metadata or establish the Matrix validity window. That orchestration remains a CyborgHeart responsibility.

### `verify_event()`

Coverage: **Adopt and wrap**

```rust
pub fn verify_event(
    public_key_map: &PublicKeyMap,
    object: &CanonicalJsonObject,
    rules: &RoomVersionRules,
) -> Result<Verified, VerificationError>
```

Stable interpretation:

| Ruma result | CyborgHeart meaning |
|---|---|
| `Ok(Verified::All)` | Required signatures and content hash are valid. |
| `Ok(Verified::Signatures)` | Required signatures are valid; content hash differs. Continue with the room-version-redacted authorization representation. |
| verification error caused by locally absent key facts | Preflight bug or incomplete dependency snapshot; do not classify as invalid event. |
| verification error after complete key facts | Drop under the appropriate cryptographic code. |

CyborgHeart must preserve the distinction between the received representation and the effective redacted representation used for authorization. Unverified content must not become authoritative semantic input.

In the reviewed `0.21.0` implementation, event signatures whose key IDs are absent from the supplied key map are skipped, while matching supported signatures are verified. CyborgHeart must therefore supply every locally known eligible key for each required entity after validity filtering. Deliberately withholding a known eligible key would change protocol behavior and violates the dependency contract.

### `content_hash()` and `reference_hash()`

Coverage: **Adopt**

Use Ruma for content and reference hashing. Do not implement independent canonicalization or hashing.

CyborgHeart uses the reference hash to derive the room-version-12 event identity and to verify fixture expectations.

### Signing functions

Released functions include:

- `add_content_hash_to_event()`;
- `sign_event()`;
- `hash_and_sign_event()`;
- `sign_json()`.

Coverage: **Deferred to command runtime and server-key capabilities**

The event engine verifies candidate events. It does not own local key material or event construction.

---

## 11. Policy Server coverage

Matrix v1.19 Policy Server behavior is the largest identified released-Ruma gap for the initial lifecycle.

### Released baseline

`ruma-signatures 0.21.0` added separate hashing and signing steps to support additional signatures, but its changelog places `verify_policy_server_signature()` under **Unreleased**, not under `0.21.0`.

Therefore:

```text
verify_policy_server_signature()
    must not be treated as available in the 0.21.0 release
```

Coverage: **Gap**

### Initial CyborgHeart resolution

At repository initialization, choose one of these paths after checking the newest released Ruma version:

1. **Preferred:** use a released Ruma Policy Server verification helper if one is available and fixture-verified;
2. **Fallback:** implement one narrow internal adapter using Ruma canonical JSON, room-version redaction rules, Base64 types, and `verify_canonical_json_bytes()`.

The fallback must:

- live only in `internal/policy.rs`;
- implement no general-purpose signature framework;
- preserve the empty-state-key exemption;
- verify the `ed25519:policy_server` signature against the public key embedded in active `m.room.policy` state;
- preserve ordinary origin signatures when the Policy Server shares the origin name;
- have complete v1.19 fixtures;
- be marked for replacement by the released Ruma helper;
- be recorded in an ADR.

Policy Server signature acquisition remains outside the engine.

---

## 12. Room-version-12 identity gaps

### Event IDs

Ruma reference hashing is adopted. CyborgHeart owns the small room-version-aware composition that turns the hash into an event ID.

Coverage: **Verify**

Required fixtures:

- official or independently cross-checked reference-hash vectors;
- create, state, and message events;
- signature-order independence;
- unsigned-field independence;
- redaction-sensitive reference hashes.

### Room IDs and create-event relationship

Ruma’s state-independent authorization handles the room-version-12 relationship between a room ID and its create event internally. The exact helper used inside `ruma-state-res` is not part of its public API.

Coverage: **Gap in public composition, covered internally by Ruma auth**

CyborgHeart still needs a stable internal way to request the create event derived from the room ID before calling the auth function.

The implementation may perform the room-version-12 sigil transformation locally, but it must:

- use Ruma identifier validation;
- avoid reimplementing reference hashing;
- be fixture-backed;
- remain isolated in `room_version.rs`.

---

## 13. Redaction coverage

Ruma owns room-version redaction rules and the redacted representation used by `verify_event()`.

Coverage: **Adopt for verification representation**

Ruma authorization does not complete the lifecycle effect of an accepted `m.room.redaction` event on its target.

Coverage: **CyborgHeart-owned gap**

CyborgHeart must determine:

- whether the redaction event is accepted through ordinary authorization;
- whether a valid target partner is available;
- whether the target-effect rules permit redaction;
- whether the effect is `applied` or remains `pending_partner` for later re-checking.

A missing, not-yet-valid, or currently nonqualifying partner is a pending consequence, not a missing dependency for the redaction event's disposition. Matrix does not define a terminal `not_permitted` redaction effect here.

---

## 14. Error and type adaptation policy

Ruma types are used directly where they express stable Matrix domain concepts.

Ruma errors are wrapped where they expose implementation detail or unstable text.

### Directly reusable

- Matrix identifier types;
- canonical JSON types;
- room-version rule types;
- `Verified` as an internal cryptographic branch;
- state maps and event IDs inside the engine.

### Internal only

- Ruma authorization error strings;
- `VerificationError` variants as public decision codes;
- lazy helper wrappers;
- internal state-resolution maps and sets;
- Ruma-specific closure shapes.

### Public CyborgHeart boundary

CyborgHeart exposes:

- supported room version;
- candidate event;
- immutable dependency snapshot;
- dependency requests;
- evaluation result;
- stable decision codes;
- semantic consequences;
- structured evidence and work report.

---

## 15. Coverage register

| Coverage ID | Ruma surface | Status | CyborgHeart action |
|---|---|---:|---|
| `RU-CANON-001` | canonical JSON types | Adopt | Use directly; add v1.19 canonical vectors. |
| `RU-RULES-001` | `RoomVersionRules` | Adopt | Centralize under `SupportedRoomVersion::V12`. |
| `RU-FORMAT-001` | `check_pdu_format()` | Wrap | Stable drop code; supplement its documented field subset with CyborgHeart structural preflight. |
| `RU-EVENT-001` | `Event` trait and `TimelineEventType` | Adopt | Enable umbrella `events`; implement a private canonical-PDU adapter without strict full-content validation. |
| `RU-AUTH-001` | `auth_types_for_event()` | Adopt | Preflight event dependencies. |
| `RU-AUTH-002` | state-independent auth | Wrap | Prevalidate create `content.room_version` recognition without adding an equality rule; map call-site failure to rejection. |
| `RU-AUTH-003` | state-dependent auth | Wrap | Map by claimed, historical, or current context. |
| `RU-STATE-001` | `resolve()` | Orchestrate | Supply auth chains, events, subgraph, and budgets. |
| `RU-SIG-001` | required signer discovery | Wrap | Convert server names and signature key IDs into key dependencies with frozen event-time validity context. |
| `RU-SIG-002` | `verify_event()` | Wrap | Prefilter keys by Matrix validity metadata; preserve `All` versus `Signatures`; map complete-key errors. |
| `RU-HASH-001` | content and reference hashes | Adopt | No local hashing implementation. |
| `RU-POLICY-001` | Policy Server verification | Gap | Use next released helper or isolated adapter. |
| `RU-REDACT-001` | redacted verification representation | Adopt | Use Ruma room-version rules. |
| `RU-REDACT-002` | target redaction effect | Gap | CyborgHeart semantic consequence. |
| `RU-ID-001` | v12 event identity composition | Verify | Isolated room-version helper and vectors. |
| `RU-ID-002` | v12 room/create relationship | Verify | Preload create event and rely on Ruma auth check. |

---

## 16. Update procedure

When Ruma changes:

1. update the candidate or pinned version in a dedicated pull request;
2. inspect changelogs for `ruma`, `ruma-common`, `ruma-events`, `ruma-state-res`, `ruma-signatures`, and any newly enabled crate;
3. compare public signatures used by CyborgHeart;
4. run the full conformance, adversarial, property, and regression corpus;
5. audit Matrix-version references in Ruma documentation;
6. review resolved and newly introduced gaps;
7. update this document and `SPECIFICATION-MAP.md`;
8. do not automatically widen `SUPPORTED-SURFACE.md`.

A dependency update is not a protocol-support update unless the support gate is separately satisfied.

---

## 17. Upstream contribution rule

When CyborgHeart finds a Ruma defect or missing reusable primitive:

- minimize it into a fixture;
- confirm the Matrix requirement;
- reproduce it against the pinned release and current upstream;
- report or contribute upstream where appropriate;
- isolate any temporary local adapter;
- remove the adapter after a suitable released fix is adopted.

CyborgHeart should improve the Ruma boundary rather than accumulate a permanent shadow implementation.

---

## 18. Final coverage commitment

> CyborgHeart uses Ruma as the authoritative Rust implementation of Matrix primitives and core algorithms wherever the released surface is sufficient. It wraps Ruma to preserve lifecycle meaning, explicit dependencies, stable decisions, bounded work, and evidence. Every remaining gap is named, isolated, fixture-backed, and either owned deliberately by CyborgHeart or targeted for upstream resolution.

---

## Primary sources

- [Ruma 0.16.0](https://docs.rs/ruma/0.16.0/ruma/)
- [ruma-state-res 0.17.0](https://docs.rs/ruma-state-res/0.17.0/ruma_state_res/)
- [ruma-signatures 0.21.0](https://docs.rs/ruma-signatures/0.21.0/ruma_signatures/)
- [Ruma repository](https://github.com/ruma/ruma)
- [ruma-events 0.34.0 source](https://github.com/ruma/ruma/tree/ruma-events-0.34.0/crates/ruma-events)
- [ruma-state-res 0.17.0 source](https://github.com/ruma/ruma/tree/ruma-state-res-0.17.0/crates/ruma-state-res)
- [ruma-signatures 0.21.0 source](https://github.com/ruma/ruma/tree/ruma-signatures-0.21.0/crates/ruma-signatures)
- [ruma-state-res changelog](https://github.com/ruma/ruma/blob/main/crates/ruma-state-res/CHANGELOG.md)
- [ruma-signatures changelog](https://github.com/ruma/ruma/blob/main/crates/ruma-signatures/CHANGELOG.md)
- [`SPECIFICATION-MAP.md`](./SPECIFICATION-MAP.md)
- [`REFERENCES.md`](./REFERENCES.md)

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

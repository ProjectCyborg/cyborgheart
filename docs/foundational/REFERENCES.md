# CyborgHeart References

**Status:** Canonical source index
**Purpose:** Keep external authorities, implementation dependencies, and documentation ownership in one place.
**Last reviewed:** 2026-07-13

This file is the canonical external-source index for the CyborgHeart foundational pack. Individual documents keep inline links only where a specific obligation must be reviewable in place; they should not maintain separate long reference appendices.

## 1. Source hierarchy

CyborgHeart uses this order of authority:

1. Matrix specification — normative protocol behavior.
2. Applicable room-version rules — room-specific normative behavior.
3. Released Ruma documentation and reproduced behavior — Rust primitives and algorithms used by CyborgHeart.
4. CyborgHeart support and architecture documents — project claims and boundaries.
5. Implementation and fixtures — executable evidence.

When sources appear to disagree, Matrix determines intended protocol behavior, the pinned Ruma release is reproduced, the mismatch is recorded in `RUMA-COVERAGE.md`, and CyborgHeart makes no support claim until the gap is resolved or isolated.

Peer implementations listed below are comparative research inputs only. They do not enter this authority order and cannot establish protocol behavior, validate a CyborgHeart support claim, or replace independent reproduction against Matrix and the pinned Ruma release.

## 2. Matrix specification

### Baseline

- [Matrix Specification v1.19](https://spec.matrix.org/v1.19/)
- [Matrix v1.19 changelog](https://spec.matrix.org/v1.19/changelog/v1.19/)
- [Room Versions](https://spec.matrix.org/v1.19/rooms/)
- [Room Version 12](https://spec.matrix.org/v1.19/rooms/v12/)

### Event and room model

- [Events](https://spec.matrix.org/v1.19/#events)
- [Event graphs](https://spec.matrix.org/v1.19/#event-graphs)
- [Room structure](https://spec.matrix.org/v1.19/#room-structure)
- [Room state](https://spec.matrix.org/v1.19/#room-state)

### Server–Server behavior

- [PDUs](https://spec.matrix.org/v1.19/server-server-api/#pdus)
- [Checks performed on receipt of a PDU](https://spec.matrix.org/v1.19/server-server-api/#checks-performed-on-receipt-of-a-pdu)
- [Auth events selection](https://spec.matrix.org/v1.19/server-server-api/#auth-events-selection)
- [Rejection](https://spec.matrix.org/v1.19/server-server-api/#rejection)
- [Soft failure](https://spec.matrix.org/v1.19/server-server-api/#soft-failure)
- [Policy Servers](https://spec.matrix.org/v1.19/server-server-api/#policy-servers)
- [Server ACLs](https://spec.matrix.org/v1.19/server-server-api/#server-access-control-lists-acls)

### Canonical data and cryptography

- [Signing events](https://spec.matrix.org/v1.19/server-server-api/#signing-events)
- [Content hashes](https://spec.matrix.org/v1.19/server-server-api/#calculating-the-content-hash-for-an-event)
- [Reference hashes](https://spec.matrix.org/v1.19/server-server-api/#calculating-the-reference-hash-for-an-event)
- [Validating event hashes and signatures](https://spec.matrix.org/v1.19/server-server-api/#validating-hashes-and-signatures-on-received-events)
- [Canonical JSON](https://spec.matrix.org/v1.19/appendices/#canonical-json)
- [Signing JSON](https://spec.matrix.org/v1.19/appendices/#signing-json)
- [Unpadded Base64](https://spec.matrix.org/v1.19/appendices/#unpadded-base64)
- [URL-safe unpadded Base64](https://spec.matrix.org/v1.19/appendices/#url-safe-unpadded-base64)

### Room-version-12 rules

- [Event format](https://spec.matrix.org/v1.19/rooms/v12/#event-format)
- [Event IDs](https://spec.matrix.org/v1.19/rooms/v12/#event-ids)
- [Authorization rules](https://spec.matrix.org/v1.19/rooms/v12/#authorisation-rules)
- [Redactions](https://spec.matrix.org/v1.19/rooms/v12/#redactions)
- [Handling redactions](https://spec.matrix.org/v1.19/rooms/v12/#handling-redactions)
- [Rejected events](https://spec.matrix.org/v1.19/rooms/v12/#rejected-events)
- [State resolution](https://spec.matrix.org/v1.19/rooms/v12/#state-resolution)
- [Signing-key validity period](https://spec.matrix.org/v1.19/rooms/v12/#signing-key-validity-period)

## 3. Ruma

### Candidate initial baseline

- [Ruma 0.16.0](https://docs.rs/ruma/0.16.0/ruma/)
- [ruma-events 0.34.0](https://docs.rs/ruma-events/0.34.0/ruma_events/)
- [ruma-state-res 0.17.0](https://docs.rs/ruma-state-res/0.17.0/ruma_state_res/)
- [ruma-signatures 0.21.0](https://docs.rs/ruma-signatures/0.21.0/ruma_signatures/)

### Repository and changelogs

- [Ruma repository](https://github.com/ruma/ruma)
- [Ruma releases](https://github.com/ruma/ruma/releases)
- [ruma-state-res 0.17.0 source](https://github.com/ruma/ruma/tree/ruma-state-res-0.17.0/crates/ruma-state-res)
- [ruma-signatures 0.21.0 source](https://github.com/ruma/ruma/tree/ruma-signatures-0.21.0/crates/ruma-signatures)
- [ruma-events 0.34.0 source](https://github.com/ruma/ruma/tree/ruma-events-0.34.0/crates/ruma-events)
- [ruma-state-res changelog](https://github.com/ruma/ruma/blob/main/crates/ruma-state-res/CHANGELOG.md)
- [ruma-signatures changelog](https://github.com/ruma/ruma/blob/main/crates/ruma-signatures/CHANGELOG.md)

The upstream `main` changelog may document unreleased helpers. CyborgHeart uses only APIs present in the pinned released dependency set unless a reviewed Git dependency is explicitly adopted by ADR.

The first reviewed `Cargo.lock` becomes the operational dependency baseline. Dependency availability does not itself establish CyborgHeart support.

## 4. Comparative Matrix homeserver implementations

These projects are useful for implementation comparison, operational lessons, regression discovery, interoperability investigation, and historical context:

- [Conduit](https://conduit.rs/) — the original Rust homeserver lineage.
- [conduwuit](https://github.com/x86pup/conduwuit) — an archived hard fork of Conduit with substantial implementation divergence.
- [Continuwuity](https://github.com/continuwuity/continuwuity) — an active community continuation of the conduwuit codebase.
- [Tuwunel](https://github.com/matrix-construct/tuwunel) — an active Rust homeserver derived from conduwuit.

These links do not designate architectural dependencies, preferred implementations, compatibility authorities, or sources of normative truth. CyborgHeart may study their code, tests, fixtures, incident history, and operational tradeoffs, but any adopted behavior must be independently justified against the Matrix specification and reproduced against the pinned Ruma surface. Project claims about lineage, succession, completeness, stability, or deployment are treated as claims of those projects rather than CyborgHeart findings.

Do not copy implementation behavior merely because multiple homeservers agree. Shared behavior may still be historical, incomplete, accidental, room-version-specific, or inconsistent with the current normative baseline.

## 5. Rust and Cargo

- [Rust 1.97.0 announcement](https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/)
- [Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo dependency resolver](https://doc.rust-lang.org/cargo/reference/resolver.html)
- [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Cargo lockfile guidance](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [rustdoc book](https://doc.rust-lang.org/rustdoc/)

The repository’s `rust-toolchain.toml`, `Cargo.toml`, and `Cargo.lock` become operational sources of truth after initialization.

## 6. Security, quality, and conformance

- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
- [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)
- [RustSec Advisory Database](https://rustsec.org/)
- [Dependabot](https://docs.github.com/en/code-security/dependabot)
- [GitHub protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches)
- [Matrix Complement](https://github.com/matrix-org/complement)
- [Matrix specification proposals](https://github.com/matrix-org/matrix-spec-proposals)
- [Matrix specification repository](https://github.com/matrix-org/matrix-spec)

Complement is a later end-to-end homeserver gate. It does not replace the pure event-engine fixture corpus. Unstable MSC behavior is outside the initial supported surface unless adopted explicitly by ADR and support update.

## 7. Identity and legal references

- [The Matrix.org Foundation](https://matrix.org/foundation/)
- [Matrix.org Foundation Trademark Policy](https://matrix.org/legal/trademark-policy/)
- [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](https://opensource.org/license/mit)
- [Contributor Covenant 3.0](https://www.contributor-covenant.org/version/3/0/)
- [Creative Commons Attribution-ShareAlike 4.0](https://creativecommons.org/licenses/by-sa/4.0/)

CyborgHeart is an independent ProjectCyborg project. External fixture material retains its original provenance and license terms.

## 8. Canonical foundational documents

| Document | Owns |
|---|---|
| [`MANIFESTO.md`](./MANIFESTO.md) | Enduring principles and choices |
| [`WORKING-THESIS.md`](./WORKING-THESIS.md) | Falsifiable project argument and hypotheses |
| [`WORKING-LEXICON.md`](./WORKING-LEXICON.md) | Current preferred terminology |
| [`ARCHITECTURE.md`](./ARCHITECTURE.md) | Layers, ownership, and dependency direction |
| [`SUPPORTED-SURFACE.md`](./SUPPORTED-SURFACE.md) | Exact current and targeted support claims |
| [`SPECIFICATION-MAP.md`](./SPECIFICATION-MAP.md) | Matrix obligations mapped to Ruma, code, and fixtures |
| [`RUMA-COVERAGE.md`](./RUMA-COVERAGE.md) | Exact dependency APIs, assumptions, and gaps |
| [`EVENT-LIFECYCLE.md`](./EVENT-LIFECYCLE.md) | Stage order, dependencies, and disposition points |
| [`DEPENDENCY-MODEL.md`](./DEPENDENCY-MODEL.md) | Immutable fact identities, snapshots, and resumability |
| [`DECISION-CODES.md`](./DECISION-CODES.md) | Stable machine-readable outcome taxonomy |
| [`REFERENCES.md`](./REFERENCES.md) | External source index and documentation boundary |

## 9. Completion and document boundary

The foundational set is complete. No additional foundational document is currently required.

Do not create broad overlapping files such as:

```text
PRINCIPLES.md
VISION.md
PHILOSOPHY.md
DESIGN.md
OVERVIEW.md
GLOSSARY.md
ROADMAP.md
STATUS.md
COMPATIBILITY.md
TESTING-STRATEGY.md
SECURITY-ARCHITECTURE.md
```

Their responsibilities already belong to the manifesto, working thesis, working lexicon, architecture, supported surface, code and fixture documentation, operational security policy, or issue tracker.

Further architectural decisions belong in focused ADRs under `../adr/` only when they materially change layer ownership, a public contract, protocol support, a Ruma workaround, or a significant infrastructure dependency.

## 10. Supporting repository documents

The public repository root owns `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `NOTICE.md`, and the dual-license texts. They govern the implementation repository but are not additions to the foundational architecture sequence. Private project-operations material may link to these files but must not maintain competing canonical copies.

## 11. Maintenance

Update this index when:

- the Matrix baseline or target room version changes;
- Ruma or Rust baselines change;
- a required external source is added or retired;
- a comparative implementation reference becomes materially stale or misleading;
- the canonical document set changes.

When an external source changes a compatibility claim, update in the same pull request:

- `REFERENCES.md`;
- `SPECIFICATION-MAP.md`;
- `RUMA-COVERAGE.md`;
- `SUPPORTED-SURFACE.md`;
- affected lifecycle, dependency, and decision-code documents;
- affected fixtures and code.

---

CyborgHeart is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

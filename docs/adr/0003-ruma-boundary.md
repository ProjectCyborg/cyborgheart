# ADR 0003: Compose Ruma rather than reimplement Matrix primitives

**Status:** Accepted
**Date:** 2026-07-12

## Context

Ruma already provides Rust implementations and types for substantial Matrix protocol behavior, including canonical JSON, identifiers, room-version rules, event hashing and signing, signature verification, event authorization, and state resolution.

CyborgHeart’s identified gap is not the absence of those primitives. It is the absence of a reusable deterministic application lifecycle that sequences them, requests missing facts, preserves outcome distinctions, bounds work, and returns storage-neutral consequences.

Reimplementing Ruma-owned behavior would enlarge the correctness and security burden without advancing that core thesis.

At the same time, Ruma is not a complete event-application runtime. Some Matrix v1.19 behavior may be newer than the pinned release or require host orchestration beyond one Ruma call.

## Decision

CyborgHeart will use released Ruma crates for protocol primitives and core algorithms where the pinned release provides the required behavior.

Ruma owns, subject to version review:

- Matrix identifiers and room-version rules;
- canonical JSON representation;
- event and reference hashing;
- event signing and signature verification;
- required server-signature discovery;
- state-independent authorization;
- state-dependent authorization;
- state resolution.

CyborgHeart owns:

- lifecycle sequencing;
- supported-version admission;
- explicit dependency discovery;
- trust-stage progression;
- work budgeting;
- stable decision codes;
- structured evidence;
- semantic graph and state consequences;
- orchestration for normative behavior not exposed as one released Ruma operation.

The first reviewed `Cargo.lock` in the implementation repository becomes the operational dependency baseline. Documentation must refer to exact released crate versions and must not assume unreleased `main` behavior.

## Consequences

### Positive

- CyborgHeart concentrates on its distinct responsibility;
- cryptographic and protocol primitives receive upstream ecosystem review;
- Matrix rule updates can often be adopted through controlled dependency upgrades;
- local code remains smaller and easier to audit;
- upstream defects can be reproduced and contributed back.

### Negative

- CyborgHeart inherits Ruma API changes and release timing;
- Ruma error strings and internal structures cannot become stable CyborgHeart APIs;
- Matrix baseline changes may temporarily exceed the released Ruma surface;
- some behavior requires adapters or explicit orchestration around Ruma.

### Required safeguards

- `RUMA-COVERAGE.md` records exact APIs, assumptions, versions, and gaps;
- `SPECIFICATION-MAP.md` remains authoritative for normative obligations;
- Ruma errors are mapped to CyborgHeart-owned stable codes;
- dependency updates require protocol review and fixture execution;
- the initial umbrella feature set is exactly `events`, `signatures`, and `state-res`;
- `events` is enabled to expose public event types required by the `ruma-state-res::Event` adapter, not to add strict validation to untrusted PDUs;
- no `full`, API, unstable MSC, or compatibility feature is enabled without an explicit need;
- untrusted federated events use lazy server-oriented event access rather than strict client event deserialization.

## Local replacement policy

A local implementation of Ruma-owned behavior is permitted only when:

1. a concrete defect or missing released capability is reproduced;
2. expected Matrix behavior is cited;
3. a failing regression fixture exists;
4. the workaround has a narrow boundary;
5. an upstream issue or contribution is considered;
6. removal remains possible after an upstream release;
7. the decision is recorded in an ADR.

A local adapter that sequences or translates Ruma behavior is not considered a reimplementation when the underlying algorithm remains Ruma-owned.

## Alternatives considered

### Reimplement all protocol algorithms

Rejected because it duplicates mature work and expands the cryptographic and interoperability risk surface.

### Treat Ruma as the complete server core

Rejected because Ruma deliberately supplies primitives and algorithms rather than CyborgHeart’s full dependency, lifecycle, evidence, budget, and durable-execution model.

### Track Ruma `main`

Rejected for the baseline because unreleased behavior is not a stable dependency contract.

## Reconsider when

This boundary should be reviewed if:

- a required normative capability remains unavailable in released Ruma for a sustained period;
- Ruma’s public API can no longer support deterministic or bounded orchestration;
- a security or correctness issue cannot be isolated safely;
- upstream and CyborgHeart lifecycle responsibilities materially overlap.

The default response should still be upstream collaboration or a removable adapter, not a broad fork.

## Related documents

- [`../foundational/RUMA-COVERAGE.md`](../foundational/RUMA-COVERAGE.md)
- [`../foundational/SPECIFICATION-MAP.md`](../foundational/SPECIFICATION-MAP.md)
- [`../foundational/EVENT-LIFECYCLE.md`](../foundational/EVENT-LIFECYCLE.md)

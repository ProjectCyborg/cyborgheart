# Reviewing CyborgHeart Changes

CyborgHeart review starts with ownership and evidence, not diff size.

This guide is for reviewers of changes to the public `cyborgheart` repository. It complements [`CONTRIBUTING.md`](./CONTRIBUTING.md), which remains the author-facing policy.

## First fifteen minutes

1. Confirm the pull request targets `development`, unless it is the repository's `development` branch promoting to `main`.
2. Read the PR description, explicit non-goals, and linked issue.
3. Inspect the changed-file categories before reading implementation details.
4. Classify the maximum review risk.
5. Identify the public documents and Matrix or Ruma sources that govern the change.
6. Run or inspect the quick gate.
7. Look first for boundary, compatibility, support-claim, and evidence failures.
8. Decide whether the change is reviewable as one unit.

If the scope cannot be summarized in one sentence, request clarification or decomposition before detailed review.

## Risk classes

Use the highest applicable class.

| Risk | Examples | Minimum review depth |
|---|---|---|
| Low | prose, links, comments, formatting | accuracy, ownership, unchanged support claims |
| Rust/internal | private helpers, refactors | behavior preservation, lints, tests, determinism |
| Fixture/testkit | schemas, loaders, fixtures | provenance, canonical form, expected outcome, regression value |
| Public API | exported types, constructors, stable codes | compatibility, invariants, serialization, migration impact |
| Protocol | Matrix/Ruma behavior, dispositions, state/auth/redaction | normative source, lifecycle order, dependencies, executable evidence |
| Architecture | ownership, crate boundary, I/O/runtime assumptions | foundational documents, ADR need, dependency direction |
| Security/release | crypto, dependencies, workflows, tags, MSRV/platform | threat surface, permissions, pinning, full verification, recovery |

Small diffs can still be high risk.

## Authority set

Always read:

- the PR description and linked issue;
- the relevant code, fixtures, tests, or documents;
- [`SUPPORTED-SURFACE.md`](./docs/foundational/SUPPORTED-SURFACE.md) when behavior or claims may change.

Add these as needed:

- [`ARCHITECTURE.md`](./docs/foundational/ARCHITECTURE.md) for ownership and dependency direction;
- [`SPECIFICATION-MAP.md`](./docs/foundational/SPECIFICATION-MAP.md) for Matrix obligations;
- [`RUMA-COVERAGE.md`](./docs/foundational/RUMA-COVERAGE.md) for Ruma APIs and gaps;
- [`EVENT-LIFECYCLE.md`](./docs/foundational/EVENT-LIFECYCLE.md) for stage order;
- [`DEPENDENCY-MODEL.md`](./docs/foundational/DEPENDENCY-MODEL.md) for facts, snapshots, and resumability;
- [`DECISION-CODES.md`](./docs/foundational/DECISION-CODES.md) for stable outcomes;
- applicable ADRs under [`docs/adr/`](./docs/adr/);
- [`SECURITY.md`](./SECURITY.md) for sensitive findings.

Private project-ops material is not required context for public review.

## Review order

Review semantics before style:

1. responsibility and layer ownership;
2. normative or public-contract meaning;
3. missing-fact and bounded-halt behavior;
4. decision and consequence semantics;
5. determinism and resource bounds;
6. public API and serialization compatibility;
7. fixture and test evidence;
8. dependency and feature impact;
9. diagnostics, naming, and style.

For Rust changes, check that no `unsafe` code entered, public exports are necessary, output ordering is deterministic, and storage, network, async, clock, process, or product-policy concerns did not enter the pure event engine.

For Matrix or Ruma-sensitive work, verify the exact Matrix v1.19 and room-version-12 source, the current supported surface, lifecycle order, Ruma APIs in the pinned release, and executable evidence. Peer homeserver behavior is comparative research only, not authority.

## Evidence standard

A normative behavior change requires evidence that:

- fails before the change and passes after it;
- records provenance and the normative source;
- expresses expected dependency, decision, and consequence;
- covers malformed or adversarial edges where relevant;
- remains deterministic across supported platforms.

Tests that only execute new code are not enough for protocol-sensitive behavior.

## Validation

Use the quick gate for ordinary integration review:

```bash
./scripts/verify.sh --quick
```

Use the full verifier for public APIs, stable codes, dependency or Ruma changes, support claims, fixture schemas, release or CI behavior, security-sensitive changes, and final promotion confidence:

```bash
./scripts/verify.sh
```

CI success is necessary but does not prove Matrix correctness.

## Findings

Report findings in descending severity.

- **Blocker** - incorrect protocol meaning, security flaw, violated invariant, data or compatibility break, missing required evidence.
- **High** - likely correctness or architecture failure in realistic use.
- **Medium** - bounded defect, unclear contract, insufficient test coverage, or maintainability risk.
- **Low** - local clarity, ergonomics, or cleanup issue.
- **Question** - clarification needed before judging the change.

Each finding should include the exact location, observed behavior or omission, why it matters, the governing source or invariant, and the smallest acceptable correction or evidence.

## Approval

Approve only when:

- scope and ownership are clear;
- authoritative obligations are satisfied;
- public and serialized compatibility is understood;
- evidence matches the claimed behavior;
- support claims remain exact;
- required checks pass;
- no unresolved high-risk finding remains;
- the diff contains no hidden follow-on requirement.

Use request-changes for correctness, safety, architecture, evidence, or compatibility blockers. Use comments for non-blocking improvements.

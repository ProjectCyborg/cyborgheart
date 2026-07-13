# Contributing to CyborgHeart

CyborgHeart is establishing a correctness-first Matrix server foundation. Contributions are welcome when they preserve explicit protocol claims, layer ownership, and durable evidence.

## Before contributing

Read:

1. [`MANIFESTO.md`](./docs/foundational/MANIFESTO.md)
2. [`ARCHITECTURE.md`](./docs/foundational/ARCHITECTURE.md)
3. [`SUPPORTED-SURFACE.md`](./docs/foundational/SUPPORTED-SURFACE.md)
4. [`SPECIFICATION-MAP.md`](./docs/foundational/SPECIFICATION-MAP.md)
5. [`RUMA-COVERAGE.md`](./docs/foundational/RUMA-COVERAGE.md)

Security-sensitive findings must follow [`SECURITY.md`](./SECURITY.md), not a public issue.

## Contribution principles

A contribution should:

- solve one clearly owned problem;
- preserve the downward dependency direction;
- keep protocol meaning independent from storage, transport, and product policy;
- distinguish missing facts, bounded halts, Matrix dispositions, and durable-runtime outcomes;
- cite the applicable Matrix obligation;
- add executable evidence for normative behavior;
- avoid widening compatibility claims without the promotion gate;
- prefer a narrow public contract over exposed lifecycle internals.

## Issues

A protocol or implementation issue should include, when applicable:

- Matrix specification section and specification-map IDs;
- observed and expected behavior;
- pinned Ruma crate versions and API involved;
- minimized canonical event, graph, or fixture;
- expected dependency, decision code, and consequence;
- budget dimensions affected;
- current support claim affected.

Never include real access tokens, private signing keys, secrets, or non-public user data.

## Labels

CyborgHeart uses a small controlled label vocabulary:

- Type: `type: bug`, `type: documentation`, `type: proposal`, `type: test-fixture`, `type: maintenance`
- Area: `area: event-application`, `area: testkit`, `area: fixtures`, `area: ruma`, `area: docs`, `area: ci-release`
- Risk: `risk: low`, `risk: protocol`, `risk: public-api`, `risk: security`, `risk: architecture`
- State: `needs: reproduction`, `needs: specification`, `needs: decision`, `needs: tests`, `blocked`, `ready`
- Contributor entry: `good first issue`, `help wanted`, `mentored`

`good first issue` is reserved for bounded work with exact acceptance criteria, validation commands, explicit non-goals, and no hidden protocol decision.

## Pull requests

Keep pull requests focused. The description should state:

- responsibility changed;
- specification-map IDs served;
- Ruma APIs relied upon;
- dependency identities added or changed;
- decision codes reachable;
- budget dimensions affected;
- fixtures and properties added;
- support-state changes;
- explicit non-goals.

A normative behavior change requires a fixture or property test that fails before the change and passes after it.

Reviewers use [`REVIEWING.md`](./REVIEWING.md) to classify risk, inspect evidence, and decide whether a change is ready.

New contributors should start with a bounded issue that names expected files or search areas, acceptance criteria, validation commands, explicit non-goals, and the relevant architecture constraints. Do not treat protocol-sensitive work as a first issue unless a maintainer has already supplied the specification mapping and evidence target.

## Required checks

Once the Rust workspace exists, the default pull-request checks are:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo deny check
```

The implementation repository may use Rust 1.97's Cargo warning control in CI, but the commands above remain the readable baseline. Fuzz, property, platform, and end-to-end checks are added only where the affected surface requires them.

## Branch validation standard

`main` is the protected correctness baseline. Pushes and pull requests targeting `main` run the full validation suite:

- format;
- clippy with warnings denied;
- all workspace tests;
- documentation with warnings denied;
- dependency policy through `cargo-deny`;
- Windows and macOS smoke tests.

`development` is the long-lived integration branch. Pushes and pull requests targeting `development` run a faster Linux gate:

- format;
- clippy with warnings denied;
- all workspace tests.

The development gate must stay fast enough for frequent integration but strict enough that obvious Rust, lint, and fixture failures do not accumulate. Changes that affect dependencies, public API shape, support claims, release behavior, documentation contracts, or platform assumptions should still run the full verifier locally before merge.

Normal changes must be proposed as pull requests into `development`. Promotion to `main` must be a pull request from the repository's `development` branch into `main`; direct feature-branch pull requests to `main` are rejected by CI policy.

The scheduled security workflow runs dependency policy independently so advisories are still checked even when no `main` change is in flight.

## Protocol-support changes

A support change normally updates together:

- code;
- conformance, malformed, adversarial, and regression fixtures;
- `SUPPORTED-SURFACE.md`;
- `SPECIFICATION-MAP.md`;
- `RUMA-COVERAGE.md`;
- `EVENT-LIFECYCLE.md`;
- `DEPENDENCY-MODEL.md`;
- `DECISION-CODES.md`.

Do not create a new broad document when an existing canonical document already owns the responsibility.

## Ruma dependency changes

A Ruma update requires:

- exact crate-version and feature review;
- changelog and relevant source review for every used crate;
- full conformance, adversarial, property, and regression execution;
- review of changed errors, ordering, room-version rules, and serialization behavior;
- an updated `RUMA-COVERAGE.md` and specification map;
- no automatic widening of `SUPPORTED-SURFACE.md`.

The initial umbrella feature set is exactly `events`, `signatures`, and `state-res`. The `events` feature exists to support public event types required by the state-resolution adapter; it does not authorize strict typed deserialization of untrusted PDUs.

Do not enable umbrella `full`, API, unstable MSC, or compatibility features without a fixture-backed requirement and, when durable, an ADR.

## Public API changes

Treat public types and stable decision codes as promises.

A public API change should explain:

- why the current contract is insufficient;
- why the new surface expresses stable domain meaning;
- migration and serialization impact;
- effect on fixtures or future durable records;
- whether an ADR is required.

## Architecture decisions

Create an ADR only for material decisions that alter layer ownership, protocol support, public contracts, Ruma workarounds, or significant infrastructure. Follow [`docs/adr/README.md`](./docs/adr/README.md).

## Licensing

Unless explicitly stated otherwise, contributions intentionally submitted for inclusion are provided under:

```text
MIT OR Apache-2.0
```

By submitting a contribution, you represent that you have the right to provide it under those terms.

Changes to files with a different stated license—currently `CODE_OF_CONDUCT.md` under CC BY-SA 4.0—or to third-party fixtures and excerpts must comply with that file or source material's license and attribution requirements.

## Conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md).

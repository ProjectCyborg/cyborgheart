# CyborgHeart

CyborgHeart is a correctness-first, modular Matrix server foundation written in Rust.

The repository begins with a deterministic room-event application engine and its conformance testkit. Later transaction, command, capability, SDK, and homeserver layers may be added only after the lower contracts are proven.

## Current status

```text
Matrix support claim: none
Normative baseline: Matrix v1.19
Initial target: room version 12 only
First implementation boundary: pure room-event application
Initial crates:
  cyborg-heart-event-application
  cyborg-heart-event-testkit
```

This repository is currently at contract-seeding stage. The Rust workspace is initialized, public domain vocabulary exists, and fixture validation is available; no Matrix room version or event-application lifecycle is supported yet.

## Repository model

CyborgHeart is intended to remain one public Git repository and one virtual Cargo workspace during its early development.

```text
cyborgheart/
├── README.md
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── NOTICE.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── crates/
│   ├── event-application/
│   └── event-testkit/
├── fixtures/
│   ├── README.md
│   ├── schema/
│   └── seed/
├── docs/
│   ├── foundational/
│   └── adr/
└── .github/
```

The private sibling repository `cyborgheart-project-ops` may coordinate plans, handoffs, delegation, and internal review. It is not required to understand, build, test, or release this repository and is never the authority for public implementation behavior.

## Reading order

1. [`MANIFESTO.md`](./docs/foundational/MANIFESTO.md)
2. [`WORKING-THESIS.md`](./docs/foundational/WORKING-THESIS.md)
3. [`WORKING-LEXICON.md`](./docs/foundational/WORKING-LEXICON.md)
4. [`ARCHITECTURE.md`](./docs/foundational/ARCHITECTURE.md)
5. [`SUPPORTED-SURFACE.md`](./docs/foundational/SUPPORTED-SURFACE.md)
6. [`EVENT-LIFECYCLE.md`](./docs/foundational/EVENT-LIFECYCLE.md)
7. [`DEPENDENCY-MODEL.md`](./docs/foundational/DEPENDENCY-MODEL.md)
8. [`DECISION-CODES.md`](./docs/foundational/DECISION-CODES.md)
9. [`SPECIFICATION-MAP.md`](./docs/foundational/SPECIFICATION-MAP.md)
10. [`RUMA-COVERAGE.md`](./docs/foundational/RUMA-COVERAGE.md)
11. [`REFERENCES.md`](./docs/foundational/REFERENCES.md)

The foundational set is intentionally closed. Further detail belongs in code documentation, fixtures, focused ADRs, and issues.

## Fixtures

[`fixtures/`](./fixtures/) contains the implementation-neutral schema and the first reviewed seed scenarios. Fixtures are executable protocol claims and become mandatory semantic tests when their declared lifecycle stage exists.

Validate the fixture contract through the testkit:

```bash
cargo test -p cyborg-heart-event-testkit --locked
```

## Validation

Run the local verification script:

```bash
./scripts/verify.sh
```

For the faster development-branch gate:

```bash
./scripts/verify.sh --quick
```

The CI command set for the first implementation boundary is:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
cargo deny check
```

`main` runs all gates plus Windows and macOS smoke checks. `development` runs the quick Linux gate. See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for the branch validation standard.

Local shells should use the pinned Rust toolchain from [`rust-toolchain.toml`](./rust-toolchain.toml). If `cargo --version` does not report Rust 1.97.0, run the same commands through `rustup run 1.97.0 cargo ...`.

`./scripts/verify.sh` automatically installs the pinned `cargo-deny` version when it is missing or mismatched.

## Architecture decisions

[`docs/adr/`](./docs/adr/) records decisions that govern this repository. An ADR belongs here when it changes this repository's code, public contract, compatibility surface, dependency boundary, or release behavior.

## Contribution and security

- [`CONTRIBUTING.md`](./CONTRIBUTING.md)
- [`REVIEWING.md`](./REVIEWING.md)
- [`SECURITY.md`](./SECURITY.md)
- [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md)
- [`NOTICE.md`](./NOTICE.md)

## Licensing

Except where a file or imported artifact states otherwise, CyborgHeart is available under your choice of:

```text
MIT OR Apache-2.0
```

See [`LICENSE-MIT`](./LICENSE-MIT), [`LICENSE-APACHE`](./LICENSE-APACHE), and [`NOTICE.md`](./NOTICE.md).

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

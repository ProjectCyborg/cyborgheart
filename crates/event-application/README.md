# cyborg-heart-event-application

Domain contract types for CyborgHeart room-event application.

This crate owns:

- supported room-version representation;
- canonical candidate-event domain types;
- immutable non-empty dependency request types;
- evaluation budgets and work reports;
- Matrix disposition/result vocabulary;
- decision evidence and typed semantic consequence vocabulary.

This crate does not own networking, persistence, duplicate detection, clocks, randomness, async execution, homeserver endpoints, or transaction/command runtimes.

The first implementation slice intentionally does not export an `evaluate` function. Public types are present so fixtures and future lifecycle work can preserve the distinctions documented in:

- [`ARCHITECTURE.md`](../../docs/foundational/ARCHITECTURE.md)
- [`SUPPORTED-SURFACE.md`](../../docs/foundational/SUPPORTED-SURFACE.md)
- [`EVENT-LIFECYCLE.md`](../../docs/foundational/EVENT-LIFECYCLE.md)
- [`DEPENDENCY-MODEL.md`](../../docs/foundational/DEPENDENCY-MODEL.md)
- [`DECISION-CODES.md`](../../docs/foundational/DECISION-CODES.md)

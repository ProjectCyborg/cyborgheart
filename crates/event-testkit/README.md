# cyborg-heart-event-testkit

Fixture infrastructure for CyborgHeart event application.

This crate owns:

- fixture JSON parsing;
- JSON Schema Draft 2020-12 validation;
- deterministic fixture discovery and ordering;
- duplicate fixture-ID detection;
- typed expected-result metadata;
- canonical JSON input adaptation for fixture candidates;
- staged fixture classification.

This crate does not own Matrix protocol decisions, networking, persistence, async execution, or private lifecycle-stage access to the event-application crate.

Canonical fixture assets live in [`fixtures/`](../../fixtures/). Fixture rules are documented in [`fixtures/README.md`](../../fixtures/README.md), with stable outcome vocabulary in [`DECISION-CODES.md`](../../docs/foundational/DECISION-CODES.md).

# ADR 0005: Correct pre-release halt and fixture contracts

**Status:** Accepted  
**Date:** 2026-07-13

## Context

The contract-seeding repository exposed two states that contradicted its own architecture:

1. `Halt::new` could create `halt.budget_exceeded` when consumed work was equal to or below the configured limit.
2. Policy Server recommendation fixtures accepted a host-asserted `valid` or `invalid` state even though the event engine must validate supplied signature material itself.

The typed fixture loader also discarded schema-required input, fact, provenance, option, detail, and expectation fields after schema validation. No CyborgHeart crate or fixture schema has been released or published, and Matrix support remains `none`.

## Decision

- A budget-exceeded halt is constructed only through a checked constructor that requires `consumed > limit`.
- Invalid halt construction returns a typed error carrying the dimension, limit, and consumed amount.
- A Policy Server recommendation fact carries either signature material for engine validation or a terminal acquisition-unavailable outcome.
- Host assertions that recommendation material is already valid or invalid are rejected by schema.
- The event testkit retains every schema-required fixture field in typed structures.
- Fixture schema version 1 is corrected in place because it has no released or supported consumer contract.

## Consequences

- Contradictory budget halts are unrepresentable through the public constructor.
- The host remains responsible for acquisition, while the event engine remains responsible for cryptographic meaning.
- Future Stage 1 work can construct complete explicit inputs and fact snapshots without reparsing discarded fixture JSON.
- Any external experiment using the unreleased earlier schema must update before relying on the repository.

## Alternatives considered

### Keep host-provided validity states

Rejected because it moves protocol validation authority outside the event engine and permits inconsistent replay evidence.

### Introduce fixture schema version 2 immediately

Rejected because there is no release, published crate, compatibility claim, or supported consumer requiring migration machinery. Versioning an unshipped mistake would add process without preserving a real contract.

### Leave `Halt::new` unrestricted and rely on callers

Rejected because the public type would continue to represent a state its stable machine code says cannot exist.

## Reconsider when

- fixture schema version 1 has an external supported consumer;
- Policy Server rules change in a later Matrix baseline;
- a broader typed budget-exhaustion model replaces the initial halt type.

## Related documents

- [`../foundational/EVENT-LIFECYCLE.md`](../foundational/EVENT-LIFECYCLE.md)
- [`../foundational/DEPENDENCY-MODEL.md`](../foundational/DEPENDENCY-MODEL.md)
- [`../foundational/DECISION-CODES.md`](../foundational/DECISION-CODES.md)
- [`../../fixtures/README.md`](../../fixtures/README.md)
- [`0004-structural-public-contract-invariants.md`](./0004-structural-public-contract-invariants.md)

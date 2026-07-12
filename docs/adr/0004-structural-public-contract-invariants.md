# ADR 0004: Enforce structural public contract invariants

**Status:** Accepted
**Date:** 2026-07-12

## Context

The first generated public event-application model exposed useful domain names, but some constructors could assemble states that the foundational lifecycle treats as impossible.

Examples included admitted decisions without semantic consequences, arbitrary consequence-code bags with contradictory state or graph effects, caller-supplied terminal stages that could disagree with decision codes, and dependency results with no requests or no evidence.

No public release had been made, so preserving those shapes would create compatibility burden around an incorrect seed API.

## Decision

The public contract will make the closed structural invariants unrepresentable:

- use one admitted decision envelope containing outcome, evidence, and semantic consequences;
- expose specialized accepted, rejected, and soft-failed constructors instead of arbitrary admitted construction;
- represent consequences as typed historical-state, forward-extremity, and redaction dimensions;
- derive disposition, visibility, and terminal lifecycle stage from outcome and code;
- require dependency needs, request sets, and individual requests to be non-empty by construction;
- require dependency requests to be homogeneous by stable dependency code;
- expose code-family registries and make `known_decision_code()` delegate to those registries;
- mark public enums non-exhaustive where future protocol growth should not freeze downstream matching.

## Consequences

### Positive

- rejected and soft-failed admitted decisions cannot omit their semantic consequences;
- complete decision stages cannot contradict their stable machine codes;
- stable consequence-code projection is deterministic and dimensionally exclusive;
- incomplete dependency results carry evidence and at least one concrete request;
- future serialization can follow one source of truth for codes, stages, and visibility.

### Negative

- the seed API changes before the initial commit;
- constructors are more specialized;
- tests and documentation must describe structural invariants separately from future evaluator behavior.

## Alternatives considered

### Runtime validation on every getter

Rejected because invalid public values would still exist and could leak through comparison, serialization, or tests before validation happened.

### Arbitrary constructors with documentation warnings

Rejected because comments are a weak substitute for type-level invariants in a security-sensitive protocol boundary.

### Retain three admitted wrapper structs

Rejected because accepted, rejected, and soft-failed admitted decisions share outcome, evidence, and consequence semantics. Separate wrappers duplicated the common fields and made omission easier.

### Use `Vec` or `BTreeSet` plus comments for non-empty values

Rejected because the API can require a first element and normalize ordering without adding dependencies.

## Reconsider when

Reconsider only if a later Matrix lifecycle stage proves that one of these invariants is not actually closed, or if a versioned public wire format requires a different compatibility strategy.

## Related documents

- [`../foundational/ARCHITECTURE.md`](../foundational/ARCHITECTURE.md)
- [`../foundational/EVENT-LIFECYCLE.md`](../foundational/EVENT-LIFECYCLE.md)
- [`../foundational/DEPENDENCY-MODEL.md`](../foundational/DEPENDENCY-MODEL.md)
- [`../foundational/DECISION-CODES.md`](../foundational/DECISION-CODES.md)

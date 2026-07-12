# ADR 0002: Support room version 12 first and exclusively

**Status:** Accepted
**Date:** 2026-07-12

## Context

Matrix room versions are immutable bundles of rules. They affect event format, identifiers, create-event semantics, authorization, redaction, signing, and state resolution.

Ruma can represent and implement behavior for more than one room version. That does not make every represented version supported by CyborgHeart.

Attempting several room versions before one complete lifecycle is proven would multiply fixtures and branches while leaving the core architecture unvalidated.

## Decision

The initial event-application public contract admits only:

```text
SupportedRoomVersion::V12
```

Arbitrary `RoomVersionId` values are not accepted as supported input.

An unsupported room version fails at the capability boundary. It is not classified as a Matrix `Dropped` event because CyborgHeart has not evaluated that room version’s rules.

Room version 12 is selected because it provides a concrete modern surface that exercises:

- canonical JSON;
- reference-hash event IDs;
- room ID derivation from the create event;
- current authorization behavior;
- redaction rules;
- room-version-12 state-resolution changes, including the conflicted state subgraph and empty initial iterative-auth state.

## Consequences

### Positive

- every support claim is precise;
- fixture scope remains bounded;
- room-version behavior cannot silently fall back to a default;
- the architecture must prove a complete lifecycle rather than broad enum coverage;
- adding a later room version becomes an explicit compatibility event.

### Negative

- the initial engine cannot process existing rooms on earlier versions;
- end-to-end client compatibility is deliberately limited;
- some Ruma capabilities remain unused despite being available;
- a future multi-version public API will require deliberate extension.

### Required safeguards

- `SUPPORTED-SURFACE.md` must continue to state that no room version is supported until the promotion gate passes;
- fixtures must identify Matrix specification and room version explicitly;
- no generic “known room version” path may bypass the supported enum;
- every new room version requires an ADR, specification-map additions, Ruma review, and conformance corpus.

## Alternatives considered

### Support all Ruma-known room versions

Rejected because representability is not evidence of complete CyborgHeart support.

### Begin with the oldest broadly deployed room version

Rejected because it would prioritize compatibility breadth before validating the architecture’s modern lifecycle and state-resolution boundary.

### Make room version configurable through arbitrary rule objects

Rejected for the initial public API. Custom rules may be useful to Ruma consumers, but CyborgHeart’s support contract must remain enumerable and testable.

## Reconsider when

Add another room version only after:

- the complete room-version-12 target is supported;
- lifecycle and fixture abstractions have proven stable;
- the new version has a concrete consumer need;
- all differing normative branches are identified;
- the additional support can be stated without weakening existing claims.

## Related documents

- [`../foundational/SUPPORTED-SURFACE.md`](../foundational/SUPPORTED-SURFACE.md)
- [`../foundational/SPECIFICATION-MAP.md`](../foundational/SPECIFICATION-MAP.md)
- [`../foundational/ARCHITECTURE.md`](../foundational/ARCHITECTURE.md)

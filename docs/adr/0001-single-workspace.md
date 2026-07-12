# ADR 0001: Begin in one Git repository and one Cargo workspace

**Status:** Accepted
**Date:** 2026-07-12

## Context

CyborgHeart is intended to grow from a pure event-application engine into durable runtimes, server capabilities, an SDK, and homeserver products.

Those layers may eventually have independent consumers, release schedules, ownership, or security requirements. Splitting them into repositories before their contracts exist would create coordination and versioning overhead while making boundary changes harder.

A single repository does not require a monolithic crate or a single runtime process. Cargo crates can enforce compile-time dependency direction inside one workspace.

## Decision

The initial implementation will use:

```text
one Git repository
one virtual Cargo workspace
separate crates for proven architectural boundaries
one shared Cargo.lock
one CI and security baseline
```

The first crates are:

```text
cyborg-heart-event-application
cyborg-heart-event-testkit
```

Future crates are added only when their responsibility is required and the lower contract is stable enough to consume.

No repository split is scheduled.

## Consequences

### Positive

- protocol and fixture changes can be reviewed atomically;
- dependency versions remain aligned through one lockfile;
- cross-crate refactoring is cheap while contracts are young;
- CI, linting, security policy, and licensing remain consistent;
- architectural boundaries are expressed through crates rather than repository ceremony.

### Negative

- repository ownership cannot initially be delegated independently;
- all crates share one integration cadence;
- maintainers must actively prevent convenience dependencies from violating layer direction;
- future extraction may require release and history work.

### Required safeguards

- lower crates never depend on upper crates;
- no circular dependency is hidden behind Cargo features;
- each crate exposes a deliberately narrow API;
- the reference homeserver consumes public contracts rather than private workspace internals;
- repository convenience is not treated as permission for architectural coupling.

## Alternatives considered

### Separate repositories from the beginning

Rejected because there are no stable release boundaries or independent consumers yet. It would optimize for hypothetical organizational scale rather than present correctness.

### One crate for the whole server

Rejected because event application, durable execution, commands, and capabilities own different authority and must remain separable.

### Multiple repositories with git submodules or path pins

Rejected because it adds coordination overhead without improving a proven boundary.

## Reconsider when

A repository split may be justified when one or more of these are real:

- independent external consumers;
- stable versioned interfaces;
- independent release cadence;
- separate maintainership or security stewardship;
- a different implementation language or build system;
- material repository-scale or access-control problems.

Repository separation must not be confused with process separation.

## Related documents

- [`../foundational/ARCHITECTURE.md`](../foundational/ARCHITECTURE.md)
- [`../foundational/MANIFESTO.md`](../foundational/MANIFESTO.md)

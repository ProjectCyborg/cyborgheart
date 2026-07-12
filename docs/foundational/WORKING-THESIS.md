# CyborgHeart Working Thesis

**Status:** Working thesis — falsifiable and expected to evolve
**Parent project:** ProjectCyborg
**Protocol:** Matrix
**Implementation language:** Rust
**Primary protocol dependency:** Ruma
**Protocol baseline:** Matrix v1.19, room version 12
**Last reviewed:** 2026-07-12

---

## 1. Thesis

CyborgHeart is founded on the following belief:

> The most valuable missing building block for a new Matrix server stack is not another protocol type library, transport layer, database schema, or complete homeserver. It is a deterministic, storage-neutral event-application runtime that turns Matrix and Ruma primitives into a complete room-event lifecycle.

If that layer can be made correct, bounded, inspectable, and reusable, it can become the stable core beneath:

- durable transaction processing;
- local command construction;
- server capabilities;
- a composable homeserver SDK;
- multiple homeserver products.

The project therefore begins with the smallest layer that can establish trustworthy protocol meaning before infrastructure is introduced.

---

## 2. The problem

Matrix homeservers must do more than parse events and call isolated algorithms.

For each candidate room event, a server must coordinate a lifecycle that includes:

- format validation;
- event ID, hash, and signature handling;
- dependency discovery;
- authorization against the correct contexts;
- historical state reconstruction;
- current-state authorization;
- Policy Server recommendation validation when enabled;
- state resolution;
- distinction between dropped, rejected, soft-failed, and accepted events;
- graph and state consequences;
- bounded work under hostile input.

The Matrix specification defines the required behavior. Ruma provides many of the essential Rust primitives and algorithms.

What remains is application logic: the sequencing, dependency model, trust progression, outcome model, evidence, and semantic consequences that turn those primitives into one complete decision.

In existing homeservers, this work is commonly embedded inside a larger implementation that also owns persistence, networking, federation, caching, retries, and product policy. That may be appropriate for the homeserver, but it makes the protocol lifecycle difficult to isolate, test, reuse, or reason about independently.

CyborgHeart treats that lifecycle as a first-class component.

---

## 3. The gap

The gap is not a lack of Matrix libraries.

Ruma already provides:

- identifiers and protocol types;
- canonical JSON;
- event hashing and signing;
- signature verification;
- room-version rule data;
- event authorization algorithms;
- state resolution;
- API request and response models.

CyborgHeart should not reproduce those capabilities.

The gap is between:

```text
correct protocol primitives and algorithms
```

and:

```text
a complete homeserver that applies events durably
```

That gap contains the logic required to answer:

- What facts are still missing?
- Which trust stage has the event reached?
- Which representation should be authorized?
- Is evaluation incomplete, invalid, rejected, soft-failed, or accepted?
- What state existed before the event?
- What state and graph effects follow?
- How much work was consumed?
- Can the same decision be reproduced from the same facts?

CyborgHeart’s first thesis is that this boundary can be defined independently of transport implementation and storage, while accepting explicit provenance and protocol-required external facts where Matrix makes them relevant.

Its second thesis is that doing so creates more value than beginning with another complete homeserver.

---

## 4. Why this boundary matters

A complete homeserver is operationally useful, but it is a poor place to discover foundational semantics.

When protocol meaning is entangled with persistence and networking:

- missing dependencies can be confused with errors;
- retries can change observable behavior;
- database state can become an implicit algorithm input;
- protocol decisions can be hidden inside generic exceptions;
- product policy can leak into authorization;
- tests require large integration environments;
- alternative stores and transports become expensive;
- defects become difficult to reproduce precisely.

A pure event-application boundary reverses this dependency.

```text
immutable facts
        ↓
deterministic evaluation
        ↓
dependencies, bounded halt, or final decision
        ↓
semantic consequences
```

The host remains responsible for obtaining facts and committing results. The engine remains responsible for protocol meaning.

This creates a stable point of composition between Matrix semantics and server infrastructure.

---

## 5. The core hypothesis

The central technical hypothesis is:

> Matrix room-event application can be expressed as a deterministic function over a candidate event, an explicit room version, immutable supporting facts, explicit event provenance, a frozen key-validity reference timestamp, declared evaluation options, and a bounded work budget.

Conceptually:

```text
evaluate(
    candidate event,
    supported room version,
    immutable dependency snapshot,
    event provenance,
    key-validity reference timestamp,
    evaluation options,
    work budget
)
    →
    final decision
    | missing dependencies
    | bounded halt
```

A correct implementation should not need:

- a database connection;
- an HTTP client;
- an async runtime;
- ambient process state;
- wall-clock access;
- randomness;
- hidden caches;
- product configuration.

This does not mean Matrix itself is simple or stateless. It means all state relevant to one evaluation can be supplied explicitly as facts.

If this hypothesis fails, the project must identify the exact protocol behavior that requires ambient or effectful state rather than hiding the failure behind an abstraction.

---

## 6. The outcome hypothesis

The engine must model protocol meaning more precisely than `Result<Success, Error>`.

The thesis requires these distinctions:

```text
EvaluationResult
├── Complete
│   ├── Dropped
│   └── Admitted
│       ├── Accepted
│       ├── Rejected
│       └── SoftFailed
├── NeedsDependencies
└── Halted
    └── BudgetExceeded
```

These outcomes belong to different layers:

- **Dropped** means the event failed before normal participation in room history.
- **Rejected** means the event may remain relevant to history but cannot affect normal state or visibility.
- **Soft-failed** means it may be valid at its historical position but fails against current resolved state, or lacks a valid Policy Server recommendation after the required host acquisition attempt.
- **Accepted** means normal semantic consequences follow.
- **NeedsDependencies** means no final protocol decision is yet possible.
- **BudgetExceeded** means evaluation was deliberately halted without inventing a protocol result.
- **Duplicate** is deliberately absent because it requires durable runtime state.

The hypothesis is that preserving these distinctions in the public model will simplify every later layer rather than burden it.

---

## 7. The dependency hypothesis

Distributed event evaluation frequently begins with incomplete information.

CyborgHeart assumes that missing information should be represented as immutable fact requirements, not hidden I/O operations.

```text
evaluate
    ↓
NeedsDependencies
    ↓
host acquires facts
    ↓
evaluate again
```

This model is valuable because the same dependency may be satisfied by:

- memory;
- an embedded store;
- PostgreSQL;
- local cache;
- federation;
- import data;
- deterministic fixtures;
- a Policy Server recommendation acquisition result.

The engine need not know which source was used.

The first resumability model should be deterministic restart with an enriched snapshot. Internal execution continuations should not become a public persistence format before the lifecycle is stable.

The key property is dependency monotonicity:

> Adding valid required facts may turn an incomplete evaluation into a complete one, but must not change one already complete decision into a different complete decision.

If this property does not hold, either the earlier result was premature, the supplied facts conflict, or the evaluator is nondeterministic.

---

## 8. The bounded-work hypothesis

A protocol implementation is not correct in production if hostile input can force unbounded work.

Matrix event graphs, auth chains, state maps, canonical data, and signature sets can amplify CPU and memory use.

CyborgHeart therefore treats resource accounting as part of the semantic evaluation contract rather than a timeout added by the host.

The engine should measure dimensions such as:

- canonical JSON bytes;
- events inspected;
- state entries inspected;
- auth events and auth-chain edges;
- graph edges;
- signature verifications;
- state-resolution steps.

A budget halt must be:

- explicit;
- reproducible;
- distinguishable from a Matrix rejection;
- testable with adversarial fixtures.

The thesis is that bounded evaluation can be introduced without changing the protocol decision itself. The budget controls whether the engine completes, not what Matrix means.

---

## 9. The Ruma thesis

Ruma is a foundation, not a competitor.

CyborgHeart’s opportunity exists because Ruma already handles much of the protocol machinery correctly and exposes server-relevant algorithms.

The project should concentrate on what Ruma does not intend to own:

- lifecycle orchestration;
- immutable dependency discovery;
- trust-stage progression;
- semantic decision structure;
- work accounting;
- storage-neutral consequences;
- durable execution above the engine;
- homeserver composition.

This division reduces both scope and risk.

CyborgHeart succeeds by making Ruma more usable as part of a complete server architecture, not by replacing it.

A local implementation of Ruma-owned behavior is justified only when:

1. a defect is reproduced;
2. the expected Matrix behavior is documented;
3. a regression fixture exists;
4. the workaround is isolated;
5. removal remains possible when upstream behavior is corrected.

---

## 10. The architectural thesis

If the event-application boundary proves stable, the rest of the server can grow as a sequence of independently understandable layers.

```text
event application
        ↓
transaction runtime
        ↓
command runtime
        ↓
server capabilities
        ↓
server SDK
        ↓
homeserver products
```

### Event application

Determines protocol meaning from immutable facts.

### Transaction runtime

Obtains dependencies, detects duplicates, retries evaluation, commits results atomically, recovers from crashes, and creates durable outbox entries.

### Command runtime

Converts authenticated intent into signed local candidate events and submits them through the same authoritative path.

### Server capabilities

Add authentication, Client–Server APIs, sync, federation, devices, server keys, media, moderation, and administration without redefining room-event semantics.

### Server SDK

Composes proven layers into a coherent developer surface.

The architecture is not valuable merely because it has layers. It is valuable only if each boundary isolates a different kind of authority:

- protocol authority;
- durable execution;
- authenticated intent;
- product and operational capability;
- composition.

---

## 11. The product thesis

The eventual product is not only a homeserver.

CyborgHeart aims to become infrastructure from which different homeservers can be assembled.

Potential consumers include:

- a reference homeserver;
- a narrow self-hosted homeserver;
- an embedded or appliance-style server;
- a product-specific homeserver;
- experimental Matrix deployments;
- other Rust projects that need authoritative event application without adopting an entire server.

The value proposition is:

> Build on a tested server core rather than rediscovering Matrix room semantics inside each product.

This is only valuable if the lower layers remain independently usable. A nominal SDK wrapped around one inseparable implementation would not validate the thesis.

---

## 12. Why begin with room version 12

CyborgHeart begins with one explicit room version to make its support claim meaningful.

Room version 12 is not treated as a generic placeholder. It defines a concrete rule bundle for:

- event format;
- room and event IDs;
- creator semantics;
- authorization;
- redaction;
- state resolution;
- event references.

Beginning with one version allows the project to prove:

- the full lifecycle;
- the Ruma integration;
- the fixture model;
- the decision taxonomy;
- the dependency contract;
- the work-budget model.

Supporting several versions before one complete lifecycle is proven would increase surface area without validating the architecture.

New room versions should become explicit additions, not values accepted because an upstream enum can represent them.

---

## 13. Why begin with a testkit

The testkit is not support tooling around the product. It is part of the product thesis.

A reusable event engine requires a corpus that can prove:

- specification traceability;
- deterministic replay;
- dependency monotonicity;
- outcome distinctions;
- state and graph consequences;
- malformed-input behavior;
- adversarial work ceilings;
- regressions across Ruma and Matrix updates.

Without that corpus, the engine would be another implementation whose correctness depends on confidence in its authors.

The thesis is that protocol support should be demonstrated through durable evidence:

```text
specification clause
        ↕
Ruma behavior relied upon
        ↕
engine code
        ↕
fixture and expected result
```

A broad implementation without this mapping is less valuable than a narrow one with it.

---

## 14. What CyborgHeart is not betting on

The project does not depend on these claims:

- that every Matrix homeserver should share one internal architecture;
- that Ruma is incomplete or incorrect;
- that one database can serve every deployment;
- that all server capabilities should run in one process;
- that every client should use a CyborgHeart-specific API;
- that a plugin system is necessary;
- that broad client compatibility must precede a useful product;
- that repositories must be separated early;
- that CyborgHeart should replace existing homeservers.

CyborgHeart is making a narrower bet:

> A precise, reusable event-application and durable-execution core is independently valuable and can reduce the cost and risk of building new Matrix server products.

---

## 15. What must be proven first

The thesis is not validated by compiling a crate or accepting a happy-path event.

Before the first transaction runtime is justified, the event engine must demonstrate:

- a complete room-version-12 lifecycle;
- valid cryptographic trust progression;
- explicit missing-dependency behavior;
- deterministic replay;
- stable output ordering;
- dependency monotonicity;
- accepted, rejected, soft-failed, and dropped outcomes;
- bounded halts that do not invent decisions;
- linear and branching histories;
- state-before reconstruction;
- state resolution;
- the distinction between a soft-failed state event’s historical state-after and immediate current-room application;
- Policy Server enablement and recommendation handling;
- redaction partner handling, including pending-partner consequences and later re-evaluation;
- semantic graph and state consequences;
- malformed and adversarial input handling;
- specification and Ruma traceability;
- a public contract usable without repository internals.

Only then is the next hypothesis worth testing:

> Can a durable runtime repeatedly supply facts, commit consequences atomically, and preserve the engine’s semantics under retries and crashes?

---

## 16. Falsifiability

A thesis is useful only if it can fail.

The CyborgHeart thesis should be reconsidered if evidence shows that:

1. **The lifecycle cannot be isolated.**
   Normative event meaning genuinely depends on hidden mutable state or unavoidable I/O that cannot be represented as explicit facts.

2. **The boundary is unstable.**
   Ordinary lifecycle cases repeatedly require storage- or transport-specific types in the engine’s public contract.

3. **The abstraction adds more complexity than it removes.**
   A direct homeserver implementation remains clearer, more testable, and safer than the separated engine across representative scenarios.

4. **Ruma already provides the complete reusable boundary.**
   CyborgHeart becomes only a thin renaming layer with no meaningful lifecycle, dependency, evidence, or consequence model.

5. **The engine cannot be bounded predictably.**
   Work limits necessarily change protocol decisions or cannot be measured at useful boundaries.

6. **The engine is not independently reusable.**
   It only works through privileged assumptions from one transaction runtime or one homeserver.

7. **Conformance cannot be demonstrated.**
   The supported surface cannot be mapped reliably from specification to dependency behavior, code, and fixtures.

Any of these findings should lead to redesign, scope reduction, upstream contribution, or termination of the affected layer—not rationalization.

---

## 17. Primary risks

### Incorrect boundary placement

The engine may own too much, especially graph persistence policy or durable duplicate handling.

**Response:** Keep inputs immutable, outputs semantic, and durable concerns above the engine.

### False purity

An interface may look deterministic while relying on hidden caches, global state, unordered collections, or time-sensitive behavior.

**Response:** Make every observable input explicit and test replay across process runs and input orderings.

### Ruma mismatch

CyborgHeart may misunderstand, duplicate, or incorrectly sequence Ruma behavior.

**Response:** Maintain `ruma-coverage.md`, specification fixtures, upstream references, and regression cases.

### Premature API stability

Early internal wrappers may escape into the public contract and constrain later corrections.

**Response:** Publish a deliberately small API and keep lifecycle machinery private until external need is proven.

### Conformance theatre

Large fixture counts may create confidence without testing the normative branches that matter.

**Response:** Organize fixtures by specification obligation, outcome, adversarial property, and known regression—not by volume.

### Infrastructure drift

Pressure to produce a runnable server may pull storage, HTTP, and sync into the core before the lifecycle is complete.

**Response:** Maintain a hard gate before adding the transaction runtime and later layers.

### Overgeneralization

Designing for every room version, storage model, and deployment shape may prevent completion.

**Response:** Prove room version 12 and one complete lifecycle before broadening support.

---

## 18. Measures of success

Early success is not user count, federation traffic, or feature breadth.

The first phase succeeds when:

- the event engine can be understood without a homeserver;
- every required fact is explicit;
- every outcome has a precise owner;
- replay is deterministic;
- work is bounded;
- conformance is traceable;
- no storage or network assumption leaks into protocol meaning;
- an external-style testkit can exercise the full public contract.

The next phase succeeds when:

- a transaction runtime can host the engine without modifying its semantics;
- in-memory and durable stores can implement the same Matrix-shaped contract;
- retries, duplicate submissions, and crash recovery produce one authoritative commit;
- outbox projections remain downstream of the atomic room decision.

Long-term success means:

- multiple server products can share the core;
- capabilities can be selected without forking protocol semantics;
- new room versions can be added deliberately;
- defects can be reproduced from fixtures;
- downstream users can trust narrow support claims.

---

## 19. Strategic sequence

CyborgHeart should proceed by proving one hypothesis at a time.

### Hypothesis 1

A complete Matrix room-event lifecycle can be deterministic, explicit, bounded, and storage-neutral.

**Artifact:** event application and conformance testkit.

### Hypothesis 2

A durable host can repeatedly satisfy dependencies and commit semantic consequences atomically without contaminating the engine.

**Artifact:** transaction runtime and memory store.

### Hypothesis 3

Authenticated local intent can be translated into candidate events without creating a second authorization path.

**Artifact:** command runtime.

### Hypothesis 4

Server capabilities can compose around the lower runtime without bypassing it.

**Artifact:** minimal authentication, client API, sync, keys, media, and administration.

### Hypothesis 5

The proven composition can become a useful SDK rather than a wrapper around one application.

**Artifact:** server SDK extracted from a reference homeserver.

Each stage exists to test the next claim. No stage is justified only because it appears in the long-term diagram.

---

## 20. Working-status rule

This thesis remains deliberately marked **working** until the event-application engine either proves or falsifies its central boundary. Evidence may narrow, split, or replace a hypothesis. The working label is a commitment to revise the argument when implementation evidence disagrees with it.

---

## 21. Final thesis

CyborgHeart is a bet on a boundary.

The Matrix specification defines room behavior. Ruma provides the essential protocol and algorithmic machinery. Complete homeservers provide persistence, transport, APIs, operations, and products.

Between them lies a reusable responsibility:

> turn a candidate room event and explicit immutable facts into a deterministic, bounded, inspectable Matrix decision and semantic consequence.

If that responsibility can be isolated and proven, it becomes the heart of a modular server architecture.

If it cannot, CyborgHeart should discover that early—before databases, transports, SDKs, and products make the boundary too expensive to question.

That is why CyborgHeart begins with event application, and why every later layer must be earned by evidence.

---

## Companion documents

- [`MANIFESTO.md`](./MANIFESTO.md) — the principles CyborgHeart chooses.
- [`ARCHITECTURE.md`](./ARCHITECTURE.md) — the concrete layer and dependency design.
- [`SUPPORTED-SURFACE.md`](./SUPPORTED-SURFACE.md) — the exact Matrix behavior currently claimed.
- [`SPECIFICATION-MAP.md`](./SPECIFICATION-MAP.md) — the mapping from Matrix obligations to implementation and fixtures.

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

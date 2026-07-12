# CyborgHeart Architecture

**Status:** Initial architecture baseline
**Parent project:** ProjectCyborg
**Protocol baseline:** Matrix v1.19
**Initial target room version:** 12 only
**Implementation language:** Rust
**Primary protocol dependency:** Ruma
**Last reviewed:** 2026-07-12

---

## 1. Purpose

CyborgHeart is a modular server runtime for Matrix.

Its first responsibility is to turn one candidate room event and an explicit set of immutable room facts into a deterministic Matrix result.

Its later responsibilities may include durable event execution, authenticated command handling, server capabilities, and a composable homeserver SDK.

The architecture is built around one rule:

> Protocol meaning must be decided independently from transport, persistence, retries, projection, and product policy.

---

## 2. System context

```text
Matrix specification
normative protocol and interoperability rules
        ↓
Ruma
protocol types, canonical JSON, signatures,
room-version rules, authorization, state resolution
        ↓
CyborgHeart event application
deterministic lifecycle and semantic consequences
        ↓
CyborgHeart transaction runtime
dependency acquisition and durable atomic execution
        ↓
CyborgHeart command runtime
authenticated intent and local event construction
        ↓
server capabilities
auth, APIs, sync, federation, keys, devices,
media, moderation, administration
        ↓
server SDK
composition, configuration, lifecycle, secure defaults
        ↓
reference and product homeservers
```

Matrix defines correctness.

Ruma supplies protocol primitives and core algorithms.

CyborgHeart supplies the application lifecycle and the durable server execution around it.

---

## 3. Architectural principles

### 3.1 One authoritative event path

Every candidate room event that passes transport-specific admission must pass through one authoritative event-application path.

This applies to:

- federated events;
- locally constructed events;
- replayed events;
- imported events.

Different entry points may have different transport admission and explicit provenance, but they must not create different Matrix room-authorization rules.

### 3.2 Explicit facts

The event engine receives immutable facts.

It does not:

- query a database;
- fetch federation data;
- read ambient caches;
- inspect process-global state;
- consult the wall clock;
- use randomness.

If a fact required to decide the event is missing, the engine returns a dependency request. A related fact that affects only a deferred consequence, such as a missing redaction target, must not erase an otherwise complete event decision.

### 3.3 Deterministic results

Given the same:

- candidate event;
- supported room version;
- dependency snapshot;
- event provenance;
- key-validity reference timestamp;
- evaluation options;
- evaluation budget;

the engine must return the same:

- dependency request;
- completed decision;
- semantic consequences;
- work report.

### 3.4 Bounded work

All work influenced by untrusted input must be bounded and measured.

Budget exhaustion halts evaluation without inventing a Matrix decision.

### 3.5 Layered authority

Each layer owns a different form of authority:

| Layer | Authority |
|---|---|
| Matrix specification | Normative protocol behavior |
| Ruma | Protocol primitives and core algorithms |
| Event application | Matrix meaning of one candidate event, including Policy Server validation when applicable |
| Transaction runtime | Durable execution and atomic commit |
| Command runtime | Translation of authenticated intent into a candidate event |
| Server capabilities | Product and operational server behavior |
| Server SDK | Composition and developer-facing integration |
| Homeserver product | Deployment and product policy |

No layer may silently take ownership from the layer below it.

### 3.6 Narrow support claims

A room version or protocol rule is supported only when it is:

- deliberately admitted;
- mapped to the specification;
- mapped to relied-on Ruma behavior;
- implemented;
- covered by conformance fixtures;
- covered by malformed and adversarial tests where applicable.

---

## 4. Repository organization

The public CyborgHeart source of truth is one Git repository and one virtual Cargo workspace:

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
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── rustfmt.toml
├── deny.toml
└── .github/
```

The initial packages are:

```text
cyborg-heart-event-application
cyborg-heart-event-testkit
```

The workspace is a development and integration boundary. Crates are architectural boundaries. A repository split is justified only by an independent ownership, release, compatibility, security, or consumer boundary; no split is scheduled.

A private sibling repository named `cyborgheart-project-ops` may hold plans, delegation briefs, research notes, handoffs, reviews, and promotion history. It is a coordination layer only:

- public implementation behavior is authoritative in `cyborgheart`;
- accepted ADRs that govern code live in `cyborgheart/docs/adr/`;
- protocol contracts and support claims live in `cyborgheart/docs/foundational/`;
- fixture schemas and executable claims live in `cyborgheart/fixtures/`;
- the public repository must remain understandable, buildable, testable, and releasable without access to project operations;
- private drafts never override an accepted public document or merged code.

The two repositories should be siblings in an untracked local workspace rather than nested Git repositories:

```text
cyborgheart-workspace/
├── cyborgheart-project-ops/   # private coordination repository
└── cyborgheart/               # public product monorepo
```

Future packages are added only after their lower-layer contracts are proven. Likely additions are the transaction runtime, memory store, command runtime, selected capability crates, server SDK, and reference homeserver.

## 5. Dependency direction

Compile-time dependencies point toward lower layers.

```text
Ruma
  ↓
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
reference homeserver
```

Rules:

- lower layers never depend on upper layers;
- the event engine never depends on storage or transport;
- the transaction runtime never depends on HTTP or client APIs;
- capabilities depend on runtime contracts, not application internals;
- the reference homeserver consumes public SDK and capability APIs;
- no circular dependency may be hidden behind Cargo features.

Runtime requests travel downward:

```text
authenticated intent
        ↓
command runtime
        ↓
candidate event
        ↓
transaction runtime
        ↓
event application
        ↓
semantic result
        ↓
atomic commit
```

Compile-time and runtime directions are complementary, not contradictory.

---

## 6. Event-application architecture

### 6.1 Responsibility

> Given one canonical candidate room event and an immutable snapshot of available room facts, evaluate the Matrix room-event lifecycle and return a deterministic result.

### 6.2 Inputs

Conceptually:

```rust
struct EvaluationInput<'a> {
    event: CandidateEvent<'a>,
    room_version: SupportedRoomVersion,
    provenance: EventProvenance,
    options: EvaluationOptions,
    key_validity_reference_ts: MilliSecondsSinceUnixEpoch,
}
```

The evaluation also receives:

```rust
struct DependencySnapshot<'a> {
    events: &'a dyn EventView,
    states: &'a dyn StateView,
    signing_keys: &'a dyn SigningKeyView,
    policy_recommendations: &'a dyn PolicyRecommendationView,
}
```

and:

```rust
struct EvaluationBudget {
    // declared remaining limits and consumed work
}
```

The exact public API may evolve, but the boundary must preserve:

- explicit candidate event;
- explicit room version;
- explicit immutable facts;
- explicit frozen key-validity reference time;
- explicit evaluation options;
- explicit budget.

### 6.3 Result

The engine-level result is:

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

Important ownership:

- `Duplicate` does not belong to the event engine.
- `BudgetExceeded` is not a Matrix disposition.
- `Dropped` does not produce ordinary room graph or state effects.
- `Accepted` does not imply durable commit.
- protocol decisions are returned as values, not generic errors.

### 6.4 Lifecycle

The event-application lifecycle is:

```text
supported room-version admission
        ↓
canonical candidate event
        ↓
PDU format check
        ↓
required signer discovery and eligible-key preflight
        ↓
event identity, signature, and content-hash verification
        ↓
redacted authorization representation when required
        ↓
state-independent authorization
        ↓
claimed auth-event resolution
        ↓
authorization against claimed auth events
        ↓
state-before reconstruction
        ↓
authorization against historical state
        ↓
authorization against frozen current room state
        ↓
Policy Server recommendation validation when enabled
        ↓
accepted / rejected / soft-failed
        ↓
semantic graph, historical-state, redaction, and visibility consequences
```

Failure to map an external room-version identifier into `SupportedRoomVersion` is an unsupported-capability error before evaluation, not `Dropped`. A format or required-signature failure after evaluation begins may produce `Dropped`.

Missing facts required for the event decision produce `NeedsDependencies`. Facts that affect only a deferred redaction consequence remain represented in that consequence.

Budget exhaustion produces `Halted(BudgetExceeded)`.

### 6.5 Internal trust stages

Internal wrappers may represent trust progression:

```text
ParsedPdu
FormatCheckedPdu
VerifiedPdu
HistoricallyAuthorizedPdu
```

These are implementation details unless a stable external use case proves otherwise.

Initial internal responsibility modules include PDU inspection, cryptographic verification, authorization, state reconstruction, Policy Server validation, and redaction-effect evaluation.

The public API should expose domain meaning, not temporary algorithm structure.

### 6.6 Untrusted event representation

Untrusted PDUs enter as canonical JSON and are inspected lazily.

Do not deserialize every federated event into strict typed event content and treat deserialization success as protocol validity.

Typed Ruma event content remains appropriate later for:

- local command construction;
- client-originated content validation;
- known application-level event handling.

### 6.7 Ruma boundary

The engine composes Ruma for:

- Matrix identifiers;
- canonical JSON;
- room-version rules;
- event IDs and reference hashes;
- content hashes;
- signing;
- signature verification;
- event authorization;
- state resolution.

The engine owns:

- lifecycle sequencing;
- dependency discovery;
- trust-stage progression;
- outcome structure;
- bounded evaluation;
- evidence;
- semantic consequences.

A local replacement for Ruma-owned behavior requires:

- a reproduced upstream defect;
- a specification-backed expected result;
- a regression fixture;
- an ADR;
- an isolated and removable compatibility boundary.

---

## 7. Dependency architecture

### 7.1 Dependency

A dependency is an immutable fact required to continue evaluation.

Examples:

```text
event by ID
state before an event
state after an event
current room state
server signing keys
Policy Server recommendation fact
```

A dependency is not an instruction to perform I/O.

### 7.2 Dependency request

The engine returns the facts it lacks.

```text
evaluate
    ↓
NeedsDependencies
    ↓
host acquires facts
    ↓
evaluate again
```

The first resumability model is deterministic restart with an enriched snapshot.

Do not persist opaque internal continuations before the lifecycle is stable.

### 7.3 Dependency monotonicity

Adding valid missing facts may change:

```text
NeedsDependencies → Complete
```

It must not change:

```text
Complete(Accepted) → Complete(Rejected)
```

If a complete decision changes after adding valid facts, then:

- the earlier result was premature;
- the facts conflict;
- or the engine is nondeterministic.

This property must be tested directly.

### 7.4 Fact providers

The engine observes abstract fact views.

The future host may satisfy them from:

- in-memory fixtures;
- memory store;
- embedded durable store;
- PostgreSQL;
- cache;
- federation;
- import archive.

The engine must not know which provider was used.

---

## 8. Decision and consequence architecture

### 8.1 Dropped

A candidate fails before normal room-history participation.

Examples:

- malformed required fields;
- invalid required signature;
- unrecoverable canonical-data failure.

A dropped event produces no ordinary graph or state effect.

### 8.2 Rejected

An admitted event fails state-independent authorization, claimed-auth authorization, or authorization against historical state.

It may remain relevant to room history or federation processing but must not behave as an ordinarily accepted event.

### 8.3 Soft-failed

An admitted event may be valid at its historical position but fail authorization against frozen current room state, or lack a valid required Policy Server recommendation after the host’s required acquisition attempt.

A soft-failed event remains part of room history and federation semantics. It is not immediately relayed normally to clients or selected as a local predecessor.

### 8.4 Accepted

An admitted event passes every required Matrix decision stage, including Policy Server recommendation validation when the room uses one, and produces normal semantic consequences.

Accepted does not mean committed.

### 8.5 Deferred consequences

A related effect may remain pending after the event disposition is complete.

For an accepted redaction event without a currently valid partner event, the engine returns an accepted event application with `consequence.redaction_pending_partner`. It does not return `NeedsDependencies` solely to wait for that partner. The durable runtime retains the relation and re-evaluates the effect when a valid partner becomes available.

### 8.6 Semantic consequences

The engine returns protocol meaning, not database instructions.

Conceptually:

```rust
struct AdmittedDecision {
    outcome: AdmittedOutcome,
    consequences: SemanticConsequences,
    evidence: DecisionEvidence,
}

enum AdmittedOutcome {
    Accepted(AcceptedCode),
    Rejected(RejectionCode),
    SoftFailed(SoftFailureCode),
}

struct SemanticConsequences {
    historical_state: HistoricalStateEffect,
    forward_extremity: ForwardExtremityEffect,
    redaction: RedactionEffect,
}
```

Disposition, terminal lifecycle stage, and visibility class are derived from the admitted outcome and code. Dropped events are complete decisions but have no ordinary graph or state consequence object.

### 8.7 Graph effect

A graph effect describes protocol-level DAG consequences, such as:

- predecessor references;
- forward-extremity removal;
- forward-extremity addition.

### 8.8 State effect

A state effect describes historical state at the candidate’s graph position:

- a message event leaves historical state unchanged;
- an accepted or soft-failed state event that passed historical authorization replaces its `(type, state_key)` tuple in historical state after;
- a rejected state event leaves historical state after equal to historical state before.

A soft-failed state event does not immediately advance the server’s current forward extremities or current resolved state. Its historical state-after remains available because later events may reference it and state resolution may eventually place it in current state.

Historical state effect, current-room application, and durable storage representation are therefore distinct concepts. This layer does not describe tables, rows, keys, or SQL operations.

### 8.9 Evidence

Evidence should make a decision inspectable.

It may contain:

- completed lifecycle stages;
- signers and keys checked;
- content-integrity status;
- authorization contexts;
- state-resolution information;
- work consumed.

Logs are supplementary. Evidence is part of the result model.

---

## 9. Evaluation budget architecture

The engine must account for work explicitly.

Initial budget dimensions may include:

- canonical JSON bytes;
- events inspected;
- state entries inspected;
- auth events inspected;
- auth-chain edges;
- graph edges;
- signature verifications;
- state-resolution steps.

A budget has two roles:

1. prevent unbounded work under hostile input;
2. make resource use reproducible and testable.

The budget must not change Matrix meaning.

It determines whether evaluation completes, not what the completed decision would be.

---

## 10. Event testkit architecture

### 10.1 Responsibility

> Exercise the event-application crate as an external consumer using specification-traceable scenarios.

The testkit owns:

- fixture loading;
- scenario assembly;
- expected-result comparison;
- deterministic replay;
- budget-ceiling assertions;
- property tests;
- adversarial tests;
- regression tests.

Dependency direction:

```text
event-testkit
    depends on event-application

event-application
    never depends on event-testkit
```

### 10.2 Fixture shape

Each fixture should include:

- fixture ID;
- source and license provenance;
- Matrix specification version;
- room version;
- normative section links;
- candidate event;
- supporting events;
- supporting states;
- signing keys;
- expected dependency requests;
- expected decision;
- expected graph and state consequences;
- expected evidence;
- work ceiling.

Current seed shape:

```text
fixtures/
├── schema/
│   └── fixture.schema.json
└── seed/
    ├── budget/
    ├── capability/
    ├── input/
    ├── room-v12/
    └── signing/
```

Each seed file is one self-contained fixture object.

### 10.3 Test layers

```text
unit tests
conformance fixtures
property tests
adversarial fixtures
regression fixtures
fuzz targets
```

Snapshots may help review but must not be the only correctness oracle.

---

## 11. Future transaction runtime

The transaction runtime is added only after the event engine is proven.

### 11.1 Responsibility

> Supply dependencies, invoke event application repeatedly, detect duplicates, and commit authoritative effects atomically.

### 11.2 Flow

```text
event submission
        ↓
derive or inspect event identity
        ↓
check durable duplicate state
        ↓
invoke event application
        ↓
load requested facts
        ↓
repeat until final result
        ↓
translate semantic consequences
        ↓
atomic commit
        ↓
record durable outbox entries
        ↓
return committed outcome
```

### 11.3 Ownership

The transaction runtime owns:

- durable duplicate detection;
- idempotency;
- dependency acquisition;
- retries;
- transaction conflicts;
- atomic persistence;
- crash recovery;
- durable outbox;
- committed outcome.

It must not redefine:

- event authorization;
- state resolution;
- event disposition;
- room-version semantics.

### 11.4 Storage boundary

Storage APIs should be Matrix-shaped rather than generic key-value access.

Likely capabilities:

- event retrieval;
- state before and after an event;
- current room state;
- graph extremities;
- signing keys;
- processing outcomes;
- atomic room commit;
- outbox storage.

Initial adapter order:

```text
memory store
    ↓
one embedded durable store
    ↓
PostgreSQL
```

No storage type may leak into the event engine.

---

## 12. Future command runtime

### 12.1 Responsibility

> Convert authenticated intent into a valid local candidate event.

Examples:

```text
CreateRoom
SendMessage
SendStateEvent
InviteUser
JoinRoom
LeaveRoom
KickUser
BanUser
RedactEvent
```

### 12.2 Flow

```text
authenticated actor
        ↓
command
        ↓
load current forward extremities and required state
        ↓
construct candidate event
        ↓
select auth-event references
        ↓
select predecessors and depth
        ↓
hash and sign through Ruma
        ↓
submit to transaction runtime
```

The command runtime does not decide Matrix authorization.

The resulting event must pass through the same event-application engine as a remote event.

## 13. Future server capabilities

Capabilities remain separate concerns because they have different:

- data models;
- security boundaries;
- dependencies;
- operational behavior;
- test strategies.

Likely capability areas:

```text
authentication and accounts
Client–Server API
sync
federation ingress
federation dependency acquisition
federation delivery
server keys
devices and E2EE metadata
media
moderation
administration
```

A capability is not automatically:

- a separate repository;
- a separate process;
- a plugin;
- replaceable protocol-critical logic.

Protocol-critical authorization and state resolution must not become casual extension hooks.

### 13.1 Policy Server acquisition boundary

Event application determines whether the room uses a Policy Server and validates supplied recommendation facts. It never calls `POST /_matrix/policy/v1/sign`.

The future host or federation capability obtains or refreshes a Policy Server signature and re-invokes event application with the resulting explicit fact. Local command handling should refuse the client action when required and possible; a federated event, or a local event deliberately processed anyway, may be soft-failed after the required attempt fails.

### 13.2 Projection rule

Sync, federation delivery, search, analytics, and notifications should be downstream projections of an authoritative atomic commit.

```text
authoritative commit
        ↓
durable outbox
        ↓
projection workers
```

A projection is rebuildable.

Authoritative room state is not a projection.

---

## 14. Future server SDK

The SDK is created only after a real reference homeserver composition exists.

Its responsibilities may include:

- composition;
- configuration validation;
- lifecycle management;
- secure defaults;
- capability registration;
- supported extension hooks;
- deployment profiles;
- integration test utilities.

Conceptually:

```rust
let server = CyborgHeartServer::builder()
    .store(store)
    .authentication(authentication)
    .client_api(client_api)
    .sync(sync)
    .server_keys(server_keys)
    .media(media)
    .federation(federation)
    .build()?;
```

The SDK assembles proven parts.

It must not absorb their implementations or conceal a monolith.

---

## 15. Security architecture

Treat all event input as hostile.

Requirements:

- bound accepted input size;
- charge untrusted data and graph work to the evaluation budget;
- avoid recursion controlled directly by input depth;
- guard arithmetic and traversal;
- treat parser panics as defects;
- fuzz untrusted byte and JSON boundaries;
- retain every discovered crash or pathological case as a regression fixture;
- forbid unsafe Rust in the initial workspace;
- keep secrets and real private keys out of fixtures;
- review protocol and cryptographic dependency updates explicitly.

Security should follow from:

- immutable inputs;
- explicit dependencies;
- no ambient authority;
- bounded work;
- narrow public APIs;
- strict dependency direction;
- explicit decisions;
- permanent regression evidence.

---

## 16. Documentation architecture

The repository documentation is divided by responsibility.

| Document | Responsibility |
|---|---|
| `MANIFESTO.md` | Enduring principles |
| `WORKING-THESIS.md` | Falsifiable project argument, intentionally revised by evidence |
| `WORKING-LEXICON.md` | Current preferred terminology |
| `ARCHITECTURE.md` | Concrete system structure and boundaries |
| `SUPPORTED-SURFACE.md` | Exact implemented Matrix support |
| `SPECIFICATION-MAP.md` | Matrix clauses mapped to code and fixtures |
| `RUMA-COVERAGE.md` | Ruma APIs relied upon and known gaps |
| `EVENT-LIFECYCLE.md` | Detailed event-processing stages |
| `DEPENDENCY-MODEL.md` | Fact model and resumability |
| `DECISION-CODES.md` | Stable machine-readable decision taxonomy |
| `REFERENCES.md` | Canonical external-source index and documentation boundary |
| `../adr/*` | Decisions that should not be rediscovered |

When supported behavior changes, code, fixtures, and documentation change in the same pull request.

---

## 17. Evolution gates

### Gate 1 — before transaction runtime

Do not add the transaction runtime until:

- the complete room-version-12 lifecycle is represented;
- deterministic replay is proven;
- output ordering is stable;
- dependency monotonicity is tested;
- accepted, rejected, soft-failed, and dropped cases exist;
- budget exhaustion is explicit and reproducible;
- branching state resolution is covered;
- signing-key validity and restricted-join signer requirements are covered;
- Policy Server enablement, recommendation acquisition, and final soft failure are covered;
- redaction effects include applied and pending-partner cases with later re-evaluation;
- soft-failed historical state-after semantics are covered;
- malformed and adversarial input is tested;
- untrusted intake is fuzzed;
- the public contract works without repository internals;
- the engine contains no I/O, async runtime, storage, durable duplicate detection, or product policy.

### Gate 2 — before command runtime

Do not add the command runtime until:

- dependency acquisition works through the transaction runtime;
- duplicate submissions are idempotent;
- commits are atomic;
- crash recovery is defined;
- outbox creation is atomic with authoritative room commit;
- a memory store proves the runtime contract.

### Gate 3 — before server SDK

Do not add the SDK until:

- a minimal reference homeserver exists;
- real capability composition has exposed the needed interfaces;
- configuration and lifecycle needs are concrete;
- the reference server can consume the intended public APIs as an external project would.

---

## 18. Deliberate non-goals

CyborgHeart does not initially aim to:

- replace Ruma;
- support every Matrix room version;
- support arbitrary Matrix clients;
- implement every homeserver capability;
- define a universal storage abstraction;
- provide dynamic plugins;
- split into many repositories;
- require distributed deployment;
- become a new Matrix specification authority;
- fork or repackage an existing homeserver.

The initial goal is narrower:

> Prove a deterministic, bounded, storage-neutral Matrix room-event application boundary.

---

## 19. Future separability

The early monorepo should preserve future separation without scheduling it.

Possible future boundaries:

```text
event foundation
server runtime
storage adapters
server capabilities
server SDK
reference homeserver
```

A repository split is justified only by real pressure such as:

- independent consumers;
- stable versioned interfaces;
- independent releases;
- separate ownership;
- stricter security stewardship;
- different languages or build systems;
- material repository-scale problems.

Repository separation does not require process separation.

A mature homeserver may remain one executable or later use separate workers for sync, federation, media, and other projections.

---

## 20. Final architecture commitment

> CyborgHeart starts as a pure Matrix room-version-12 event-application engine and a conformance testkit in one Rust workspace. The engine accepts canonical candidate events and immutable facts, composes Ruma’s protocol algorithms, reports missing dependencies and bounded halts explicitly, and returns deterministic Matrix decisions with semantic graph and state consequences. Durable execution, commands, capabilities, SDK composition, and homeserver products are added only after the lower boundary is proven.

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

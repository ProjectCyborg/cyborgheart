# CyborgHeart Working Lexicon

**Status:** Working terminology baseline
**Purpose:** Keep architecture, code, tests, and documentation precise and consistent.
**Last reviewed:** 2026-07-12

This document defines CyborgHeart’s current preferred terms. It remains **working** until the public event-application contract stabilizes.

Where Matrix or Ruma already defines a term, CyborgHeart inherits that meaning and does not redefine it. Where CyborgHeart introduces an architectural term, this document defines its current scope and owning layer.

---

## 1. Naming rules

### Use the narrowest correct term

Prefer:

```text
candidate event
dependency request
event decision
state before
transaction runtime
```

Avoid broad substitutes such as:

```text
payload
data
context
result
processor
manager
service
```

unless the broader term is genuinely intended.

### Use project spelling consistently

CyborgHeart uses **authorization** in project prose and APIs. Preserve **authorisation** only when quoting or naming a Matrix specification section.

### Distinguish protocol meaning from durable execution

Use:

```text
event application
```

for deterministic Matrix evaluation.

Use:

```text
transaction
commit
retry
duplicate
outbox
```

for durable runtime behavior.

Do not describe database operations as event authorization, and do not describe protocol decisions as persistence outcomes.

### Distinguish incomplete evaluation from failure

Missing dependencies and budget exhaustion do not imply a Matrix rejection.

Use the exact term:

```text
NeedsDependencies
BudgetExceeded
Rejected
SoftFailed
Dropped
Accepted
```

Do not collapse them into `failed`, `invalid`, or `error`.

### Do not imply unsupported compatibility

Use:

```text
supported room version
```

only when the version is deliberately admitted, mapped, implemented, and tested.

Do not use:

```text
known room version
recognized room version
available RoomVersionId
```

as a synonym for support.

---

## 2. Project and ecosystem terms

### CyborgHeart

The ProjectCyborg server-runtime project described by the manifesto and thesis.

CyborgHeart begins with deterministic Matrix room-event application and may later include durable runtimes, server capabilities, an SDK, and homeserver products.

CyborgHeart is not a synonym for Matrix, Ruma, or a complete homeserver.

### ProjectCyborg

The overarching project under which CyborgHeart is developed.

### Matrix

The open protocol and specification that defines interoperable behavior.

Matrix is the normative source of protocol correctness.

### The Matrix.org Foundation

The independent organization that stewards the Matrix specification and ecosystem.

CyborgHeart is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation.

### Ruma

The Rust Matrix library ecosystem used by CyborgHeart for protocol types, canonical JSON, cryptographic operations, room-version rules, authorization, state resolution, and related primitives.

Ruma is a dependency and upstream collaborator boundary, not a layer owned by CyborgHeart.

---

## 3. Core protocol terms

These terms retain their Matrix meanings.

### Event

A Matrix protocol record.

Use a more specific term when possible:

- room event;
- state event;
- message event;
- PDU;
- EDU;
- candidate event;
- accepted event.

Do not use `event` when the subject is actually a command, API request, transaction, or database record.

### Room event

An event that participates in a Matrix room.

### State event

A room event with a `state_key`.

A state event may update the room’s state at the tuple:

```text
(event type, state key)
```

### PDU

A persistent data unit used by Matrix federation for room events.

In CyborgHeart’s initial scope, untrusted room-event input is handled as canonical PDU-shaped JSON.

Do not use PDU as a synonym for every Matrix event.

### EDU

An ephemeral data unit used in federation.

EDUs are outside the initial event-application scope.

### Canonical JSON

The canonical JSON representation required by Matrix for hashing, signing, and other protocol behavior.

CyborgHeart uses Ruma’s canonical JSON types and rules.

### Event ID

The Matrix identifier of an event, derived or represented according to the applicable room version.

Do not assume one event-ID algorithm applies to every room version.

### Room version

An immutable bundle of Matrix room rules, including event format, identifiers, references, authorization, redaction, and state resolution.

### Supported room version

A room version explicitly implemented and proven by CyborgHeart.

The initial target is room version 12. At repository initialization, no room version is supported. Room version 12 becomes supported only after the promotion gate in `SUPPORTED-SURFACE.md` is satisfied.

### Unsupported room version

A room version that CyborgHeart has not deliberately admitted into its supported contract.

This is an implementation capability error at the evaluation boundary, not a Matrix `Dropped` disposition for the event.

### Auth event

An event referenced or required for authorization of another event.

Do not confuse:

```text
auth event
```

with:

```text
user authentication
```

Auth events concern room-event authorization. Authentication concerns establishing actor identity.

### State before

The room state immediately before a candidate event at its historical position.

### State after

The historical room state immediately after a candidate event at its position in the event graph.

- For a message event, state after equals state before.
- For a historically authorized state event, including one later classified as soft-failed, state after replaces the matching `(event type, state key)` tuple with the candidate.
- For a rejected state event, state after equals state before.

Historical state after is distinct from the server’s immediate current room state.

### Current room state

The server’s current resolved room state used for the current-state authorization check.

Current room state is not necessarily the same as the historical state before the candidate event.

### State map

A mapping from:

```text
(event type, state key)
```

to a state event identifier.

### State resolution

The Matrix algorithm that resolves conflicting room-state sets.

CyborgHeart orchestrates Ruma’s implementation of the applicable room-version algorithm.

### Forward extremity

An event at the forward edge of the room DAG that is not referenced as a predecessor by another known event.

Do not use `room head` when exact Matrix graph terminology is required. `Room head` may be used informally only where the distinction is irrelevant.

### Redaction

The Matrix-defined process that removes event fields according to room-version rules.

Do not use `redaction` as a generic synonym for secret removal or privacy filtering.

### Signature verification

Cryptographic verification of required Matrix signatures.

### Content-hash mismatch

A condition in which required signatures may verify while the event content hash does not match.

This is not automatically equivalent to forgery. Matrix processing may require authorization using a redacted representation.

### Key-validity reference timestamp

The explicit host-supplied timestamp used when applying Matrix's seven-day signing-key validity cap.

It is frozen for one dependency-completion sequence and included in replay evidence. It is not read from the wall clock by the event engine and is distinct from the event's `origin_server_ts`.

### Matrix Policy Server

A Matrix protocol role enabled by current room state through `m.room.policy` and a joined user from the configured `via` server.

A Matrix Policy Server is not product moderation policy and does not replace room-event authorization.

### Policy Server recommendation

The Matrix recommendation represented by a valid `ed25519:policy_server` signature when a Policy Server is enabled for the room.

If the signature is missing or invalid, the host may need to acquire a fresh recommendation. After the required attempt, a federated event—or a local event deliberately processed anyway—may be soft-failed if no valid recommendation is available.

### Policy Server recommendation fact

Host-supplied signature material, or a terminal acquisition outcome, for a required Matrix Policy Server recommendation.

The fact is bound to the candidate event and the active `m.room.policy` configuration. The engine validates supplied signature material itself and must not trust a host-provided `valid` flag. It never contacts the Policy Server.

## 4. Event-application terms

These terms are defined by CyborgHeart’s architecture.

### Event application

The deterministic evaluation of one candidate room event against an explicit room version, immutable facts, evaluation options, and work budget.

Event application determines protocol meaning. It does not persist data or perform network requests.

Preferred forms:

```text
event application
event-application engine
event-application crate
```

Avoid:

```text
event handler
event processor
event service
```

when referring to the authoritative lifecycle.

### Candidate event

The event currently being evaluated.

A candidate event has not yet been classified as accepted, rejected, soft-failed, or dropped.

Do not call it an accepted event before evaluation completes.

### Event provenance

Explicit information about how a candidate entered the system, such as federated, locally constructed, replayed, or imported.

Provenance must be supplied as data when Matrix makes it relevant. It must not be inferred from hidden transport or process state, and it must not silently alter room authorization rules.

### Immutable fact

A caller-supplied piece of information used during evaluation, such as:

- another event;
- a state map;
- current room state;
- a signing key;
- room-version selection.

The term emphasizes that the engine observes facts but does not acquire or mutate them.

### Dependency

A specific immutable fact required to continue evaluation.

Examples:

- an event by ID;
- state before or after an event;
- current room state;
- server signing keys.

A dependency is not an instruction to perform I/O.

### Dependency request

The explicit set of missing dependencies returned by the engine.

Preferred result term:

```text
NeedsDependencies
```

Avoid:

```text
fetch request
database request
network request
missing-data error
```

### Dependency snapshot

The immutable collection or view of facts currently available to one evaluation attempt.

A dependency snapshot may be backed by memory, a store, a cache, federation results, imported data, or fixtures. The engine must not know which.

### Dependency monotonicity

The required property that adding valid missing facts may advance an incomplete evaluation to completion but must not change one completed decision into another completed decision.

### Evaluation

One deterministic attempt to apply a candidate event using the currently supplied facts and budget.

Evaluation may end with:

- a complete decision;
- a dependency request;
- a bounded halt.

### Evaluation options

Explicit non-product options controlling evidence collection or other non-normative evaluation behavior.

Evaluation options must not skip a required Matrix stage for a final decision. Product moderation, account rules, and operator preferences are not evaluation options.

### Evaluation stage

A named lifecycle phase such as:

- format checking;
- cryptographic verification;
- state-independent authorization;
- claimed-auth authorization;
- historical-state authorization;
- current-state authorization;
- consequence calculation.

### Trust stage

An internal representation of how far an event has progressed through validation and authorization.

Trust-stage wrappers should remain private unless a stable external need emerges.

### Event decision

The completed Matrix classification of a candidate event.

An event decision is not a durable commit result.

### Complete

An evaluation result indicating that no additional facts or work are required to classify the event.

A complete result may be dropped or admitted.

### Admitted

A completed event that passed the pre-admission checks required to participate in room-history processing.

An admitted event is then classified as:

- accepted;
- rejected;
- soft-failed.

`Admitted` does not mean `Accepted`.

### Dropped

A completed decision indicating that the candidate failed before normal participation in room history.

Typical reasons include malformed PDU structure, invalid required signatures, or unrecoverable canonical-data failure.

A dropped event does not produce ordinary graph or state consequences.

Avoid using `dropped` for database deletion, log loss, queue eviction, or operator filtering.

### Accepted

An admitted event that passes the required Matrix authorization checks and produces normal semantic consequences.

Accepted does not mean durably committed.

### Rejected

An admitted event that fails state-independent authorization, claimed-auth authorization, or authorization against historical state.

A rejected event may remain relevant to room history or federation processing, but it must not behave as an ordinarily accepted event.

Do not use `rejected` for malformed pre-admission input.

### Soft-failed

An admitted event that may be valid at its historical position but fails authorization against current resolved room state, or lacks a valid required Policy Server recommendation after the host’s required acquisition attempt.

Soft failure is a specific Matrix outcome. It is not a generic warning, partial failure, or retryable error.

### Halt

An evaluation that ends without a Matrix decision because an explicit operational bound was reached.

### Budget exceeded

A halt caused by exhaustion of a declared evaluation budget.

Budget exhaustion must not be represented as rejection, soft failure, or invalid input.

### Evaluation budget

The declared upper limits on work that one evaluation may consume.

Possible dimensions include:

- canonical JSON bytes;
- events inspected;
- state entries inspected;
- auth-chain edges;
- graph edges;
- signature verifications;
- state-resolution steps.

### Work report

The structured account of work consumed during evaluation.

A work report is evidence and operational input, not a Matrix disposition.

### Evidence

Structured information explaining how a decision was reached.

Evidence may include:

- completed stages;
- signers and keys checked;
- integrity status;
- authorization contexts;
- state-resolution details;
- work consumed.

Logs are not a substitute for evidence.

### Semantic consequence

A protocol-level description of what follows from an admitted event.

Examples include:

- state before;
- state after;
- state delta;
- graph effects;
- visibility class.

Semantic consequences do not specify database tables or transaction statements.

### Pending consequence

A semantic consequence that cannot yet be completed but does not prevent the event’s disposition from being decided.

The principal initial example is an accepted redaction event without a currently valid partner event. This is not `NeedsDependencies`: the event decision is complete, while the redaction effect and client visibility remain pending and are re-evaluated when a valid partner is available.

### Redaction effect

The semantic result of applying the additional Matrix checks for an accepted `m.room.redaction` event.

The initial stable states are:

```text
applied
pending_partner
```

`pending_partner` covers a missing partner, a partner that is not yet valid, or a currently nonqualifying pair. Matrix requires the server to wait and re-check if a valid partner later arrives; CyborgHeart therefore does not model a terminal `not_permitted` redaction outcome.

### Event application result

The full engine result:

```text
complete decision
or
missing dependencies
or
bounded halt
```

Avoid using `event result` without clarifying whether it means evaluation, commit, API, or projection result.

---

## 5. Graph and state consequence terms

### Graph effect

The semantic change to the known room DAG implied by an admitted event.

This may include predecessor references and forward-extremity changes.

### State effect

The semantic effect of an event on state at its historical position.

Preferred forms:

```text
state unchanged
state updated
```

A soft-failed state event may produce an updated historical state-after while remaining unapplied to current resolved room state at receipt. A rejected state event does not update its historical state tuple.

### Current-room application

Whether an admitted event is immediately incorporated into the server's current resolved room state.

This is distinct from historical state effect. Soft-failed events are retained with their historical consequences for later state resolution but are not immediately applied as ordinary current-state events.

### State delta

The difference between state before and state after.

A state delta describes protocol meaning, not a database patch format.

### State reference

A stable reference to a state set or state snapshot.

The exact storage representation is deliberately unspecified at the event-application layer.

### Visibility class

The protocol-relevant initial delivery treatment derived from an event’s disposition, such as normal, rejected, or soft-failed handling.

This is not a permanent access-control verdict. A soft-failed state event can later enter resolved current state through state resolution and then be delivered to clients in the usual way. Product-specific access control and moderation visibility belong to higher layers.

### Event record

A semantic representation of the evaluated event suitable for handoff to a durable runtime.

Avoid using `event record` to imply a particular database row layout.

---

## 6. Durable-runtime terms

These belong to the future transaction runtime, not the event engine.

### Transaction runtime

The layer that supplies dependencies, invokes event application, detects duplicates, retries safely, and commits authoritative effects atomically.

Use `transaction runtime` for this architectural layer.

Do not shorten it to `transaction` when referring to the component itself.

### Submission

A request to the transaction runtime to process a candidate event.

### Duplicate

A submission whose event has already been processed or committed according to durable runtime state.

Duplicate is not an event-engine disposition.

### Idempotency

The property that repeated equivalent submissions do not create multiple authoritative commits.

### Retry

A repeated runtime attempt caused by incomplete dependencies, transient failures, conflicts, or crash recovery.

A retry must not change protocol meaning.

### Commit

The atomic durable application of authoritative event, graph, state, outcome, and outbox records.

Accepted does not mean committed. Committed means persistence has succeeded.

### Commit plan

The storage-oriented translation of semantic consequences into one durable atomic operation.

A commit plan belongs above event application.

### Committed outcome

The durable result returned after a successful atomic commit.

### Crash recovery

The process of restoring or completing durable runtime work after interruption without violating idempotency or atomicity.

### Outbox

Durable downstream work recorded atomically with an authoritative commit.

Examples may include:

- sync projection;
- federation delivery;
- audit projection;
- notifications.

The outbox is not authoritative room state.

### Projection

A derived view produced from authoritative committed data.

Sync feeds, search indexes, analytics, and outbound queues are projections.

A projection may be rebuilt. Authoritative room state may not be reconstructed from an incomplete projection without proof.

---

## 7. Command-runtime terms

### Command

Authenticated intent to perform a room operation.

Examples:

- create room;
- send message;
- send state event;
- invite;
- join;
- leave;
- ban;
- redact.

A command is not a Matrix room event.

### Command runtime

The future layer that converts authenticated intent into a candidate local event.

### Actor

The authenticated identity on whose behalf a command is issued.

### Local event construction

The process of creating a candidate room event from a command, including selection of predecessors and auth references, hashing, and signing.

### Authoritative event path

The single event-application path through which both local and remotely received candidate events must pass.

Local construction must not create an alternative authorization path.

---

## 8. Server terms

### Product policy

Operator- or product-defined behavior such as account suspension, spam controls, server allowlists, media quarantine, and moderation workflows.

Product policy must not be confused with Matrix authorization or a Matrix Policy Server recommendation.

### Server capability

A separately scoped area of homeserver behavior built around the lower runtimes.

Examples:

- authentication;
- Client–Server API;
- sync;
- federation;
- server keys;
- devices;
- media;
- moderation;
- administration.

A capability is not automatically a separate repository or process.

### Authentication

The process of establishing the identity of an actor.

Do not confuse authentication with Matrix room-event authorization.

### Authorization

The Matrix rules that determine whether an event is permitted in its room context.

### Client–Server API

The Matrix API surface used by clients to communicate with a homeserver.

### Federation ingress

The capability that receives and authenticates server-to-server traffic and submits candidate PDUs to the durable runtime.

### Server ACL

A Matrix federation-ingress rule controlling whether a requesting server may submit traffic for a room.

Server ACL evaluation happens before a candidate PDU reaches event application. It is not room-event authorization and is outside the initial engine.

### Federation dependency acquisition

The capability that obtains missing events, state, auth chains, or signing keys from remote servers.

### Federation delivery

The downstream capability that delivers committed local events to remote destinations.

### Server-key service

The capability that owns local signing keys and retrieves, validates, caches, and publishes remote or local server-key material.

Ruma performs cryptographic operations; the server-key service owns key lifecycle and discovery.

### Sync

A client-facing projection of authoritative committed changes.

Sync is not the source of truth for room state.

### Server SDK

The future composition layer that assembles proven runtimes and capabilities into a developer-facing server platform.

The SDK is not the entire repository and must not absorb all implementations.

### Reference homeserver

A complete runnable homeserver that proves the SDK and lower contracts are sufficient.

### Product homeserver

An opinionated homeserver distribution with selected capabilities, policies, storage, federation behavior, and operational defaults.

### Product client

A client designed for a product homeserver’s supported Matrix profile.

It may use an existing Matrix client SDK.

---

## 9. Testing and support terms

### Testkit

The `cyborg-heart-event-testkit` crate and fixture system that exercise the event application through its public API.

The testkit is a primary deliverable.

### Fixture

A self-contained protocol scenario with inputs, available dependencies, expected results, provenance, and specification references.

### Conformance fixture

A fixture that demonstrates expected behavior for a normative Matrix rule.

### Adversarial fixture

A fixture designed to test malformed input, amplification, pathological graphs, or budget behavior.

### Regression fixture

A minimized permanent reproduction of a discovered defect.

### Property test

A test of a general invariant across generated or varied inputs.

Important properties include:

- deterministic replay;
- stable output ordering;
- dependency monotonicity;
- input-order independence where required.

### Fuzz target

An isolated entry point for automated generation of hostile or unexpected inputs.

### Supported surface

The exact Matrix behavior CyborgHeart currently claims to implement.

### Specification map

The mapping from normative Matrix requirements to implementation locations and fixtures.

### Ruma coverage

The documented mapping from CyborgHeart behavior to the Ruma APIs and rules it relies upon, including known gaps or workarounds.

### Compatibility claim

A public statement of protocol or room-version support backed by implementation, documentation, and conformance evidence.

---

## 10. Terms to avoid or qualify

### Foundation

Do not use `Foundation` as the CyborgHeart project or repository name. It risks confusion with The Matrix.org Foundation.

It may be used generically in prose only where no organizational identity is implied.

### Kernel

Avoid unless CyborgHeart eventually defines a precise component with that responsibility.

The term currently implies more authority and stability than has been proven.

### Core

Use only with qualification:

```text
event core
runtime core
server core
```

Avoid calling the entire project `the Matrix core`.

### Mutation

Prefer `semantic consequence`, `state effect`, or `commit plan`.

Use `mutation` only when the subject is an actual in-memory or durable change operation.

### Context

Use a more specific term when possible:

- dependency snapshot;
- historical state;
- current room state;
- evaluation options;
- provenance.

### Valid

Always qualify what is valid:

- canonical JSON valid;
- signature valid;
- format valid;
- historically authorized;
- currently authorized;
- accepted.

An event can satisfy one condition and fail another.

### Failure

Prefer the exact outcome:

- dropped;
- rejected;
- soft-failed;
- needs dependencies;
- budget exceeded;
- commit failed.

### State

Specify which state:

- state before;
- state after;
- current room state;
- resolved state;
- client-side state;
- durable runtime state.

### Runtime

Qualify the layer:

- event-application runtime;
- transaction runtime;
- command runtime;
- server runtime.

### SDK

Qualify client or server:

- Matrix client SDK;
- CyborgHeart server SDK.

Do not confuse `matrix-js-sdk`, a client SDK, with the future CyborgHeart server SDK.

### Event source

Prefer `provenance` when describing how a candidate entered the system.

Use `source` for storage or transport only when that distinction matters.

---

## 11. Canonical architectural sentences

Use these formulations consistently.

### Project purpose

> CyborgHeart is a ProjectCyborg runtime for deterministic Matrix room-event processing and, later, composable homeserver execution.

### Governing rule

> Matrix defines correctness. Ruma supplies protocol primitives and core algorithms. CyborgHeart supplies the missing application lifecycle and durable server execution around it.

### Event-engine responsibility

> Given one canonical candidate room event and an immutable snapshot of available room facts, evaluate the Matrix room-event lifecycle and return a deterministic result.

### Dependency model

> The engine reports missing immutable facts; the host decides how to obtain them.

### Persistence boundary

> Event application returns semantic consequences. The transaction runtime translates them into an atomic commit.

### Local-event rule

> Locally constructed and remotely received events converge on one authoritative event-application path.

### Support rule

> A room version is supported only when it is explicitly admitted, mapped to the specification and Ruma, and proven by conformance and adversarial fixtures.

---

## 12. Maintenance rule

This lexicon is part of the architecture.

When a new public term is introduced, or an existing term changes meaning:

1. update this document;
2. update affected public APIs and documentation;
3. update fixtures and decision codes where relevant;
4. record an ADR when the change affects layer ownership or compatibility;
5. remove conflicting terminology rather than preserving synonyms indefinitely.

Terms should become more precise as CyborgHeart matures, not more numerous.

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

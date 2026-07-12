# CyborgHeart Manifesto

**Status:** Foundational principles
**Scope:** CyborgHeart
**Last reviewed:** 2026-07-12

CyborgHeart exists to make Matrix server construction understandable, composable, and correct from the inside out.

Matrix already defines the protocol. Ruma already provides essential Rust types, cryptographic operations, room-version rules, authorization, and state resolution. The missing opportunity is not another copy of those foundations. It is a clear application layer that turns them into a complete, deterministic room-event lifecycle and, from there, a durable and composable server runtime.

CyborgHeart begins at that boundary.

## Meaning before machinery

A room event has protocol meaning before it has a database row, an HTTP response, a retry policy, or a sync update.

We therefore separate:

```text
what an event means
from
how a server stores, transports, retries, and exposes it
```

The core must evaluate an event without hidden dependence on where facts were obtained or how consequences will later be committed. When Matrix makes provenance relevant, provenance must be supplied explicitly rather than inferred from transport state.

Infrastructure serves protocol meaning. It must not define it.

## Correctness is the product

CyborgHeart is not built around the fastest path to a runnable homeserver. It is built around the smallest layer that can make a homeserver trustworthy.

Correctness means more than returning success or failure. It means preserving the distinctions the protocol requires:

- incomplete evaluation is not rejection;
- invalid input is not the same as an unauthorized event;
- rejection is not the same as soft failure;
- final Policy Server recommendation failure after the required host attempt can soft-fail an otherwise authorized event;
- budget exhaustion is not a protocol decision;
- a duplicate is a durable-runtime fact, not an event-engine fact.

When distinctions matter to Matrix, they must remain visible in our types, tests, and results.

## Determinism is a contract

Given the same event, room version, immutable facts, evaluation options, and work budget, CyborgHeart must return the same result.

No hidden network request, database lookup, clock read, random value, process state, or unordered iteration may influence protocol output.

Determinism gives us:

- replay;
- reproducibility;
- precise regression tests;
- trustworthy caching;
- safe retry;
- clear fault isolation;
- confidence that persistence has not become part of protocol semantics.

A deterministic core is not merely easier to test. It is the basis of every reliable layer above it.

## Dependencies must be explicit

Missing information is normal in a distributed system.

The engine must not hide dependency acquisition behind an interface that looks pure while performing I/O underneath. It must report the immutable facts it requires and allow the host to provide them.

The model is:

```text
evaluate
    ↓
request missing facts
    ↓
supply facts
    ↓
evaluate again
```

This keeps federation, storage, caching, retries, and operational policy outside the protocol core.

It also makes incompleteness observable instead of disguising it as failure.

## Bound all work

Correctness without resource discipline is not sufficient for hostile input.

Every operation influenced by untrusted events or graphs must be bounded, measured, and capable of halting without inventing a protocol result.

CyborgHeart treats work accounting as part of the evaluation contract.

CPU, memory, graph traversal, signature checks, state inspection, and canonical data size are not incidental implementation concerns. They are part of building a server that remains correct under pressure.

## Compose; do not reimplement

CyborgHeart does not seek ownership of every Matrix primitive.

Where Ruma already provides protocol types, canonical JSON, hashing, signing, authorization, room-version behavior, or state resolution, we use and test those capabilities rather than create competing versions.

Our responsibility is to:

- compose the required operations in the correct lifecycle;
- expose trust progression clearly;
- preserve evidence and outcomes;
- bound evaluation;
- close gaps between algorithms and complete server behavior.

A local replacement is justified only by a reproduced defect, a regression fixture, and a deliberately removable boundary.

## One authoritative event path

After transport-specific admission and local event construction, locally created and remotely received events must converge on the same authoritative event-application path with explicit provenance.

Authentication may establish who is acting. A command layer may construct and sign an event. Federation may deliver it. Import and replay may provide it through other means.

None of these entry points may quietly redefine Matrix authorization.

There must be one place where room-event meaning is decided.

## Separate authority from policy

Matrix protocol rules and product policy are not the same thing.

Membership, power levels, event authorization, state resolution, and room-version behavior belong to the protocol layer.

Account suspension, spam controls, server allowlists, media quarantine, operational limits, and product moderation belong to higher layers.

Product policy may decide whether to admit a command, expose, relay, or act on something. It must not silently rewrite what Matrix says the event means.

A Matrix Policy Server is a protocol mechanism, not product policy. Its recommendation is an explicit protocol fact and does not replace Matrix authorization.

## Small boundaries before broad frameworks

CyborgHeart will not begin as a homeserver framework, plugin platform, database abstraction, or universal SDK.

Each layer must earn its existence by solving a proven problem above a stable lower contract.

The intended progression is:

```text
event application
        ↓
durable transaction runtime
        ↓
command runtime
        ↓
server capabilities
        ↓
server SDK
        ↓
homeserver products
```

This sequence is directional, not dogmatic. The enduring rule is that abstractions should emerge from demonstrated boundaries, not anticipated complexity.

## Conformance before claims

Support is not declared because a type can represent a value or because an upstream dependency exposes an API.

A Matrix rule or room version is supported only when it is:

- deliberately admitted by the public contract;
- mapped to the specification;
- mapped to the Ruma behavior we rely on;
- exercised by conformance fixtures;
- tested against malformed and adversarial cases;
- covered by regression tests when defects are found.

Documentation, fixtures, and code must describe the same supported surface.

We prefer a narrow claim that is proven over a broad claim that is merely plausible.

## Hostile input, calm design

Incoming protocol data is untrusted.

CyborgHeart should respond to malformed, incomplete, contradictory, oversized, and adversarial input through explicit results and bounded halts—not panics, accidental allocation, hidden retries, or undefined behavior.

Security should emerge from the architecture:

- immutable inputs;
- no ambient authority;
- no hidden I/O;
- bounded work;
- strict dependency direction;
- explicit outcomes;
- minimized public surface;
- regression fixtures for every discovered failure.

## Public APIs are promises

A public type is not just convenient access to an internal structure. It is a commitment to downstream users and future layers.

CyborgHeart keeps lifecycle plumbing private until an external consumer truly needs it. Public contracts should express stable domain meaning, not temporary implementation shape.

The codebase should remain separable even while it lives in one repository:

- lower layers never depend on upper layers;
- crates communicate through public APIs;
- test utilities do not become privileged backdoors;
- storage and transport types do not leak into protocol meaning;
- repository convenience does not become architectural coupling.

## Evidence over confidence

When CyborgHeart makes a decision, it should be possible to understand why.

Results should carry structured evidence about completed stages, verification, authorization context, state resolution, dependencies, and work consumed.

Logs are useful, but logs are not the correctness model.

We want failures that can be reproduced, decisions that can be inspected, and regressions that can be retained permanently.

## The long-term promise

CyborgHeart aims to become a reusable server core beneath multiple Matrix homeservers and products.

Its value will not come from owning every concern. It will come from making the central concerns precise:

- protocol meaning;
- durable execution;
- authenticated intent;
- composable server capabilities.

As the project grows, the pure event core must remain understandable without the rest of the server. The runtime must remain usable without a particular product. The SDK must compose proven parts rather than conceal a monolith.

## Our standard

We choose:

- correctness over convenience;
- explicitness over hidden behavior;
- determinism over ambient state;
- bounded work over optimistic assumptions;
- composition over reinvention;
- conformance over unsupported claims;
- narrow, proven scope over premature breadth;
- durable boundaries over fashionable architecture.

CyborgHeart should be small where smallness protects meaning, strict where strictness protects interoperability, and modular where modularity protects the future.

That is the heart we intend to build.

---

CyborgHeart is an independent ProjectCyborg project. It is not affiliated with, endorsed by, or maintained by The Matrix.org Foundation. “Matrix” is used only to describe protocol compatibility.

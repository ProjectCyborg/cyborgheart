# Security Policy

CyborgHeart processes hostile protocol input and treats security as part of correctness.

## Current support status

CyborgHeart is in contract-seeding and currently claims no supported Matrix room version, deployable homeserver, or maintained binary release.

When maintained releases exist, this section will list supported versions, security-support windows, and any end-of-support dates.

## Reporting a vulnerability

Do not disclose exploitable details in a public issue, discussion, pull request, fixture, commit message, or chat channel.

Use the first available private path:

1. the repository's GitHub private vulnerability-reporting or security-advisory feature;
2. the private security contact published in repository or organization metadata;
3. if neither exists, a minimal public issue asking maintainers to establish private contact, without vulnerability details or identifying information.

Before a public launch or maintained release, maintainers must configure and test a durable private security channel. This seed policy deliberately does not invent an address that is not yet operated.

Include, when available:

- affected commit, package, or document;
- impact and attack preconditions;
- a minimized reproduction;
- malformed event, state, dependency, or graph shape;
- observed CPU, memory, recursion, or traversal behavior;
- confidentiality, integrity, or availability effect;
- suggested mitigation;
- whether the issue is already public.

Use synthetic identifiers and keys. Never send real access tokens, production private keys, personal data, or unrelated secrets.

Reports about community conduct belong under [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md), not this process, unless they also expose a security risk.

## Security-sensitive areas

Reports are especially valuable for:

- signature, content-hash, reference-hash, event-identity, or signing-key-validity bypass;
- authorization or state-resolution divergence;
- incorrect dropped, rejected, soft-failed, accepted, dependency, or halt classification;
- Matrix Policy Server recommendation bypass or signature confusion;
- canonical JSON or Base64 inconsistencies;
- redaction-partner handling that leaks or hides content incorrectly;
- unbounded CPU, memory, recursion, or graph traversal;
- panic, abort, unsafe behavior, or nondeterminism reachable through untrusted input;
- dependency-snapshot substitution, stale-state use, or conflicting-fact acceptance;
- secret leakage through evidence, diagnostics, fixtures, or logs;
- duplicate, idempotency, atomicity, crash-recovery, or outbox defects once the transaction runtime exists;
- supply-chain or build-reproducibility weaknesses affecting released artifacts.

## Response process

Maintainers should:

1. acknowledge the report privately;
2. reproduce the issue and assess affected scope;
3. limit access to people needed for remediation;
4. establish a coordinated disclosure plan;
5. create a minimized private regression;
6. prepare the fix, fixtures, and affected document updates;
7. coordinate a release or mitigation where applicable;
8. retain a safe public regression after disclosure.

No response-time guarantee is made before a maintained release process exists. Once releases are supported, the project should publish target acknowledgement and update intervals here.

## Disclosure

Please allow maintainers a reasonable opportunity to investigate and correct the issue before public disclosure.

CyborgHeart will aim to credit reporters who request credit, subject to consent and legal or safety constraints.

## Security architecture

The governing expectations are in:

- [`ARCHITECTURE.md`](./docs/foundational/ARCHITECTURE.md)
- [`EVENT-LIFECYCLE.md`](./docs/foundational/EVENT-LIFECYCLE.md)
- [`DEPENDENCY-MODEL.md`](./docs/foundational/DEPENDENCY-MODEL.md)
- [`SUPPORTED-SURFACE.md`](./docs/foundational/SUPPORTED-SURFACE.md)

The central controls are explicit immutable inputs, no hidden I/O, bounded work, deterministic results, narrow public APIs, strict dependency direction, and permanent regression evidence.

## Third-party vulnerabilities

Report a defect in Ruma, Rust, or another dependency to its upstream security process where appropriate. Also report it privately to CyborgHeart when the candidate or pinned dependency set, documentation, fixtures, or integration is affected.

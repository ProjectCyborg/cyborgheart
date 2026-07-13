# CyborgHeart Documentation Guide

This page is a non-normative navigation aid. The linked foundational documents, ADRs, fixtures, code, and repository policies remain authoritative.

## Understand the project

Start with:

1. [`foundational/MANIFESTO.md`](./foundational/MANIFESTO.md)
2. [`foundational/WORKING-THESIS.md`](./foundational/WORKING-THESIS.md)
3. [`foundational/ARCHITECTURE.md`](./foundational/ARCHITECTURE.md)
4. [`foundational/SUPPORTED-SURFACE.md`](./foundational/SUPPORTED-SURFACE.md)

The current Matrix support claim is **none**. Room version 12 is an implementation target, not a supported compatibility claim.

## Make a documentation-only correction

Read:

- [`../CONTRIBUTING.md`](../CONTRIBUTING.md)
- the document being changed
- [`foundational/SUPPORTED-SURFACE.md`](./foundational/SUPPORTED-SURFACE.md) when wording could imply protocol support

Run:

```bash
python3 scripts/check-markdown-links.py
./scripts/verify.sh --quick
```

## Work on Rust contract types

Read:

- [`foundational/ARCHITECTURE.md`](./foundational/ARCHITECTURE.md)
- [`foundational/DEPENDENCY-MODEL.md`](./foundational/DEPENDENCY-MODEL.md)
- [`foundational/DECISION-CODES.md`](./foundational/DECISION-CODES.md)
- [`../REVIEWING.md`](../REVIEWING.md)

The event-application crate currently exposes domain vocabulary, not a callable evaluator.

## Author or review a fixture

Read:

- [`../fixtures/README.md`](../fixtures/README.md)
- [`../fixtures/schema/fixture.schema.json`](../fixtures/schema/fixture.schema.json)
- [`foundational/SPECIFICATION-MAP.md`](./foundational/SPECIFICATION-MAP.md)
- [`foundational/DECISION-CODES.md`](./foundational/DECISION-CODES.md)

Fixtures are executable protocol claims. Adding a fixture does not widen Matrix support by itself.

## Review Matrix or Ruma-sensitive behavior

Read:

- [`../REVIEWING.md`](../REVIEWING.md)
- [`foundational/SUPPORTED-SURFACE.md`](./foundational/SUPPORTED-SURFACE.md)
- [`foundational/SPECIFICATION-MAP.md`](./foundational/SPECIFICATION-MAP.md)
- [`foundational/RUMA-COVERAGE.md`](./foundational/RUMA-COVERAGE.md)
- applicable files under [`adr/`](./adr/)

Matrix v1.19 and room-version-12 rules are normative. Peer homeserver behavior is comparative evidence only.

## Architecture decisions

Focused decisions live under [`adr/`](./adr/). The foundational set remains intentionally closed; add an ADR only when the decision is costly to reverse or likely to be reopened.
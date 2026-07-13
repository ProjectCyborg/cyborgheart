# CyborgHeart Agent Guide

This is the public, authoritative implementation repository.

## Before changing files

1. Read [`README.md`](./README.md) and the applicable public policy or foundational documents.
2. Confirm the requested change belongs in this repository.
3. Check the current branch and repository status.
4. Keep changes focused and preserve the downward dependency direction.

## Current boundary

- Matrix support claim: **none**.
- Matrix baseline: v1.19.
- Initial target: room version 12 only.
- The event-application crate currently exposes contract vocabulary, not a callable evaluator.
- Private project-operations material cannot define or override public behavior.

## Branch flow

Normal changes target `development` through a pull request. Only the repository's `development` branch promotes into `main`.

## Validation

Run the quick gate for ordinary work:

```bash
./scripts/verify.sh --quick
```

Run the full verifier for public API, dependencies, fixtures, support claims, security, CI, release, documentation-contract, or platform-sensitive work:

```bash
./scripts/verify.sh
```

## Stop and report

Stop before proceeding when:

- a protocol claim lacks a Matrix source and executable evidence;
- a change would widen support without the promotion gate;
- storage, network, async, clock, randomness, or product policy enters the pure event engine;
- a private plan conflicts with public code or documentation;
- unrelated work would be modified or staged.

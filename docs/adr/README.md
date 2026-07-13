# Architecture Decision Records

This directory stores focused Architecture Decision Records for CyborgHeart.

ADRs preserve decisions that materially affect:

- layer ownership;
- public contracts;
- Matrix or room-version support;
- Ruma integration or workarounds;
- significant runtime or infrastructure dependencies;
- future separability.

They do not replace the foundational documents. The foundational pack defines the current architecture; ADRs record why consequential choices were made and when they were superseded.

## Status values

```text
Proposed
Accepted
Superseded
Rejected
Deprecated
```

## File naming

```text
NNNN-short-decision-name.md
```

Numbers are assigned sequentially and never reused.

## Required structure

```markdown
# ADR NNNN: Decision title

**Status:** Accepted
**Date:** YYYY-MM-DD

## Context
## Decision
## Consequences
## Alternatives considered
## Reconsider when
## Related documents
```

## Decision rule

Create an ADR only when reversing the decision later would be materially costly or when future maintainers are likely to reopen the same question.

Do not create ADRs for routine implementation choices, temporary issue ordering, or easily reversible refactors.

## Initial ADRs

- [`0001-single-workspace.md`](./0001-single-workspace.md)
- [`0002-room-version-12-only.md`](./0002-room-version-12-only.md)
- [`0003-ruma-boundary.md`](./0003-ruma-boundary.md)
- [`0004-structural-public-contract-invariants.md`](./0004-structural-public-contract-invariants.md)
- [`0005-pre-release-contract-corrections.md`](./0005-pre-release-contract-corrections.md)

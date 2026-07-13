# CyborgHeart Versioning Policy

CyborgHeart uses one lockstep workspace version during the initial pre-1.0 phase.

Current workspace version:

```text
0.1.0
```

Current packages:

```text
cyborg-heart-event-application
cyborg-heart-event-testkit
```

Both packages inherit the workspace version and remain unpublished.

## Lockstep workspace version

The initial crates move together because the testkit directly validates the event-application contract and both public surfaces are still evolving as one reviewed boundary.

Keep one version for:

- `cyborg-heart-event-application`
- `cyborg-heart-event-testkit`

Reconsider independent versions only after the crates have independent users, release cadence, or compatibility promises.

## Pre-1.0 compatibility

CyborgHeart follows Cargo's pre-1.0 convention:

- `0.y.z` changes to `0.(y+1).0` for breaking compatibility changes.
- `0.y.z` changes to `0.y.(z+1)` for compatible release changes.
- `0.0.z` is avoided because Cargo treats every release in that range as incompatible.

Before 1.0, classify changes this way:

| Change | Version effect |
|---|---|
| Breaking public API, stable code meaning, fixture-schema contract, supported behavior, MSRV, or platform baseline | Minor increment |
| Compatible public addition, new fixture family, implementation progress, or support-surface expansion within the existing contract | Patch increment |
| Compatible bug fix, documentation correction, test improvement, or tooling fix included in a release | Patch increment |
| Internal-only change with no released artifact impact | May remain unreleased until grouped |

A behavior correction can still be breaking when downstream users could have relied on the previous public result, stable code, fixture schema, or documented support statement.

## Support claims are separate

Package versions do not define Matrix compatibility.

Release notes must repeat the exact support state from [`SUPPORTED-SURFACE.md`](./docs/foundational/SUPPORTED-SURFACE.md). A version number never implies support for a room version, API, lifecycle stage, federation behavior, homeserver behavior, or Matrix feature beyond that document.

## Publication status

The workspace remains unpublished:

```toml
[workspace.package]
publish = false
```

Both initial crates inherit that setting through `publish.workspace = true`.

Do not publish to crates.io until a separate publish-readiness decision covers package ownership, metadata, documentation quality, API maturity, support and security commitments, release recovery, and credential or trusted-publishing configuration.

## Release execution

This policy does not create release automation.

Until a separate reviewed release cut exists:

- do not create `v*` tags;
- do not create GitHub releases;
- do not add crates.io publishing;
- do not add release credentials;
- do not open bot release PRs directly to `main`.

Future release preparation must target `development`. Only the repository's `development` branch may promote into `main`, and tags may only identify validated `main` commits after the release path is implemented and tested.

## Version-change review

A version-change pull request must state:

- why the version changes;
- whether the change is breaking or compatible under this policy;
- whether public API, stable codes, fixture schema, MSRV, platform baseline, or support claims changed;
- validation run;
- confirmation that both crates remain unpublished unless a separate publish-readiness decision has been accepted.

Version changes do not automatically change Matrix support claims.

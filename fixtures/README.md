# CyborgHeart Fixture Contract

**Schema version:** 1  
**Schema:** [`schema/fixture.schema.json`](./schema/fixture.schema.json)

---

## Purpose

Fixtures are executable protocol claims.

Each fixture records one principal behavior with enough context to trace it to Matrix, construct explicit inputs, and compare stable CyborgHeart results.

The fixture corpus is not a collection of arbitrary JSON examples.

---

## Layout

```text
fixtures/
├── README.md
├── schema/
│   └── fixture.schema.json
└── seed/
    ├── input/
    ├── capability/
    ├── room-v12/format/
    ├── signing/
    └── budget/
```

This directory is the canonical fixture root for the public repository and must retain this layout.

---

## Required authoring rules

1. Use one principal expected behavior per fixture.
2. Give every fixture a globally unique lowercase ID.
3. Cite at least one normative Matrix URL.
4. Record provenance and license.
5. Use `raw-json` only for pre-engine adapter behavior.
6. Use `canonical-object` for engine behavior.
7. Never model missing facts as invalid protocol input.
8. Never assert unstable Ruma error text.
9. Set conservative work ceilings.
10. Add a review note when a classification is subtle.

---

## Authoring checklist

Before opening a fixture pull request:

- [ ] keep `fixture_schema_version` at the version required by [`fixture.schema.json`](./schema/fixture.schema.json);
- [ ] choose a globally unique fixture ID and compare it with the existing [`seed/`](./seed/) corpus;
- [ ] use synthetic domains, users, keys, signatures, and event identifiers only;
- [ ] set `status` and `minimum_stage` honestly so unavailable behavior remains staged rather than falsely passing;
- [ ] cite the applicable Matrix v1.19 and room-version references;
- [ ] use only registered expected decision, dependency, and consequence codes;
- [ ] declare conservative input budgets and expected work ceilings;
- [ ] run `cargo test -p cyborg-heart-event-testkit --locked` and `./scripts/verify.sh --quick`.

A new fixture records an executable claim. It does not expand the repository's Matrix support claim by itself; support changes only through the gate in [`SUPPORTED-SURFACE.md`](../docs/foundational/SUPPORTED-SURFACE.md).

---

## Discovery

The testkit must:

- recursively discover `*.json` fixture files under `fixtures/seed/`;
- sort paths lexicographically before loading;
- validate each file against schema version 1;
- reject duplicate fixture IDs;
- retain the source path for diagnostics;
- produce deterministic error ordering.

---

## Loading sequence

```text
read bytes
    ↓
parse fixture JSON
    ↓
validate fixture schema
    ↓
parse typed fixture metadata
    ↓
apply input adapter when required
    ↓
construct normalized facts and budget
    ↓
execute only when minimum stage is available
    ↓
compare stable expected fields
```

A fixture whose minimum stage is unavailable is reported as staged, not passed.

---

## Candidate input

### Raw JSON

`input.mode = "raw-json"`

Required:

```json
{
  "raw_json": "{...}"
}
```

`candidate` must be absent.

### Canonical object

`input.mode = "canonical-object"`

Required:

```json
{
  "candidate": {}
}
```

`raw_json` must be absent.

The testkit converts the object through Ruma canonical JSON. A fixture file being valid JSON does not mean its candidate values are valid Matrix canonical JSON.

---

## Budget dimensions

Version 1 recognizes:

```text
canonical_json_bytes
events_inspected
state_entries_inspected
auth_events_inspected
auth_chain_edges
graph_edges
signature_verifications
state_resolution_steps
```

Unknown dimensions are rejected so misspellings cannot silently disable a test.

---

## Expected work

`expect.work.max` is an upper bound, not an exact operation count.

Exact counts may be asserted later only when they are part of a deliberate public resource contract.

---

## Fixture maturity

The initial seed fixtures target `verified-intake`. They may be loaded and schema-validated in the first PR before that lifecycle stage is executable.

Once a fixture's minimum stage exists, it becomes a required semantic test and may not be skipped in CI.

---

## Provenance

For authored fixtures:

```json
{
  "kind": "authored",
  "source": "CyborgHeart project seed",
  "license": "MIT OR Apache-2.0"
}
```

Imported fixtures must preserve their original source and license. Do not copy implementation test data without confirming reuse terms.

---

## Policy Server recommendation facts

A Policy Server recommendation fact never contains a host-asserted `valid` or `invalid` flag.

It contains one of:

- `outcome.kind = "signature-material"` with the server, key ID, and signature bytes that the event engine must validate itself; or
- `outcome.kind = "final-unavailable"` after the required acquisition attempt completed without usable material.

The fact remains bound to the candidate event, active policy event, and frozen policy fingerprint. Signing keys remain separate immutable facts.

---

## Updating a fixture

A fixture expectation change requires:

- a specification or architecture reason;
- review of the decision code;
- review of adjacent fixtures;
- a fixture-contract review;
- a regression note if behavior previously shipped.

Changing an expectation merely to make a failing implementation pass is prohibited.

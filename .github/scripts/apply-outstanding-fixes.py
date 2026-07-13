#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement target, found {count}")
    path.write_text(text.replace(old, new), encoding="utf-8")


def ensure_newline(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    path.write_text(text.rstrip("\n") + "\n", encoding="utf-8")


def patch_decision_contract() -> None:
    path = ROOT / "crates/event-application/src/decision.rs"
    replace_once(
        path,
        "//! Result, disposition, halt, fault, and stable decision-code types.\n\n",
        "//! Result, disposition, halt, fault, and stable decision-code types.\n\n"
        "use std::{\n"
        "    error::Error,\n"
        "    fmt::{self, Display},\n"
        "};\n\n",
    )

    old = '''/// Bounded halt without a Matrix disposition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Halt {
    code: HaltCode,
    stage: LifecycleStage,
    dimension: WorkDimension,
    limit: u64,
    consumed: u64,
    evidence: DecisionEvidence,
}

impl Halt {
    /// Creates a bounded halt.
    #[must_use]
    pub const fn new(
        stage: LifecycleStage,
        dimension: WorkDimension,
        limit: u64,
        consumed: u64,
        evidence: DecisionEvidence,
    ) -> Self {
        Self {
            code: HaltCode::BudgetExceeded,
            stage,
            dimension,
            limit,
            consumed,
            evidence,
        }
    }

    /// Stable halt code.
    #[must_use]
    pub const fn code(&self) -> HaltCode {
        self.code
    }

    /// Lifecycle stage.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        self.stage
    }

    /// Exhausted work dimension.
    #[must_use]
    pub const fn dimension(&self) -> WorkDimension {
        self.dimension
    }

    /// Configured limit.
    #[must_use]
    pub const fn limit(&self) -> u64 {
        self.limit
    }

    /// Consumed amount at halt.
    #[must_use]
    pub const fn consumed(&self) -> u64 {
        self.consumed
    }

    /// Halt evidence.
    #[must_use]
    pub const fn evidence(&self) -> &DecisionEvidence {
        &self.evidence
    }
}
'''
    new = '''/// Error returned when a budget-exceeded halt is requested without an exceedance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBudgetExceeded {
    dimension: WorkDimension,
    limit: u64,
    consumed: u64,
}

impl InvalidBudgetExceeded {
    /// Work dimension whose values were inconsistent.
    #[must_use]
    pub const fn dimension(self) -> WorkDimension {
        self.dimension
    }

    /// Configured limit.
    #[must_use]
    pub const fn limit(self) -> u64 {
        self.limit
    }

    /// Reported consumed amount.
    #[must_use]
    pub const fn consumed(self) -> u64 {
        self.consumed
    }
}

impl Display for InvalidBudgetExceeded {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot construct {} for {}: consumed {} does not exceed limit {}",
            HaltCode::BudgetExceeded.as_str(),
            self.dimension,
            self.consumed,
            self.limit
        )
    }
}

impl Error for InvalidBudgetExceeded {}

/// Bounded halt without a Matrix disposition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Halt {
    code: HaltCode,
    stage: LifecycleStage,
    dimension: WorkDimension,
    limit: u64,
    consumed: u64,
    evidence: DecisionEvidence,
}

impl Halt {
    /// Creates a budget-exceeded halt only when consumed work exceeds the limit.
    pub const fn budget_exceeded(
        stage: LifecycleStage,
        dimension: WorkDimension,
        limit: u64,
        consumed: u64,
        evidence: DecisionEvidence,
    ) -> Result<Self, InvalidBudgetExceeded> {
        if consumed <= limit {
            return Err(InvalidBudgetExceeded {
                dimension,
                limit,
                consumed,
            });
        }

        Ok(Self {
            code: HaltCode::BudgetExceeded,
            stage,
            dimension,
            limit,
            consumed,
            evidence,
        })
    }

    /// Stable halt code.
    #[must_use]
    pub const fn code(&self) -> HaltCode {
        self.code
    }

    /// Lifecycle stage.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        self.stage
    }

    /// Exhausted work dimension.
    #[must_use]
    pub const fn dimension(&self) -> WorkDimension {
        self.dimension
    }

    /// Configured limit.
    #[must_use]
    pub const fn limit(&self) -> u64 {
        self.limit
    }

    /// Consumed amount at halt.
    #[must_use]
    pub const fn consumed(&self) -> u64 {
        self.consumed
    }

    /// Halt evidence.
    #[must_use]
    pub const fn evidence(&self) -> &DecisionEvidence {
        &self.evidence
    }
}
'''
    replace_once(path, old, new)

    test = '''
    #[test]
    fn budget_halt_requires_an_actual_exceedance() {
        let error = Halt::budget_exceeded(
            LifecycleStage::PduFormat,
            WorkDimension::EventsInspected,
            5,
            5,
            evidence(),
        )
        .expect_err("equal consumption is not an exceedance");
        assert_eq!(error.dimension(), WorkDimension::EventsInspected);
        assert_eq!(error.limit(), 5);
        assert_eq!(error.consumed(), 5);

        let halt = Halt::budget_exceeded(
            LifecycleStage::PduFormat,
            WorkDimension::EventsInspected,
            5,
            6,
            evidence(),
        )
        .expect("consumption above the limit creates a halt");
        assert_eq!(halt.code(), HaltCode::BudgetExceeded);
        assert_eq!(halt.limit(), 5);
        assert_eq!(halt.consumed(), 6);
    }
'''
    text = path.read_text(encoding="utf-8")
    if test.strip() in text:
        raise SystemExit(f"{path}: budget halt test already present")
    index = text.rfind("}\n")
    if index < 0:
        raise SystemExit(f"{path}: could not locate test-module terminator")
    path.write_text(text[:index] + test + text[index:], encoding="utf-8")

    lib = ROOT / "crates/event-application/src/lib.rs"
    replace_once(
        lib,
        "    EvaluationResult, FaultCode, Halt, HaltCode, InputCode, LifecycleStage, RejectionCode,\n",
        "    EvaluationResult, FaultCode, Halt, HaltCode, InputCode, InvalidBudgetExceeded,\n"
        "    LifecycleStage, RejectionCode,\n",
    )


def patch_fixture_schema() -> None:
    path = ROOT / "fixtures/schema/fixture.schema.json"
    schema = json.loads(path.read_text(encoding="utf-8"))
    options = schema["$defs"]["input"]["properties"]["options"]
    options["required"] = ["evidence_level"]
    schema["$defs"]["policy_recommendation"] = {
        "type": "object",
        "additionalProperties": False,
        "required": [
            "event_id",
            "policy_event_id",
            "policy_fingerprint",
            "outcome",
        ],
        "properties": {
            "event_id": {"type": "string", "minLength": 1},
            "policy_event_id": {"type": "string", "minLength": 1},
            "policy_fingerprint": {"type": "string", "minLength": 1},
            "outcome": {
                "oneOf": [
                    {
                        "type": "object",
                        "additionalProperties": False,
                        "required": ["kind", "server", "key_id", "signature"],
                        "properties": {
                            "kind": {"const": "signature-material"},
                            "server": {"type": "string", "minLength": 1},
                            "key_id": {"type": "string", "minLength": 1},
                            "signature": {"type": "string", "minLength": 1},
                        },
                    },
                    {
                        "type": "object",
                        "additionalProperties": False,
                        "required": ["kind"],
                        "properties": {"kind": {"const": "final-unavailable"}},
                    },
                ]
            },
        },
    }
    path.write_text(json.dumps(schema, indent=2) + "\n", encoding="utf-8")


def fixture_model_block() -> str:
    return r'''/// Fixture lifecycle maturity.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureStage {
    /// Contract-stage fixture loading and result-model checks.
    Contract,
    /// Verified intake stage.
    VerifiedIntake,
    /// Linear event-application stage.
    LinearApplication,
    /// Complete room-version-12 stage.
    CompleteV12,
}

/// Fixture authoring status.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureStatus {
    /// Active fixture.
    Active,
    /// Planned fixture, not yet active.
    Planned,
}

/// Whether a fixture is executable for an implemented stage.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureExecutionState {
    /// Fixture is active and its minimum stage is available.
    Active,
    /// Fixture is planned and must not be counted as passed.
    Planned,
    /// Fixture is active but its minimum stage is unavailable.
    Staged {
        /// Minimum stage required by the fixture.
        minimum_stage: FixtureStage,
    },
}

/// Expected fixture result kind.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExpectedKind {
    /// Input adapter error.
    InputError,
    /// Capability boundary error.
    CapabilityError,
    /// Complete event decision.
    Complete,
    /// Missing dependencies.
    NeedsDependencies,
    /// Bounded halt.
    Halted,
    /// Host or implementation fault.
    Fault,
}

/// Complete-decision disposition in fixture expectations.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExpectedDisposition {
    /// Accepted event.
    Accepted,
    /// Dropped event.
    Dropped,
    /// Rejected event.
    Rejected,
    /// Soft-failed event.
    SoftFailed,
}

/// Loaded fixture suite in deterministic path order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureSuite {
    fixtures: Vec<Fixture>,
}

impl FixtureSuite {
    /// Loads seed fixtures from a repository root.
    pub fn load_seed_from_repo_root(repo_root: impl AsRef<Path>) -> Result<Self, FixtureError> {
        let repo_root = repo_root.as_ref();
        let schema_path = repo_root.join("fixtures/schema/fixture.schema.json");
        let seed_root = repo_root.join("fixtures/seed");
        let paths = discover_json_files(&seed_root)?;
        load_fixtures_from_paths(&schema_path, paths)
    }

    /// Loaded fixtures in deterministic order.
    #[must_use]
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }

    /// Fixture IDs in deterministic fixture order.
    #[must_use]
    pub fn ids(&self) -> Vec<&str> {
        self.fixtures.iter().map(Fixture::id).collect()
    }
}

/// One schema-valid fixture with complete typed contract retention.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Fixture {
    #[serde(skip)]
    source_path: PathBuf,
    fixture_schema_version: u64,
    id: String,
    title: String,
    description: String,
    status: FixtureStatus,
    minimum_stage: FixtureStage,
    matrix: MatrixMetadata,
    provenance: FixtureProvenance,
    input: FixtureInput,
    facts: FixtureFacts,
    budget: BTreeMap<String, u64>,
    expect: FixtureExpectation,
    #[serde(default)]
    notes: Vec<String>,
}

impl Fixture {
    /// Source path retained for diagnostics.
    #[must_use]
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    /// Declared fixture schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u64 {
        self.fixture_schema_version
    }

    /// Stable fixture ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Human-readable fixture title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Human-readable fixture description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Fixture status.
    #[must_use]
    pub const fn status(&self) -> FixtureStatus {
        self.status
    }

    /// Minimum lifecycle stage required by this fixture.
    #[must_use]
    pub const fn minimum_stage(&self) -> FixtureStage {
        self.minimum_stage
    }

    /// Matrix metadata.
    #[must_use]
    pub const fn matrix(&self) -> &MatrixMetadata {
        &self.matrix
    }

    /// Source provenance and licensing metadata.
    #[must_use]
    pub const fn provenance(&self) -> &FixtureProvenance {
        &self.provenance
    }

    /// Fixture input metadata.
    #[must_use]
    pub const fn input(&self) -> &FixtureInput {
        &self.input
    }

    /// Immutable facts supplied to future evaluation.
    #[must_use]
    pub const fn facts(&self) -> &FixtureFacts {
        &self.facts
    }

    /// Declared evaluation budget.
    #[must_use]
    pub const fn budget(&self) -> &BTreeMap<String, u64> {
        &self.budget
    }

    /// Expected result metadata.
    #[must_use]
    pub const fn expectation(&self) -> &FixtureExpectation {
        &self.expect
    }

    /// Optional author and reviewer notes.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns this fixture's execution state for the highest implemented stage.
    #[must_use]
    pub fn execution_state(&self, implemented: FixtureStage) -> FixtureExecutionState {
        match (self.status, self.minimum_stage <= implemented) {
            (FixtureStatus::Planned, _) => FixtureExecutionState::Planned,
            (FixtureStatus::Active, true) => FixtureExecutionState::Active,
            (FixtureStatus::Active, false) => FixtureExecutionState::Staged {
                minimum_stage: self.minimum_stage,
            },
        }
    }

    /// Converts the fixture input into Ruma canonical JSON when possible.
    pub fn canonical_candidate(&self) -> Result<CanonicalJsonObject, InputAdapterError> {
        self.input.canonical_candidate()
    }
}

/// Matrix metadata attached to a fixture.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MatrixMetadata {
    spec_version: String,
    room_version: String,
    references: Vec<String>,
}

impl MatrixMetadata {
    /// Matrix specification version declared by the fixture.
    #[must_use]
    pub fn spec_version(&self) -> &str {
        &self.spec_version
    }

    /// Matrix room version declared by the fixture.
    #[must_use]
    pub fn room_version(&self) -> &str {
        &self.room_version
    }

    /// Normative references declared by the fixture.
    #[must_use]
    pub fn references(&self) -> &[String] {
        &self.references
    }
}

/// Provenance kind for fixture source material.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureSourceKind {
    /// Authored by the CyborgHeart project.
    Authored,
    /// Derived from the Matrix specification.
    MatrixSpec,
    /// Derived from Ruma behavior or vectors.
    Ruma,
    /// Added from a project regression.
    Regression,
}

/// Source and licensing metadata for a fixture.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FixtureProvenance {
    kind: FixtureSourceKind,
    source: String,
    license: String,
}

impl FixtureProvenance {
    /// Fixture source kind.
    #[must_use]
    pub const fn kind(&self) -> FixtureSourceKind {
        self.kind
    }

    /// Human-readable source description.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// License or reuse terms.
    #[must_use]
    pub fn license(&self) -> &str {
        &self.license
    }
}

/// How the candidate entered the host system in a fixture.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureInputProvenance {
    /// Received from a remote Matrix server.
    Federated,
    /// Constructed locally by a future command runtime.
    LocallyConstructed,
    /// Re-evaluated from retained data.
    Replay,
    /// Supplied by an import or migration path.
    Import,
}

impl FixtureInputProvenance {
    /// Maps fixture vocabulary into the public event-application contract.
    #[must_use]
    pub const fn event_provenance(self) -> EventProvenance {
        match self {
            Self::Federated => EventProvenance::Federated,
            Self::LocallyConstructed => EventProvenance::LocallyConstructed,
            Self::Replay => EventProvenance::Replay,
            Self::Import => EventProvenance::Import,
        }
    }
}

/// Evidence volume selected by a fixture.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureEvidenceLevel {
    /// Standard public evidence.
    Standard,
}

/// Evaluation options retained from a fixture.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FixtureEvaluationOptions {
    evidence_level: FixtureEvidenceLevel,
}

impl FixtureEvaluationOptions {
    /// Requested evidence level.
    #[must_use]
    pub const fn evidence_level(&self) -> FixtureEvidenceLevel {
        self.evidence_level
    }
}

/// Fixture candidate input and complete explicit evaluation metadata.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FixtureInput {
    mode: InputMode,
    raw_json: Option<String>,
    candidate: Option<Map<String, Value>>,
    asserted_event_id: Option<String>,
    room_version: String,
    provenance: FixtureInputProvenance,
    key_validity_reference_ts: u64,
    options: Option<FixtureEvaluationOptions>,
}

impl FixtureInput {
    /// Input mode.
    #[must_use]
    pub const fn mode(&self) -> InputMode {
        self.mode
    }

    /// Raw JSON string for raw-json fixtures.
    #[must_use]
    pub fn raw_json(&self) -> Option<&str> {
        self.raw_json.as_deref()
    }

    /// Candidate object for canonical-object fixtures.
    #[must_use]
    pub const fn candidate(&self) -> Option<&Map<String, Value>> {
        self.candidate.as_ref()
    }

    /// Optional externally asserted event ID.
    #[must_use]
    pub fn asserted_event_id(&self) -> Option<&str> {
        self.asserted_event_id.as_deref()
    }

    /// External room version in the fixture input.
    #[must_use]
    pub fn room_version(&self) -> &str {
        &self.room_version
    }

    /// Candidate provenance.
    #[must_use]
    pub const fn provenance(&self) -> FixtureInputProvenance {
        self.provenance
    }

    /// Frozen signing-key validity reference timestamp.
    #[must_use]
    pub const fn key_validity_reference_ts(&self) -> u64 {
        self.key_validity_reference_ts
    }

    /// Explicit evaluation options when provided.
    #[must_use]
    pub const fn options(&self) -> Option<&FixtureEvaluationOptions> {
        self.options.as_ref()
    }

    /// Converts the fixture input into Matrix canonical JSON.
    pub fn canonical_candidate(&self) -> Result<CanonicalJsonObject, InputAdapterError> {
        match self.mode {
            InputMode::CanonicalObject => {
                let candidate = self
                    .candidate
                    .as_ref()
                    .ok_or(InputAdapterError::MissingCandidate)?;
                canonical_object_from_map(candidate)
            }
            InputMode::RawJson => {
                let raw_json = self
                    .raw_json
                    .as_deref()
                    .ok_or(InputAdapterError::MissingRawJson)?;
                let value = serde_json::from_str::<Value>(raw_json).map_err(|error| {
                    InputAdapterError::InvalidJson {
                        message: error.to_string(),
                    }
                })?;
                let object = value.as_object().ok_or(InputAdapterError::NotAnObject)?;
                canonical_object_from_map(object)
            }
        }
    }
}

/// Fixture input mode.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputMode {
    /// Exact raw JSON string.
    RawJson,
    /// Candidate represented as a JSON object expected to be canonicalizable.
    CanonicalObject,
}

/// Disposition assigned to an immutable event fact.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FactDisposition {
    /// Previously accepted event.
    Accepted,
    /// Previously rejected event.
    Rejected,
    /// Previously soft-failed event.
    SoftFailed,
    /// Event has not yet been evaluated.
    Unevaluated,
}

/// One immutable event fact.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EventFact {
    event_id: String,
    event: Map<String, Value>,
    disposition: FactDisposition,
}

impl EventFact {
    /// Event identifier.
    #[must_use]
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Canonicalizable event object.
    #[must_use]
    pub const fn event(&self) -> &Map<String, Value> {
        &self.event
    }

    /// Previously known event disposition.
    #[must_use]
    pub const fn disposition(&self) -> FactDisposition {
        self.disposition
    }
}

/// State tuple entry retained by fixture facts.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StateEntry {
    #[serde(rename = "type")]
    event_type: String,
    state_key: String,
    event_id: String,
}

impl StateEntry {
    /// Event type in the state tuple.
    #[must_use]
    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    /// State key in the state tuple.
    #[must_use]
    pub fn state_key(&self) -> &str {
        &self.state_key
    }

    /// Event identifier stored at the tuple.
    #[must_use]
    pub fn event_id(&self) -> &str {
        &self.event_id
    }
}

/// Historical state-after fact for an event.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StateAfterFact {
    event_id: String,
    entries: Vec<StateEntry>,
}

impl StateAfterFact {
    /// Event whose state-after map is retained.
    #[must_use]
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// State map entries.
    #[must_use]
    pub fn entries(&self) -> &[StateEntry] {
        &self.entries
    }
}

/// Frozen current-room-state fact.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CurrentRoomStateFact {
    room_id: String,
    revision: String,
    entries: Vec<StateEntry>,
}

impl CurrentRoomStateFact {
    /// Room identifier.
    #[must_use]
    pub fn room_id(&self) -> &str {
        &self.room_id
    }

    /// Host-supplied immutable revision token.
    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// Current state entries.
    #[must_use]
    pub fn entries(&self) -> &[StateEntry] {
        &self.entries
    }
}

/// Server signing-key fact.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SigningKeyFact {
    server: String,
    key_id: String,
    public_key: String,
    valid_from_ts: Option<u64>,
    valid_until_ts: Option<u64>,
}

impl SigningKeyFact {
    /// Signing server.
    #[must_use]
    pub fn server(&self) -> &str {
        &self.server
    }

    /// Matrix key identifier.
    #[must_use]
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Encoded public key.
    #[must_use]
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    /// Optional validity start timestamp.
    #[must_use]
    pub const fn valid_from_ts(&self) -> Option<u64> {
        self.valid_from_ts
    }

    /// Optional validity end timestamp.
    #[must_use]
    pub const fn valid_until_ts(&self) -> Option<u64> {
        self.valid_until_ts
    }
}

/// Host-supplied result of a Policy Server recommendation acquisition attempt.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PolicyRecommendationOutcome {
    /// Signature material that the event engine must validate itself.
    SignatureMaterial {
        /// Server that supplied the recommendation signature.
        server: String,
        /// Matrix key identifier for the signature.
        key_id: String,
        /// Encoded signature bytes.
        signature: String,
    },
    /// The required acquisition attempt completed without recommendation material.
    FinalUnavailable,
}

/// Policy Server recommendation fact bound to an event and policy configuration.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PolicyRecommendationFact {
    event_id: String,
    policy_event_id: String,
    policy_fingerprint: String,
    outcome: PolicyRecommendationOutcome,
}

impl PolicyRecommendationFact {
    /// Candidate event identifier.
    #[must_use]
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Active policy event identifier.
    #[must_use]
    pub fn policy_event_id(&self) -> &str {
        &self.policy_event_id
    }

    /// Fingerprint binding the fact to the frozen policy configuration.
    #[must_use]
    pub fn policy_fingerprint(&self) -> &str {
        &self.policy_fingerprint
    }

    /// Signature material or terminal acquisition outcome.
    #[must_use]
    pub const fn outcome(&self) -> &PolicyRecommendationOutcome {
        &self.outcome
    }
}

/// Complete immutable fact snapshot retained by a fixture.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FixtureFacts {
    events: Vec<EventFact>,
    state_after: Vec<StateAfterFact>,
    current_room_state: Option<CurrentRoomStateFact>,
    signing_keys: Vec<SigningKeyFact>,
    policy_recommendations: Vec<PolicyRecommendationFact>,
}

impl FixtureFacts {
    /// Event facts.
    #[must_use]
    pub fn events(&self) -> &[EventFact] {
        &self.events
    }

    /// Historical state-after facts.
    #[must_use]
    pub fn state_after(&self) -> &[StateAfterFact] {
        &self.state_after
    }

    /// Frozen current room state when available.
    #[must_use]
    pub const fn current_room_state(&self) -> Option<&CurrentRoomStateFact> {
        self.current_room_state.as_ref()
    }

    /// Server signing keys.
    #[must_use]
    pub fn signing_keys(&self) -> &[SigningKeyFact] {
        &self.signing_keys
    }

    /// Policy Server recommendation facts.
    #[must_use]
    pub fn policy_recommendations(&self) -> &[PolicyRecommendationFact] {
        &self.policy_recommendations
    }
}

/// Fixture expected-result metadata.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FixtureExpectation {
    kind: ExpectedKind,
    code: String,
    stage: String,
    disposition: Option<ExpectedDisposition>,
    #[serde(default)]
    details: Map<String, Value>,
    dependencies: Vec<ExpectedDependency>,
    consequences: Vec<String>,
    work: ExpectedWork,
}

impl FixtureExpectation {
    /// Expected result kind.
    #[must_use]
    pub const fn kind(&self) -> ExpectedKind {
        self.kind
    }

    /// Expected stable machine code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Expected lifecycle stage.
    #[must_use]
    pub fn stage(&self) -> &str {
        &self.stage
    }

    /// Expected disposition for complete decisions.
    #[must_use]
    pub const fn disposition(&self) -> Option<ExpectedDisposition> {
        self.disposition
    }

    /// Additional stable expected details.
    #[must_use]
    pub const fn details(&self) -> &Map<String, Value> {
        &self.details
    }

    /// Expected dependency requests.
    #[must_use]
    pub fn dependencies(&self) -> &[ExpectedDependency] {
        &self.dependencies
    }

    /// Expected consequence codes.
    #[must_use]
    pub fn consequences(&self) -> &[String] {
        &self.consequences
    }

    /// Expected work ceilings.
    #[must_use]
    pub const fn work(&self) -> &ExpectedWork {
        &self.work
    }
}

/// Expected dependency request metadata.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ExpectedDependency {
    code: String,
    stage: String,
    details: Map<String, Value>,
}

impl ExpectedDependency {
    /// Expected dependency code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Stage where the dependency is expected.
    #[must_use]
    pub fn stage(&self) -> &str {
        &self.stage
    }

    /// Stable dependency-identifying details.
    #[must_use]
    pub const fn details(&self) -> &Map<String, Value> {
        &self.details
    }
}

/// Expected work ceiling metadata.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ExpectedWork {
    max: BTreeMap<String, u64>,
}

impl ExpectedWork {
    /// Maximum allowed work by dimension name.
    #[must_use]
    pub const fn max(&self) -> &BTreeMap<String, u64> {
        &self.max
    }
}

'''


def patch_testkit() -> None:
    path = ROOT / "crates/event-testkit/src/lib.rs"
    text = path.read_text(encoding="utf-8")
    import_anchor = "use ruma::{CanonicalJsonObject, CanonicalJsonValue};\n"
    if import_anchor not in text:
        raise SystemExit(f"{path}: ruma import anchor missing")
    text = text.replace(
        import_anchor,
        "use cyborg_heart_event_application::EventProvenance;\n" + import_anchor,
        1,
    )

    start = text.index("/// Fixture lifecycle maturity.\n")
    end = text.index("/// Fixture loading and validation error.\n")
    text = text[:start] + fixture_model_block() + text[end:]

    old_test = '''    #[test]
    fn all_ten_seed_fixtures_validate_and_load() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        assert_eq!(suite.fixtures().len(), 10);
        assert!(
            suite
                .fixtures()
                .iter()
                .all(|fixture| fixture.fixture_schema_version == 1)
        );
    }
'''
    new_test = '''    #[test]
    fn all_discovered_seed_fixtures_validate_and_load() {
        let discovered = discover_json_files(&repo_root().join("fixtures/seed"))
            .expect("fixture discovery succeeds");
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        assert!(!discovered.is_empty(), "seed fixture corpus must not be empty");
        assert_eq!(suite.fixtures().len(), discovered.len());
        assert!(
            suite
                .fixtures()
                .iter()
                .all(|fixture| fixture.schema_version() == SUPPORTED_FIXTURE_SCHEMA_VERSION)
        );
    }
'''
    if text.count(old_test) != 1:
        raise SystemExit(f"{path}: hard-coded fixture-count test target missing")
    text = text.replace(old_test, new_test, 1)

    added_tests = r'''
    #[test]
    fn typed_model_retains_complete_fixture_contract() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        let fixture = fixture_by_id(&suite, "capability.unsupported_room_version");

        assert_eq!(fixture.schema_version(), SUPPORTED_FIXTURE_SCHEMA_VERSION);
        assert!(!fixture.description().is_empty());
        assert_eq!(fixture.provenance().kind(), FixtureSourceKind::Authored);
        assert!(!fixture.provenance().source().is_empty());
        assert!(!fixture.provenance().license().is_empty());
        assert_eq!(
            fixture.input().provenance().event_provenance(),
            EventProvenance::Federated
        );
        assert_eq!(fixture.input().key_validity_reference_ts(), 1);
        assert!(fixture.input().options().is_some());
        assert!(fixture.facts().events().is_empty());
        assert!(fixture.facts().state_after().is_empty());
        assert!(fixture.facts().current_room_state().is_none());
        assert!(fixture.facts().signing_keys().is_empty());
        assert!(fixture.facts().policy_recommendations().is_empty());
        assert_eq!(fixture.expectation().details().get("requested"), Some(&Value::from("11")));
    }

    #[test]
    fn policy_recommendation_requires_engine_verifiable_material() {
        assert_schema_rejects(
            fixture_value("capability/unsupported-room-version.json"),
            |value| {
                value
                    .get_mut("facts")
                    .and_then(Value::as_object_mut)
                    .and_then(|facts| facts.get_mut("policy_recommendations"))
                    .and_then(Value::as_array_mut)
                    .expect("recommendations array")
                    .push(serde_json::json!({
                        "event_id": "$candidate:example.org",
                        "policy_event_id": "$policy:example.org",
                        "policy_fingerprint": "policy-v1",
                        "state": "valid"
                    }));
            },
        );

        let mut value = fixture_value("capability/unsupported-room-version.json");
        value
            .get_mut("facts")
            .and_then(Value::as_object_mut)
            .and_then(|facts| facts.get_mut("policy_recommendations"))
            .and_then(Value::as_array_mut)
            .expect("recommendations array")
            .push(serde_json::json!({
                "event_id": "$candidate:example.org",
                "policy_event_id": "$policy:example.org",
                "policy_fingerprint": "policy-v1",
                "outcome": {
                    "kind": "signature-material",
                    "server": "policy.example.org",
                    "key_id": "ed25519:policy_server",
                    "signature": "AAAAAAAA"
                }
            }));

        let validator = load_schema_validator();
        validate_fixture_value(&validator, Path::new("policy-material.json"), &value)
            .expect("signature material is schema-valid");
        let fixture: Fixture = serde_json::from_value(value).expect("typed fixture parses");
        assert!(matches!(
            fixture.facts().policy_recommendations()[0].outcome(),
            PolicyRecommendationOutcome::SignatureMaterial { .. }
        ));
    }
'''
    index = text.rfind("}\n")
    if index < 0:
        raise SystemExit(f"{path}: test-module terminator missing")
    text = text[:index] + added_tests + text[index:]
    path.write_text(text, encoding="utf-8")


def patch_fixture_docs_and_adr() -> None:
    readme = ROOT / "fixtures/README.md"
    checklist = '''## Authoring checklist

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

'''
    replace_once(
        readme,
        "---\n\n## Discovery\n",
        "---\n\n" + checklist + "## Discovery\n",
    )

    policy_docs = '''## Policy Server recommendation facts

A Policy Server recommendation fact never contains a host-asserted `valid` or `invalid` flag.

It contains one of:

- `outcome.kind = "signature-material"` with the server, key ID, and signature bytes that the event engine must validate itself; or
- `outcome.kind = "final-unavailable"` after the required acquisition attempt completed without usable material.

The fact remains bound to the candidate event, active policy event, and frozen policy fingerprint. Signing keys remain separate immutable facts.

---

'''
    replace_once(
        readme,
        "---\n\n## Updating a fixture\n",
        "---\n\n" + policy_docs + "## Updating a fixture\n",
    )

    adr = ROOT / "docs/adr/0005-pre-release-contract-corrections.md"
    adr.write_text(
        '''# ADR 0005: Correct pre-release halt and fixture contracts

**Status:** Accepted  
**Date:** 2026-07-13

## Context

The contract-seeding repository exposed two states that contradicted its own architecture:

1. `Halt::new` could create `halt.budget_exceeded` when consumed work was equal to or below the configured limit.
2. Policy Server recommendation fixtures accepted a host-asserted `valid` or `invalid` state even though the event engine must validate supplied signature material itself.

The typed fixture loader also discarded schema-required input, fact, provenance, option, detail, and expectation fields after schema validation. No CyborgHeart crate or fixture schema has been released or published, and Matrix support remains `none`.

## Decision

- A budget-exceeded halt is constructed only through a checked constructor that requires `consumed > limit`.
- Invalid halt construction returns a typed error carrying the dimension, limit, and consumed amount.
- A Policy Server recommendation fact carries either signature material for engine validation or a terminal acquisition-unavailable outcome.
- Host assertions that recommendation material is already valid or invalid are rejected by schema.
- The event testkit retains every schema-required fixture field in typed structures.
- Fixture schema version 1 is corrected in place because it has no released or supported consumer contract.

## Consequences

- Contradictory budget halts are unrepresentable through the public constructor.
- The host remains responsible for acquisition, while the event engine remains responsible for cryptographic meaning.
- Future Stage 1 work can construct complete explicit inputs and fact snapshots without reparsing discarded fixture JSON.
- Any external experiment using the unreleased earlier schema must update before relying on the repository.

## Alternatives considered

### Keep host-provided validity states

Rejected because it moves protocol validation authority outside the event engine and permits inconsistent replay evidence.

### Introduce fixture schema version 2 immediately

Rejected because there is no release, published crate, compatibility claim, or supported consumer requiring migration machinery. Versioning an unshipped mistake would add process without preserving a real contract.

### Leave `Halt::new` unrestricted and rely on callers

Rejected because the public type would continue to represent a state its stable machine code says cannot exist.

## Reconsider when

- fixture schema version 1 has an external supported consumer;
- Policy Server rules change in a later Matrix baseline;
- a broader typed budget-exhaustion model replaces the initial halt type.

## Related documents

- [`../foundational/EVENT-LIFECYCLE.md`](../foundational/EVENT-LIFECYCLE.md)
- [`../foundational/DEPENDENCY-MODEL.md`](../foundational/DEPENDENCY-MODEL.md)
- [`../foundational/DECISION-CODES.md`](../foundational/DECISION-CODES.md)
- [`../../fixtures/README.md`](../../fixtures/README.md)
- [`0004-structural-public-contract-invariants.md`](./0004-structural-public-contract-invariants.md)
''',
        encoding="utf-8",
    )

    adr_readme = ROOT / "docs/adr/README.md"
    replace_once(
        adr_readme,
        "- [`0004-structural-public-contract-invariants.md`](./0004-structural-public-contract-invariants.md)\n",
        "- [`0004-structural-public-contract-invariants.md`](./0004-structural-public-contract-invariants.md)\n"
        "- [`0005-pre-release-contract-corrections.md`](./0005-pre-release-contract-corrections.md)\n",
    )


def main() -> None:
    patch_decision_contract()
    patch_fixture_schema()
    patch_testkit()
    patch_fixture_docs_and_adr()
    for relative in [
        "AGENTS.md",
        "docs/README.md",
        "scripts/check-markdown-links.py",
        ".editorconfig",
        "fixtures/README.md",
        "docs/adr/README.md",
        "docs/adr/0005-pre-release-contract-corrections.md",
    ]:
        ensure_newline(ROOT / relative)


if __name__ == "__main__":
    main()

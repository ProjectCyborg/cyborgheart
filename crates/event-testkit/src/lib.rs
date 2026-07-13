//! Fixture loading and validation for CyborgHeart event application.
//!
//! The testkit validates fixture shape and exposes deterministic fixture
//! enumeration without calling incomplete event-application lifecycle stages.

use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display},
    fs,
    path::{Path, PathBuf},
};

use cyborg_heart_event_application::EventProvenance;
use ruma::{CanonicalJsonObject, CanonicalJsonValue};
use serde::Deserialize;
use serde_json::{Map, Value};

/// Fixture schema version supported by this testkit.
pub const SUPPORTED_FIXTURE_SCHEMA_VERSION: u64 = 1;

/// Fixture lifecycle maturity.
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

/// Fixture loading and validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixtureError {
    /// Filesystem error.
    Io {
        /// Path involved in the error.
        path: PathBuf,
        /// Human-readable diagnostic.
        message: String,
    },
    /// JSON parsing error.
    Json {
        /// Path involved in the error.
        path: PathBuf,
        /// Human-readable diagnostic.
        message: String,
    },
    /// JSON Schema validation error.
    Schema {
        /// Path involved in the error.
        path: PathBuf,
        /// Stable ordered validation diagnostics.
        errors: Vec<String>,
    },
    /// Fixture schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Path involved in the error.
        path: PathBuf,
        /// Declared schema version.
        version: u64,
    },
    /// Duplicate fixture ID.
    DuplicateId {
        /// Duplicate fixture ID.
        id: String,
        /// First path where the ID appeared.
        first_path: PathBuf,
        /// Second path where the ID appeared.
        second_path: PathBuf,
    },
}

impl Display for FixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => {
                write!(formatter, "{}: {message}", path.display())
            }
            Self::Json { path, message } => {
                write!(formatter, "{}: invalid JSON: {message}", path.display())
            }
            Self::Schema { path, errors } => {
                write!(
                    formatter,
                    "{}: schema validation failed: {errors:?}",
                    path.display()
                )
            }
            Self::UnsupportedSchemaVersion { path, version } => write!(
                formatter,
                "{}: unsupported fixture schema version {version}",
                path.display()
            ),
            Self::DuplicateId {
                id,
                first_path,
                second_path,
            } => write!(
                formatter,
                "duplicate fixture id {id} in {} and {}",
                first_path.display(),
                second_path.display()
            ),
        }
    }
}

impl Error for FixtureError {}

/// Input adapter error for fixture candidate conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputAdapterError {
    /// Raw-json fixture lacks a raw string.
    MissingRawJson,
    /// Raw JSON is syntactically invalid.
    InvalidJson {
        /// Parser diagnostic.
        message: String,
    },
    /// Candidate root is not a JSON object.
    NotAnObject,
    /// Canonical-object fixture lacks a candidate.
    MissingCandidate,
    /// Candidate contains a value outside Matrix canonical JSON constraints.
    NonCanonicalValue {
        /// Field whose value failed canonical conversion.
        field: String,
    },
}

impl Display for InputAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRawJson => write!(formatter, "fixture is missing raw JSON"),
            Self::InvalidJson { message } => write!(formatter, "invalid JSON: {message}"),
            Self::NotAnObject => write!(formatter, "candidate root is not an object"),
            Self::MissingCandidate => write!(formatter, "fixture is missing candidate object"),
            Self::NonCanonicalValue { field } => {
                write!(formatter, "field {field} is not Matrix canonical JSON")
            }
        }
    }
}

impl Error for InputAdapterError {}

fn canonical_object_from_map(
    candidate: &Map<String, Value>,
) -> Result<CanonicalJsonObject, InputAdapterError> {
    candidate
        .iter()
        .map(|(key, value)| {
            ensure_canonical_json_value(value, key)?;
            let canonical = CanonicalJsonValue::try_from(value.clone()).map_err(|_| {
                InputAdapterError::NonCanonicalValue {
                    field: key.to_owned(),
                }
            })?;
            Ok((key.to_owned(), canonical))
        })
        .collect()
}

fn ensure_canonical_json_value(value: &Value, field: &str) -> Result<(), InputAdapterError> {
    match value {
        Value::Null | Value::Bool(_) | Value::String(_) => Ok(()),
        Value::Number(number) => {
            const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;
            const MAX_SAFE_INTEGER_U64: u64 = 9_007_199_254_740_991;

            let in_range = if let Some(value) = number.as_i64() {
                (MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER_U64 as i64).contains(&value)
            } else if let Some(value) = number.as_u64() {
                value <= MAX_SAFE_INTEGER_U64
            } else {
                false
            };

            if in_range {
                Ok(())
            } else {
                Err(InputAdapterError::NonCanonicalValue {
                    field: field.to_owned(),
                })
            }
        }
        Value::Array(values) => values
            .iter()
            .try_for_each(|value| ensure_canonical_json_value(value, field)),
        Value::Object(values) => values
            .values()
            .try_for_each(|value| ensure_canonical_json_value(value, field)),
    }
}

fn discover_json_files(root: &Path) -> Result<Vec<PathBuf>, FixtureError> {
    let mut paths = Vec::new();
    collect_json_files(root, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_json_files(root: &Path, paths: &mut Vec<PathBuf>) -> Result<(), FixtureError> {
    for entry in fs::read_dir(root).map_err(|error| FixtureError::Io {
        path: root.to_path_buf(),
        message: error.to_string(),
    })? {
        let entry = entry.map_err(|error| FixtureError::Io {
            path: root.to_path_buf(),
            message: error.to_string(),
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| FixtureError::Io {
            path: path.clone(),
            message: error.to_string(),
        })?;

        if file_type.is_dir() {
            collect_json_files(&path, paths)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            paths.push(path);
        }
    }

    Ok(())
}

fn load_fixtures_from_paths(
    schema_path: &Path,
    paths: Vec<PathBuf>,
) -> Result<FixtureSuite, FixtureError> {
    let schema = read_json(schema_path)?;
    validate_schema_document(schema_path, &schema)?;
    let validator = jsonschema::validator_for(&schema).map_err(|error| FixtureError::Schema {
        path: schema_path.to_path_buf(),
        errors: vec![error.to_string()],
    })?;

    let mut seen_ids: BTreeMap<String, PathBuf> = BTreeMap::new();
    let mut fixtures = Vec::with_capacity(paths.len());

    for path in paths {
        let value = read_json(&path)?;
        validate_fixture_value(&validator, &path, &value)?;

        let version = value
            .get("fixture_schema_version")
            .and_then(Value::as_u64)
            .unwrap_or_default();
        if version != SUPPORTED_FIXTURE_SCHEMA_VERSION {
            return Err(FixtureError::UnsupportedSchemaVersion { path, version });
        }

        let mut fixture: Fixture =
            serde_json::from_value(value).map_err(|error| FixtureError::Json {
                path: path.clone(),
                message: error.to_string(),
            })?;
        fixture.source_path = path.clone();

        if let Some(first_path) = seen_ids.insert(fixture.id.clone(), path.clone()) {
            return Err(FixtureError::DuplicateId {
                id: fixture.id.clone(),
                first_path,
                second_path: path,
            });
        }

        fixtures.push(fixture);
    }

    Ok(FixtureSuite { fixtures })
}

fn validate_schema_document(schema_path: &Path, schema: &Value) -> Result<(), FixtureError> {
    jsonschema::draft202012::meta::validate(schema).map_err(|error| FixtureError::Schema {
        path: schema_path.to_path_buf(),
        errors: vec![error.to_string()],
    })
}

fn validate_fixture_value(
    validator: &jsonschema::Validator,
    path: &Path,
    value: &Value,
) -> Result<(), FixtureError> {
    let mut errors = validator
        .iter_errors(value)
        .map(|error| error.to_string())
        .collect::<Vec<_>>();
    errors.sort();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(FixtureError::Schema {
            path: path.to_path_buf(),
            errors,
        })
    }
}

fn read_json(path: &Path) -> Result<Value, FixtureError> {
    let bytes = fs::read(path).map_err(|error| FixtureError::Io {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;

    serde_json::from_slice(&bytes).map_err(|error| FixtureError::Json {
        path: path.to_path_buf(),
        message: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cyborg_heart_event_application::{
        SupportedRoomVersion, WorkDimension, known_decision_code,
    };

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("crate is under crates/event-testkit")
            .to_path_buf()
    }

    fn schema_path() -> PathBuf {
        repo_root().join("fixtures/schema/fixture.schema.json")
    }

    fn seed_fixture_path(relative: &str) -> PathBuf {
        repo_root().join("fixtures/seed").join(relative)
    }

    fn load_schema_validator() -> jsonschema::Validator {
        let schema = read_json(&schema_path()).expect("schema parses");
        jsonschema::validator_for(&schema).expect("schema compiles")
    }

    fn fixture_value(relative: &str) -> Value {
        read_json(&seed_fixture_path(relative)).expect("fixture parses")
    }

    fn fixture_by_id<'a>(suite: &'a FixtureSuite, id: &str) -> &'a Fixture {
        suite
            .fixtures()
            .iter()
            .find(|fixture| fixture.id() == id)
            .expect("fixture exists")
    }

    fn assert_schema_rejects(mut value: Value, mutate: impl FnOnce(&mut Value)) {
        mutate(&mut value);
        let validator = load_schema_validator();
        let result = validate_fixture_value(&validator, Path::new("mutated.json"), &value);
        assert!(matches!(result, Err(FixtureError::Schema { .. })));
    }

    #[test]
    fn all_discovered_seed_fixtures_validate_and_load() {
        let discovered = discover_json_files(&repo_root().join("fixtures/seed"))
            .expect("fixture discovery succeeds");
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        assert!(
            !discovered.is_empty(),
            "seed fixture corpus must not be empty"
        );
        assert_eq!(suite.fixtures().len(), discovered.len());
        assert!(
            suite
                .fixtures()
                .iter()
                .all(|fixture| fixture.schema_version() == SUPPORTED_FIXTURE_SCHEMA_VERSION)
        );
    }

    #[test]
    fn fixture_ids_are_unique() {
        let first = seed_fixture_path("input/invalid-json.json");
        let result = load_fixtures_from_paths(&schema_path(), vec![first.clone(), first]);
        assert!(matches!(result, Err(FixtureError::DuplicateId { .. })));
    }

    #[test]
    fn fixture_paths_are_deterministic() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        let paths = suite
            .fixtures()
            .iter()
            .map(|fixture| fixture.source_path().to_path_buf())
            .collect::<Vec<_>>();
        let mut sorted = paths.clone();
        sorted.sort();
        assert_eq!(paths, sorted);
    }

    #[test]
    fn unknown_top_level_field_fails_schema_validation() {
        assert_schema_rejects(fixture_value("input/invalid-json.json"), |value| {
            value
                .as_object_mut()
                .expect("fixture is object")
                .insert("unknown".to_owned(), Value::Bool(true));
        });
    }

    #[test]
    fn raw_json_fixture_with_candidate_fails_schema_validation() {
        assert_schema_rejects(fixture_value("input/invalid-json.json"), |value| {
            let input = value
                .get_mut("input")
                .and_then(Value::as_object_mut)
                .expect("input is object");
            input.insert("candidate".to_owned(), Value::Object(Map::new()));
        });
    }

    #[test]
    fn canonical_object_fixture_with_raw_json_fails_schema_validation() {
        assert_schema_rejects(
            fixture_value("capability/unsupported-room-version.json"),
            |value| {
                let input = value
                    .get_mut("input")
                    .and_then(Value::as_object_mut)
                    .expect("input is object");
                input.insert("raw_json".to_owned(), Value::String("{}".to_owned()));
            },
        );
    }

    #[test]
    fn complete_expectation_without_disposition_fails_schema_validation() {
        assert_schema_rejects(fixture_value("input/invalid-json.json"), |value| {
            let expect = value
                .get_mut("expect")
                .and_then(Value::as_object_mut)
                .expect("expect is object");
            expect.insert("kind".to_owned(), Value::String("complete".to_owned()));
        });
    }

    #[test]
    fn dependency_expectation_without_dependencies_fails_schema_validation() {
        assert_schema_rejects(fixture_value("input/invalid-json.json"), |value| {
            let expect = value
                .get_mut("expect")
                .and_then(Value::as_object_mut)
                .expect("expect is object");
            expect.insert(
                "kind".to_owned(),
                Value::String("needs-dependencies".to_owned()),
            );
            expect.insert(
                "code".to_owned(),
                Value::String("need.signing_keys".to_owned()),
            );
        });
    }

    #[test]
    fn unknown_budget_dimension_fails_schema_validation() {
        assert_schema_rejects(fixture_value("input/invalid-json.json"), |value| {
            value
                .get_mut("budget")
                .and_then(Value::as_object_mut)
                .expect("budget is object")
                .insert("canonical_json_byte".to_owned(), Value::from(1));
        });
    }

    #[test]
    fn all_referenced_decision_codes_exist_in_registry() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        for fixture in suite.fixtures() {
            assert!(
                known_decision_code(fixture.expectation().code()),
                "{} should be known",
                fixture.expectation().code()
            );
            for dependency in fixture.expectation().dependencies() {
                assert!(
                    known_decision_code(dependency.code()),
                    "{} should be known",
                    dependency.code()
                );
            }
            for consequence in fixture.expectation().consequences() {
                assert!(
                    known_decision_code(consequence),
                    "{consequence} should be known"
                );
            }
        }
    }

    #[test]
    fn canonical_input_adapter_rejects_non_canonical_integer() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        let fixture = fixture_by_id(&suite, "input.non_canonical_integer");
        assert!(matches!(
            fixture.canonical_candidate(),
            Err(InputAdapterError::NonCanonicalValue { .. })
        ));
    }

    #[test]
    fn contract_stage_adapter_fixtures_are_executable() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");

        assert!(matches!(
            fixture_by_id(&suite, "input.invalid_json").canonical_candidate(),
            Err(InputAdapterError::InvalidJson { .. })
        ));
        assert!(matches!(
            fixture_by_id(&suite, "input.not_an_object").canonical_candidate(),
            Err(InputAdapterError::NotAnObject)
        ));
        assert!(matches!(
            fixture_by_id(&suite, "input.non_canonical_integer").canonical_candidate(),
            Err(InputAdapterError::NonCanonicalValue { .. })
        ));

        let unsupported = fixture_by_id(&suite, "capability.unsupported_room_version");
        let capability_error = SupportedRoomVersion::admit(unsupported.input().room_version())
            .expect_err("room version is unsupported");
        assert_eq!(
            capability_error.code().as_str(),
            unsupported.expectation().code()
        );
    }

    #[test]
    fn verified_intake_fixtures_are_staged_at_contract_stage() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        let staged = suite
            .fixtures()
            .iter()
            .filter(|fixture| {
                matches!(
                    fixture.execution_state(FixtureStage::Contract),
                    FixtureExecutionState::Staged { .. }
                )
            })
            .count();
        assert_eq!(staged, 6);
    }

    #[test]
    fn budget_dimensions_match_event_application_registry() {
        let suite = FixtureSuite::load_seed_from_repo_root(repo_root()).expect("fixtures load");
        for fixture in suite.fixtures() {
            for dimension in fixture.budget().keys() {
                assert!(
                    WorkDimension::from_str_name(dimension).is_some(),
                    "{dimension} should be a known work dimension"
                );
            }
            for dimension in fixture.expectation().work().max().keys() {
                assert!(
                    WorkDimension::from_str_name(dimension).is_some(),
                    "{dimension} should be a known work dimension"
                );
            }
        }
    }

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
        assert_eq!(
            fixture.expectation().details().get("requested"),
            Some(&Value::from("11"))
        );
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
}

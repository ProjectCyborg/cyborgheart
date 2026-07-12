//! Result, disposition, halt, fault, and stable decision-code types.

use crate::{
    DecisionEvidence, DependencyRequestSet, ForwardExtremityEffect, HistoricalStateEffect,
    RedactionEffect, SemanticConsequences, VisibilityClass, WorkDimension,
};

/// Lifecycle stage where a result, request, halt, or fault became meaningful.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LifecycleStage {
    /// External room-version admission.
    RoomVersionAdmission,
    /// Canonical candidate input.
    CanonicalInput,
    /// PDU format validation.
    PduFormat,
    /// Event identity derivation.
    EventIdentity,
    /// Required signer discovery.
    SignerDiscovery,
    /// Cryptographic verification.
    CryptographicVerification,
    /// State-independent authorization.
    StateIndependentAuth,
    /// Claimed-auth authorization.
    ClaimedAuth,
    /// State-before reconstruction.
    StateBefore,
    /// Historical authorization.
    HistoricalAuth,
    /// Current-state authorization.
    CurrentStateAuth,
    /// Policy Server validation.
    PolicyServer,
    /// Semantic consequence calculation.
    Consequences,
}

impl LifecycleStage {
    /// Stable snake-case stage identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoomVersionAdmission => "room_version_admission",
            Self::CanonicalInput => "canonical_input",
            Self::PduFormat => "pdu_format",
            Self::EventIdentity => "event_identity",
            Self::SignerDiscovery => "signer_discovery",
            Self::CryptographicVerification => "cryptographic_verification",
            Self::StateIndependentAuth => "state_independent_auth",
            Self::ClaimedAuth => "claimed_auth",
            Self::StateBefore => "state_before",
            Self::HistoricalAuth => "historical_auth",
            Self::CurrentStateAuth => "current_state_auth",
            Self::PolicyServer => "policy_server",
            Self::Consequences => "consequences",
        }
    }
}

/// Capability-boundary machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CapabilityCode {
    /// External room version is unsupported.
    UnsupportedRoomVersion,
}

impl CapabilityCode {
    /// All capability codes in stable order.
    pub const ALL: [Self; 1] = [Self::UnsupportedRoomVersion];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedRoomVersion => "capability.unsupported_room_version",
        }
    }

    /// Stable lifecycle stage owned by this code.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        match self {
            Self::UnsupportedRoomVersion => LifecycleStage::RoomVersionAdmission,
        }
    }

    /// All capability codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Raw-input adapter machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputCode {
    /// Raw input is malformed JSON.
    InvalidJson,
    /// Raw or object input is not Matrix canonical JSON.
    NonCanonicalValue,
    /// Candidate root is not a JSON object.
    NotAnObject,
}

impl InputCode {
    /// All input codes in stable order.
    pub const ALL: [Self; 3] = [
        Self::InvalidJson,
        Self::NonCanonicalValue,
        Self::NotAnObject,
    ];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidJson => "input.invalid_json",
            Self::NonCanonicalValue => "input.non_canonical_value",
            Self::NotAnObject => "input.not_an_object",
        }
    }

    /// Stable lifecycle stage owned by this code.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        LifecycleStage::CanonicalInput
    }

    /// All input codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Accepted machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AcceptedCode {
    /// Event is authorized.
    Authorized,
}

impl AcceptedCode {
    /// All accepted codes in stable order.
    pub const ALL: [Self; 1] = [Self::Authorized];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authorized => "accepted.authorized",
        }
    }

    /// Stable lifecycle stage owned by this code.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        LifecycleStage::Consequences
    }

    /// All accepted codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Dropped-event machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DropCode {
    /// Invalid PDU format.
    InvalidPduFormat,
    /// Asserted event ID differs from the derived ID.
    EventIdMismatch,
    /// Event identity could not be constructed.
    InvalidEventIdentity,
    /// Required event signature is absent.
    MissingRequiredSignature,
    /// Required signature algorithm or key ID is unsupported.
    UnsupportedRequiredSignature,
    /// Required signature encoding is invalid.
    InvalidSignatureEncoding,
    /// Required signature is cryptographically invalid.
    InvalidRequiredSignature,
    /// Integrity fields are malformed.
    InvalidIntegrityFields,
    /// Canonical event became unrecoverable after admission.
    UnrecoverableCanonicalEvent,
}

impl DropCode {
    /// All dropped-event codes in stable order.
    pub const ALL: [Self; 9] = [
        Self::InvalidPduFormat,
        Self::EventIdMismatch,
        Self::InvalidEventIdentity,
        Self::MissingRequiredSignature,
        Self::UnsupportedRequiredSignature,
        Self::InvalidSignatureEncoding,
        Self::InvalidRequiredSignature,
        Self::InvalidIntegrityFields,
        Self::UnrecoverableCanonicalEvent,
    ];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidPduFormat => "drop.invalid_pdu_format",
            Self::EventIdMismatch => "drop.event_id_mismatch",
            Self::InvalidEventIdentity => "drop.invalid_event_identity",
            Self::MissingRequiredSignature => "drop.missing_required_signature",
            Self::UnsupportedRequiredSignature => "drop.unsupported_required_signature",
            Self::InvalidSignatureEncoding => "drop.invalid_signature_encoding",
            Self::InvalidRequiredSignature => "drop.invalid_required_signature",
            Self::InvalidIntegrityFields => "drop.invalid_integrity_fields",
            Self::UnrecoverableCanonicalEvent => "drop.unrecoverable_canonical_event",
        }
    }

    /// Stable lifecycle stage owned by this code.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        match self {
            Self::InvalidPduFormat => LifecycleStage::PduFormat,
            Self::EventIdMismatch | Self::InvalidEventIdentity => LifecycleStage::EventIdentity,
            Self::MissingRequiredSignature | Self::UnsupportedRequiredSignature => {
                LifecycleStage::SignerDiscovery
            }
            Self::InvalidSignatureEncoding
            | Self::InvalidRequiredSignature
            | Self::InvalidIntegrityFields => LifecycleStage::CryptographicVerification,
            Self::UnrecoverableCanonicalEvent => LifecycleStage::CanonicalInput,
        }
    }

    /// All dropped-event codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Rejected-event machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RejectionCode {
    /// State-independent authorization failed.
    StateIndependentAuth,
    /// Claimed-auth authorization failed.
    ClaimedAuth,
    /// Historical authorization failed.
    HistoricalAuth,
}

impl RejectionCode {
    /// All rejected-event codes in stable order.
    pub const ALL: [Self; 3] = [
        Self::StateIndependentAuth,
        Self::ClaimedAuth,
        Self::HistoricalAuth,
    ];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateIndependentAuth => "reject.state_independent_auth",
            Self::ClaimedAuth => "reject.claimed_auth",
            Self::HistoricalAuth => "reject.historical_auth",
        }
    }

    /// Stable lifecycle stage owned by this code.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        match self {
            Self::StateIndependentAuth => LifecycleStage::StateIndependentAuth,
            Self::ClaimedAuth => LifecycleStage::ClaimedAuth,
            Self::HistoricalAuth => LifecycleStage::HistoricalAuth,
        }
    }

    /// All rejected-event codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Soft-failed-event machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SoftFailureCode {
    /// Current-state authorization failed.
    CurrentStateAuth,
    /// Required Policy Server recommendation remained unavailable.
    PolicyRecommendation,
}

impl SoftFailureCode {
    /// All soft-failed-event codes in stable order.
    pub const ALL: [Self; 2] = [Self::CurrentStateAuth, Self::PolicyRecommendation];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentStateAuth => "soft_fail.current_state_auth",
            Self::PolicyRecommendation => "soft_fail.policy_recommendation",
        }
    }

    /// Stable lifecycle stage owned by this code.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        match self {
            Self::CurrentStateAuth => LifecycleStage::CurrentStateAuth,
            Self::PolicyRecommendation => LifecycleStage::PolicyServer,
        }
    }

    /// All soft-failed-event codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Bounded halt machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HaltCode {
    /// Evaluation budget was exceeded.
    BudgetExceeded,
}

impl HaltCode {
    /// All halt codes in stable order.
    pub const ALL: [Self; 1] = [Self::BudgetExceeded];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BudgetExceeded => "halt.budget_exceeded",
        }
    }

    /// All halt codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Semantic consequence machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConsequenceCode {
    /// Historical state did not change.
    StateUnchanged,
    /// Historical state was updated.
    StateUpdated,
    /// Forward extremity was added.
    ForwardExtremityAdded,
    /// Forward extremity was not added.
    ForwardExtremityNotAdded,
    /// Redaction was applied.
    RedactionApplied,
    /// Redaction partner remains pending.
    RedactionPendingPartner,
}

impl ConsequenceCode {
    /// All consequence codes in stable order.
    pub const ALL: [Self; 6] = [
        Self::StateUnchanged,
        Self::StateUpdated,
        Self::ForwardExtremityAdded,
        Self::ForwardExtremityNotAdded,
        Self::RedactionApplied,
        Self::RedactionPendingPartner,
    ];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateUnchanged => "consequence.state_unchanged",
            Self::StateUpdated => "consequence.state_updated",
            Self::ForwardExtremityAdded => "consequence.forward_extremity_added",
            Self::ForwardExtremityNotAdded => "consequence.forward_extremity_not_added",
            Self::RedactionApplied => "consequence.redaction_applied",
            Self::RedactionPendingPartner => "consequence.redaction_pending_partner",
        }
    }

    /// Stable lifecycle stage owned by consequence codes.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        LifecycleStage::Consequences
    }

    /// All consequence codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Evaluation fault machine codes.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FaultCode {
    /// Host supplied inconsistent immutable facts.
    InconsistentFacts,
    /// CyborgHeart internal invariant was violated.
    InternalInvariant,
}

impl FaultCode {
    /// All fault codes in stable order.
    pub const ALL: [Self; 2] = [Self::InconsistentFacts, Self::InternalInvariant];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InconsistentFacts => "fault.inconsistent_facts",
            Self::InternalInvariant => "fault.internal_invariant",
        }
    }

    /// All fault codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Stable machine-code envelope.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecisionCode {
    /// Capability code.
    Capability(CapabilityCode),
    /// Input adapter code.
    Input(InputCode),
    /// Accepted-event code.
    Accepted(AcceptedCode),
    /// Dropped-event code.
    Dropped(DropCode),
    /// Rejected-event code.
    Rejected(RejectionCode),
    /// Soft-failed-event code.
    SoftFailed(SoftFailureCode),
    /// Dependency request code.
    Dependency(crate::DependencyCode),
    /// Halt code.
    Halt(HaltCode),
    /// Consequence code.
    Consequence(ConsequenceCode),
    /// Fault code.
    Fault(FaultCode),
}

impl DecisionCode {
    /// All initial decision codes in stable order.
    pub const ALL: [Self; 33] = [
        Self::Capability(CapabilityCode::UnsupportedRoomVersion),
        Self::Input(InputCode::InvalidJson),
        Self::Input(InputCode::NonCanonicalValue),
        Self::Input(InputCode::NotAnObject),
        Self::Accepted(AcceptedCode::Authorized),
        Self::Dropped(DropCode::InvalidPduFormat),
        Self::Dropped(DropCode::EventIdMismatch),
        Self::Dropped(DropCode::InvalidEventIdentity),
        Self::Dropped(DropCode::MissingRequiredSignature),
        Self::Dropped(DropCode::UnsupportedRequiredSignature),
        Self::Dropped(DropCode::InvalidSignatureEncoding),
        Self::Dropped(DropCode::InvalidRequiredSignature),
        Self::Dropped(DropCode::InvalidIntegrityFields),
        Self::Dropped(DropCode::UnrecoverableCanonicalEvent),
        Self::Rejected(RejectionCode::StateIndependentAuth),
        Self::Rejected(RejectionCode::ClaimedAuth),
        Self::Rejected(RejectionCode::HistoricalAuth),
        Self::SoftFailed(SoftFailureCode::CurrentStateAuth),
        Self::SoftFailed(SoftFailureCode::PolicyRecommendation),
        Self::Dependency(crate::DependencyCode::Event),
        Self::Dependency(crate::DependencyCode::StateAfter),
        Self::Dependency(crate::DependencyCode::CurrentRoomState),
        Self::Dependency(crate::DependencyCode::SigningKeys),
        Self::Dependency(crate::DependencyCode::PolicyRecommendation),
        Self::Halt(HaltCode::BudgetExceeded),
        Self::Consequence(ConsequenceCode::StateUnchanged),
        Self::Consequence(ConsequenceCode::StateUpdated),
        Self::Consequence(ConsequenceCode::ForwardExtremityAdded),
        Self::Consequence(ConsequenceCode::ForwardExtremityNotAdded),
        Self::Consequence(ConsequenceCode::RedactionApplied),
        Self::Consequence(ConsequenceCode::RedactionPendingPartner),
        Self::Fault(FaultCode::InconsistentFacts),
        Self::Fault(FaultCode::InternalInvariant),
    ];

    /// Stable dotted code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Capability(code) => code.as_str(),
            Self::Input(code) => code.as_str(),
            Self::Accepted(code) => code.as_str(),
            Self::Dropped(code) => code.as_str(),
            Self::Rejected(code) => code.as_str(),
            Self::SoftFailed(code) => code.as_str(),
            Self::Dependency(code) => code.as_str(),
            Self::Halt(code) => code.as_str(),
            Self::Consequence(code) => code.as_str(),
            Self::Fault(code) => code.as_str(),
        }
    }

    /// All initial decision codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Matrix event disposition for a complete decision.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Disposition {
    /// Dropped before normal room-history participation.
    Dropped,
    /// Accepted under Matrix authorization.
    Accepted,
    /// Rejected at historical authorization.
    Rejected,
    /// Soft-failed at current-state or policy checks.
    SoftFailed,
}

/// Complete event decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompleteDecision {
    /// Candidate failed before normal room-history participation.
    Dropped(DroppedDecision),
    /// Candidate was admitted to history processing.
    Admitted(AdmittedDecision),
}

impl CompleteDecision {
    /// Returns the single Matrix disposition for this complete decision.
    #[must_use]
    pub const fn disposition(&self) -> Disposition {
        match self {
            Self::Dropped(_) => Disposition::Dropped,
            Self::Admitted(decision) => decision.disposition(),
        }
    }

    /// Lifecycle stage where this complete decision became final.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        match self {
            Self::Dropped(decision) => decision.stage(),
            Self::Admitted(decision) => decision.stage(),
        }
    }

    /// Decision evidence.
    #[must_use]
    pub const fn evidence(&self) -> &DecisionEvidence {
        match self {
            Self::Dropped(decision) => decision.evidence(),
            Self::Admitted(decision) => decision.evidence(),
        }
    }

    /// Semantic consequences for admitted decisions.
    #[must_use]
    pub const fn consequences(&self) -> Option<&SemanticConsequences> {
        match self {
            Self::Dropped(_) => None,
            Self::Admitted(decision) => Some(decision.consequences()),
        }
    }
}

/// Dropped-event decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DroppedDecision {
    code: DropCode,
    evidence: DecisionEvidence,
}

impl DroppedDecision {
    /// Creates a dropped decision.
    #[must_use]
    pub const fn new(code: DropCode, evidence: DecisionEvidence) -> Self {
        Self { code, evidence }
    }

    /// Stable machine code.
    #[must_use]
    pub const fn code(&self) -> DropCode {
        self.code
    }

    /// Lifecycle stage derived from the stable code.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        self.code.stage()
    }

    /// Decision evidence.
    #[must_use]
    pub const fn evidence(&self) -> &DecisionEvidence {
        &self.evidence
    }
}

/// Admitted-event outcome and stable code.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdmittedOutcome {
    /// Accepted event.
    Accepted(AcceptedCode),
    /// Rejected event.
    Rejected(RejectionCode),
    /// Soft-failed event.
    SoftFailed(SoftFailureCode),
}

impl AdmittedOutcome {
    /// Matrix disposition derived from this outcome.
    #[must_use]
    pub const fn disposition(self) -> Disposition {
        match self {
            Self::Accepted(_) => Disposition::Accepted,
            Self::Rejected(_) => Disposition::Rejected,
            Self::SoftFailed(_) => Disposition::SoftFailed,
        }
    }

    /// Stable terminal stage derived from this outcome's code.
    #[must_use]
    pub const fn stage(self) -> LifecycleStage {
        match self {
            Self::Accepted(code) => code.stage(),
            Self::Rejected(code) => code.stage(),
            Self::SoftFailed(code) => code.stage(),
        }
    }

    /// Visibility class derived from this outcome.
    #[must_use]
    pub const fn visibility(self) -> VisibilityClass {
        match self {
            Self::Accepted(_) => VisibilityClass::Normal,
            Self::Rejected(_) => VisibilityClass::Rejected,
            Self::SoftFailed(_) => VisibilityClass::SoftFailed,
        }
    }
}

/// Complete admitted-event decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedDecision {
    outcome: AdmittedOutcome,
    evidence: DecisionEvidence,
    consequences: SemanticConsequences,
}

impl AdmittedDecision {
    /// Creates an accepted decision from valid consequence dimensions.
    #[must_use]
    pub const fn accepted(
        code: AcceptedCode,
        evidence: DecisionEvidence,
        historical_state: HistoricalStateEffect,
        forward_extremity: ForwardExtremityEffect,
        redaction: RedactionEffect,
    ) -> Self {
        Self {
            outcome: AdmittedOutcome::Accepted(code),
            evidence,
            consequences: SemanticConsequences::from_effects(
                historical_state,
                forward_extremity,
                redaction,
            ),
        }
    }

    /// Creates a rejected decision with fixed rejection consequences.
    #[must_use]
    pub const fn rejected(code: RejectionCode, evidence: DecisionEvidence) -> Self {
        Self {
            outcome: AdmittedOutcome::Rejected(code),
            evidence,
            consequences: SemanticConsequences::from_effects(
                HistoricalStateEffect::Unchanged,
                ForwardExtremityEffect::NotAdded,
                RedactionEffect::NotApplicable,
            ),
        }
    }

    /// Creates a soft-failed decision with fixed immediate graph consequences.
    #[must_use]
    pub const fn soft_failed(
        code: SoftFailureCode,
        evidence: DecisionEvidence,
        historical_state: HistoricalStateEffect,
    ) -> Self {
        Self {
            outcome: AdmittedOutcome::SoftFailed(code),
            evidence,
            consequences: SemanticConsequences::from_effects(
                historical_state,
                ForwardExtremityEffect::NotAdded,
                RedactionEffect::NotApplicable,
            ),
        }
    }

    /// Admitted outcome.
    #[must_use]
    pub const fn outcome(&self) -> AdmittedOutcome {
        self.outcome
    }

    /// Matrix disposition derived from the outcome.
    #[must_use]
    pub const fn disposition(&self) -> Disposition {
        self.outcome.disposition()
    }

    /// Terminal stage derived from the outcome code.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        self.outcome.stage()
    }

    /// Decision evidence.
    #[must_use]
    pub const fn evidence(&self) -> &DecisionEvidence {
        &self.evidence
    }

    /// Semantic consequences.
    #[must_use]
    pub const fn consequences(&self) -> &SemanticConsequences {
        &self.consequences
    }

    /// Visibility class derived from the outcome.
    #[must_use]
    pub const fn visibility(&self) -> VisibilityClass {
        self.outcome.visibility()
    }
}

/// Evidence-bearing incomplete dependency result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyNeed {
    requests: DependencyRequestSet,
    evidence: DecisionEvidence,
}

impl DependencyNeed {
    /// Creates a dependency need from a non-empty request set.
    #[must_use]
    pub const fn new(requests: DependencyRequestSet, evidence: DecisionEvidence) -> Self {
        Self { requests, evidence }
    }

    /// Required dependency requests.
    #[must_use]
    pub const fn requests(&self) -> &DependencyRequestSet {
        &self.requests
    }

    /// Evidence accumulated before the missing facts were discovered.
    #[must_use]
    pub const fn evidence(&self) -> &DecisionEvidence {
        &self.evidence
    }
}

/// Event application result that is not an implementation fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationResult {
    /// Evaluation reached a complete event disposition.
    Complete(CompleteDecision),
    /// Evaluation requires immutable facts to continue.
    NeedsDependencies(DependencyNeed),
    /// Evaluation halted without a Matrix disposition.
    Halted(Halt),
}

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

/// Host or implementation fault, separate from Matrix event dispositions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationFault {
    code: FaultCode,
    stage: LifecycleStage,
    evidence: DecisionEvidence,
}

impl EvaluationFault {
    /// Creates an evaluation fault.
    #[must_use]
    pub const fn new(code: FaultCode, stage: LifecycleStage, evidence: DecisionEvidence) -> Self {
        Self {
            code,
            stage,
            evidence,
        }
    }

    /// Stable fault code.
    #[must_use]
    pub const fn code(&self) -> FaultCode {
        self.code
    }

    /// Lifecycle stage.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        self.stage
    }

    /// Fault evidence.
    #[must_use]
    pub const fn evidence(&self) -> &DecisionEvidence {
        &self.evidence
    }
}

/// Returns true when a code is part of the initial registry.
#[must_use]
pub fn known_decision_code(code: &str) -> bool {
    DecisionCode::all()
        .iter()
        .any(|known| known.as_str() == code)
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, str::FromStr};

    use ruma::OwnedEventId;

    use super::*;
    use crate::dependencies::{Dependency, DependencyRequest};

    fn evidence() -> DecisionEvidence {
        DecisionEvidence::new()
    }

    fn event_id(id: &str) -> OwnedEventId {
        OwnedEventId::from_str(id).expect("valid event id")
    }

    #[test]
    fn complete_decisions_have_one_disposition() {
        let dropped =
            CompleteDecision::Dropped(DroppedDecision::new(DropCode::InvalidPduFormat, evidence()));
        assert_eq!(dropped.disposition(), Disposition::Dropped);
        assert_eq!(dropped.stage(), LifecycleStage::PduFormat);
        assert!(dropped.consequences().is_none());

        let accepted = CompleteDecision::Admitted(AdmittedDecision::accepted(
            AcceptedCode::Authorized,
            evidence(),
            HistoricalStateEffect::StateTupleUpdated,
            ForwardExtremityEffect::Added,
            RedactionEffect::NotApplicable,
        ));
        assert_eq!(accepted.disposition(), Disposition::Accepted);
        assert_eq!(accepted.stage(), LifecycleStage::Consequences);
        assert!(accepted.consequences().is_some());
    }

    #[test]
    fn admitted_outcomes_expose_consequences_and_derived_visibility() {
        let accepted = AdmittedDecision::accepted(
            AcceptedCode::Authorized,
            evidence(),
            HistoricalStateEffect::Unchanged,
            ForwardExtremityEffect::Added,
            RedactionEffect::Applied,
        );
        assert_eq!(accepted.disposition(), Disposition::Accepted);
        assert_eq!(accepted.stage(), LifecycleStage::Consequences);
        assert_eq!(accepted.visibility(), VisibilityClass::Normal);
        assert_eq!(
            accepted.consequences().redaction(),
            RedactionEffect::Applied
        );

        let rejected = AdmittedDecision::rejected(RejectionCode::HistoricalAuth, evidence());
        assert_eq!(rejected.disposition(), Disposition::Rejected);
        assert_eq!(rejected.stage(), LifecycleStage::HistoricalAuth);
        assert_eq!(rejected.visibility(), VisibilityClass::Rejected);
        assert_eq!(
            rejected.consequences().historical_state(),
            HistoricalStateEffect::Unchanged
        );
        assert_eq!(
            rejected.consequences().forward_extremity(),
            ForwardExtremityEffect::NotAdded
        );

        let soft_failed = AdmittedDecision::soft_failed(
            SoftFailureCode::CurrentStateAuth,
            evidence(),
            HistoricalStateEffect::StateTupleUpdated,
        );
        assert_eq!(soft_failed.disposition(), Disposition::SoftFailed);
        assert_eq!(soft_failed.stage(), LifecycleStage::CurrentStateAuth);
        assert_eq!(soft_failed.visibility(), VisibilityClass::SoftFailed);
        assert_eq!(
            soft_failed.consequences().historical_state(),
            HistoricalStateEffect::StateTupleUpdated
        );
        assert_eq!(
            soft_failed.consequences().forward_extremity(),
            ForwardExtremityEffect::NotAdded
        );
    }

    #[test]
    fn code_families_derive_terminal_stages() {
        assert_eq!(
            CapabilityCode::UnsupportedRoomVersion.stage(),
            LifecycleStage::RoomVersionAdmission
        );
        assert_eq!(
            InputCode::InvalidJson.stage(),
            LifecycleStage::CanonicalInput
        );
        assert_eq!(
            AcceptedCode::Authorized.stage(),
            LifecycleStage::Consequences
        );
        assert_eq!(
            DropCode::InvalidPduFormat.stage(),
            LifecycleStage::PduFormat
        );
        assert_eq!(
            DropCode::EventIdMismatch.stage(),
            LifecycleStage::EventIdentity
        );
        assert_eq!(
            DropCode::MissingRequiredSignature.stage(),
            LifecycleStage::SignerDiscovery
        );
        assert_eq!(
            DropCode::InvalidRequiredSignature.stage(),
            LifecycleStage::CryptographicVerification
        );
        assert_eq!(
            DropCode::UnrecoverableCanonicalEvent.stage(),
            LifecycleStage::CanonicalInput
        );
        assert_eq!(
            RejectionCode::StateIndependentAuth.stage(),
            LifecycleStage::StateIndependentAuth
        );
        assert_eq!(
            SoftFailureCode::PolicyRecommendation.stage(),
            LifecycleStage::PolicyServer
        );
        assert_eq!(
            ConsequenceCode::StateUpdated.stage(),
            LifecycleStage::Consequences
        );
    }

    #[test]
    fn consequence_projection_is_dimensionally_exclusive() {
        let decision = AdmittedDecision::accepted(
            AcceptedCode::Authorized,
            evidence(),
            HistoricalStateEffect::StateTupleUpdated,
            ForwardExtremityEffect::NotAdded,
            RedactionEffect::PendingPartner,
        );
        let codes = decision.consequences().codes();

        assert!(codes.contains(&ConsequenceCode::StateUpdated));
        assert!(!codes.contains(&ConsequenceCode::StateUnchanged));
        assert!(codes.contains(&ConsequenceCode::ForwardExtremityNotAdded));
        assert!(!codes.contains(&ConsequenceCode::ForwardExtremityAdded));
        assert!(codes.contains(&ConsequenceCode::RedactionPendingPartner));
        assert!(!codes.contains(&ConsequenceCode::RedactionApplied));

        let rejected = AdmittedDecision::rejected(RejectionCode::ClaimedAuth, evidence());
        assert_eq!(
            rejected.consequences().codes(),
            BTreeSet::from([
                ConsequenceCode::StateUnchanged,
                ConsequenceCode::ForwardExtremityNotAdded,
            ])
        );
    }

    #[test]
    fn dependency_needs_carry_evidence() {
        let evidence = evidence();
        let request = DependencyRequest::single(
            LifecycleStage::ClaimedAuth,
            Dependency::Event {
                event_id: event_id("$auth:example.org"),
            },
        );
        let need = DependencyNeed::new(DependencyRequestSet::single(request), evidence.clone());

        let result = EvaluationResult::NeedsDependencies(need);

        match result {
            EvaluationResult::NeedsDependencies(need) => {
                assert_eq!(need.evidence(), &evidence);
                assert_eq!(need.requests().requests().len(), 1);
            }
            EvaluationResult::Complete(_) | EvaluationResult::Halted(_) => {
                panic!("dependency need should remain incomplete")
            }
        }
    }

    #[test]
    fn registry_contains_each_unique_code_once() {
        let mut strings = BTreeSet::new();
        for code in DecisionCode::all() {
            assert!(
                strings.insert(code.as_str()),
                "{} is duplicated",
                code.as_str()
            );
            assert!(known_decision_code(code.as_str()));
        }

        assert_eq!(strings.len(), DecisionCode::all().len());
    }

    #[test]
    fn registry_contains_every_code_family_variant() {
        let registry = BTreeSet::from_iter(DecisionCode::all().iter().copied());

        assert!(
            CapabilityCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Capability(*code)))
        );
        assert!(
            InputCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Input(*code)))
        );
        assert!(
            AcceptedCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Accepted(*code)))
        );
        assert!(
            DropCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Dropped(*code)))
        );
        assert!(
            RejectionCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Rejected(*code)))
        );
        assert!(
            SoftFailureCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::SoftFailed(*code)))
        );
        assert!(
            crate::DependencyCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Dependency(*code)))
        );
        assert!(
            HaltCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Halt(*code)))
        );
        assert!(
            ConsequenceCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Consequence(*code)))
        );
        assert!(
            FaultCode::all()
                .iter()
                .all(|code| registry.contains(&DecisionCode::Fault(*code)))
        );
    }
}

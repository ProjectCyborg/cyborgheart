//! Public domain vocabulary for CyborgHeart room-event application.
//!
//! This crate deliberately exposes no callable evaluator in the contract-seeding
//! slice. It provides the stable types needed to keep unsupported capabilities,
//! missing dependencies, bounded halts, faults, and Matrix dispositions separate.

pub mod budget;
pub mod consequences;
pub mod decision;
pub mod dependencies;
pub mod evidence;
pub mod input;
pub mod room_version;

pub use budget::{EvaluationBudget, WorkDimension, WorkReport};
pub use consequences::{
    ForwardExtremityEffect, HistoricalStateEffect, RedactionEffect, SemanticConsequences,
    VisibilityClass,
};
pub use decision::{
    AcceptedCode, AdmittedDecision, AdmittedOutcome, CapabilityCode, CompleteDecision,
    DecisionCode, DependencyNeed, Disposition, DropCode, DroppedDecision, EvaluationFault,
    EvaluationResult, FaultCode, Halt, HaltCode, InputCode, LifecycleStage, RejectionCode,
    SoftFailureCode, known_decision_code,
};
pub use dependencies::{
    Dependency, DependencyCode, DependencyRequest, DependencyRequestBuildError,
    DependencyRequestSet,
};
pub use evidence::{DecisionEvidence, EvidenceDetail};
pub use input::{
    CandidateEvent, EvaluationInput, EvaluationOptions, EventProvenance, EvidenceLevel,
};
pub use room_version::{SupportedRoomVersion, UnsupportedRoomVersion};

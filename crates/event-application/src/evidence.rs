//! Structured decision evidence types.

use std::collections::{BTreeMap, BTreeSet};

use crate::{WorkReport, decision::LifecycleStage};

/// Stable detail map attached to evidence.
pub type EvidenceDetail = BTreeMap<String, String>;

/// Inspectable evidence for a decision, halt, dependency request, or fault.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DecisionEvidence {
    completed_stages: BTreeSet<LifecycleStage>,
    details: EvidenceDetail,
    work: WorkReport,
}

impl DecisionEvidence {
    /// Creates empty decision evidence.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            completed_stages: BTreeSet::new(),
            details: BTreeMap::new(),
            work: WorkReport::new(),
        }
    }

    /// Creates evidence from normalized parts.
    #[must_use]
    pub fn from_parts(
        completed_stages: impl IntoIterator<Item = LifecycleStage>,
        details: EvidenceDetail,
        work: WorkReport,
    ) -> Self {
        Self {
            completed_stages: completed_stages.into_iter().collect(),
            details,
            work,
        }
    }

    /// Completed lifecycle stages in stable order.
    #[must_use]
    pub const fn completed_stages(&self) -> &BTreeSet<LifecycleStage> {
        &self.completed_stages
    }

    /// Stable evidence details.
    #[must_use]
    pub const fn details(&self) -> &EvidenceDetail {
        &self.details
    }

    /// Work report associated with the evidence.
    #[must_use]
    pub const fn work(&self) -> &WorkReport {
        &self.work
    }
}

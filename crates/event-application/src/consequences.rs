//! Semantic consequence vocabulary.

use std::collections::BTreeSet;

use crate::decision::ConsequenceCode;

/// Visibility class derived from the event disposition.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VisibilityClass {
    /// Normally visible accepted event.
    Normal,
    /// Rejected-event visibility.
    Rejected,
    /// Soft-failed-event visibility.
    SoftFailed,
}

/// Historical state-after effect at the candidate's historical position.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HistoricalStateEffect {
    /// Historical state after is equal to historical state before.
    Unchanged,
    /// A state tuple is updated by the admitted state event.
    StateTupleUpdated,
}

/// Immediate local forward-extremity effect.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ForwardExtremityEffect {
    /// Candidate is added as a local forward extremity.
    Added,
    /// Candidate is not added as a local forward extremity.
    NotAdded,
}

/// Redaction target-effect status.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RedactionEffect {
    /// Candidate is not a redaction event with a tracked target effect.
    NotApplicable,
    /// Redaction target effect is applied.
    Applied,
    /// Redaction target effect remains pending a qualifying partner.
    PendingPartner,
}

/// Semantic consequences of a completed admitted event decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticConsequences {
    historical_state: HistoricalStateEffect,
    forward_extremity: ForwardExtremityEffect,
    redaction: RedactionEffect,
}

impl SemanticConsequences {
    /// Creates consequences from mutually exclusive typed dimensions.
    pub(crate) const fn from_effects(
        historical_state: HistoricalStateEffect,
        forward_extremity: ForwardExtremityEffect,
        redaction: RedactionEffect,
    ) -> Self {
        Self {
            historical_state,
            forward_extremity,
            redaction,
        }
    }

    /// Historical state-after effect.
    #[must_use]
    pub const fn historical_state(&self) -> HistoricalStateEffect {
        self.historical_state
    }

    /// Immediate forward-extremity effect.
    #[must_use]
    pub const fn forward_extremity(&self) -> ForwardExtremityEffect {
        self.forward_extremity
    }

    /// Redaction target-effect status.
    #[must_use]
    pub const fn redaction(&self) -> RedactionEffect {
        self.redaction
    }

    /// Stable consequence codes projected in deterministic order.
    #[must_use]
    pub fn codes(&self) -> BTreeSet<ConsequenceCode> {
        let mut codes = BTreeSet::new();

        codes.insert(match self.historical_state {
            HistoricalStateEffect::Unchanged => ConsequenceCode::StateUnchanged,
            HistoricalStateEffect::StateTupleUpdated => ConsequenceCode::StateUpdated,
        });

        codes.insert(match self.forward_extremity {
            ForwardExtremityEffect::Added => ConsequenceCode::ForwardExtremityAdded,
            ForwardExtremityEffect::NotAdded => ConsequenceCode::ForwardExtremityNotAdded,
        });

        match self.redaction {
            RedactionEffect::NotApplicable => {}
            RedactionEffect::Applied => {
                codes.insert(ConsequenceCode::RedactionApplied);
            }
            RedactionEffect::PendingPartner => {
                codes.insert(ConsequenceCode::RedactionPendingPartner);
            }
        }

        codes
    }
}

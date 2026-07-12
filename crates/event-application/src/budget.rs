//! Evaluation budget and work accounting types.

use std::collections::BTreeMap;

/// Closed set of work dimensions that can be budgeted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WorkDimension {
    /// Bytes in canonical JSON input.
    CanonicalJsonBytes,
    /// Event facts inspected.
    EventsInspected,
    /// State entries inspected.
    StateEntriesInspected,
    /// Auth events inspected.
    AuthEventsInspected,
    /// Auth-chain graph edges traversed.
    AuthChainEdges,
    /// Room DAG graph edges traversed.
    GraphEdges,
    /// Signature verification operations attempted.
    SignatureVerifications,
    /// State-resolution steps performed.
    StateResolutionSteps,
}

impl WorkDimension {
    /// Stable fixture and evidence spelling for the dimension.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalJsonBytes => "canonical_json_bytes",
            Self::EventsInspected => "events_inspected",
            Self::StateEntriesInspected => "state_entries_inspected",
            Self::AuthEventsInspected => "auth_events_inspected",
            Self::AuthChainEdges => "auth_chain_edges",
            Self::GraphEdges => "graph_edges",
            Self::SignatureVerifications => "signature_verifications",
            Self::StateResolutionSteps => "state_resolution_steps",
        }
    }

    /// Parses a stable work-dimension spelling.
    #[must_use]
    pub fn from_str_name(name: &str) -> Option<Self> {
        match name {
            "canonical_json_bytes" => Some(Self::CanonicalJsonBytes),
            "events_inspected" => Some(Self::EventsInspected),
            "state_entries_inspected" => Some(Self::StateEntriesInspected),
            "auth_events_inspected" => Some(Self::AuthEventsInspected),
            "auth_chain_edges" => Some(Self::AuthChainEdges),
            "graph_edges" => Some(Self::GraphEdges),
            "signature_verifications" => Some(Self::SignatureVerifications),
            "state_resolution_steps" => Some(Self::StateResolutionSteps),
            _ => None,
        }
    }

    /// All supported work dimensions in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::CanonicalJsonBytes,
            Self::EventsInspected,
            Self::StateEntriesInspected,
            Self::AuthEventsInspected,
            Self::AuthChainEdges,
            Self::GraphEdges,
            Self::SignatureVerifications,
            Self::StateResolutionSteps,
        ]
    }
}

/// Declared limits for untrusted work.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EvaluationBudget {
    limits: BTreeMap<WorkDimension, u64>,
}

impl EvaluationBudget {
    /// Creates an empty budget.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: BTreeMap::new(),
        }
    }

    /// Creates a budget from dimension limits, normalizing by dimension.
    #[must_use]
    pub fn from_limits(limits: impl IntoIterator<Item = (WorkDimension, u64)>) -> Self {
        Self {
            limits: limits.into_iter().collect(),
        }
    }

    /// Sets a limit and returns the updated budget.
    #[must_use]
    pub fn with_limit(mut self, dimension: WorkDimension, limit: u64) -> Self {
        self.limits.insert(dimension, limit);
        self
    }

    /// Returns a limit for a dimension when configured.
    #[must_use]
    pub fn limit(&self, dimension: WorkDimension) -> Option<u64> {
        self.limits.get(&dimension).copied()
    }

    /// Returns configured limits in stable dimension order.
    #[must_use]
    pub const fn limits(&self) -> &BTreeMap<WorkDimension, u64> {
        &self.limits
    }
}

/// Consumed work reported by evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkReport {
    consumed: BTreeMap<WorkDimension, u64>,
}

impl WorkReport {
    /// Creates an empty work report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            consumed: BTreeMap::new(),
        }
    }

    /// Creates a report from consumed work, normalizing by dimension.
    #[must_use]
    pub fn from_consumed(consumed: impl IntoIterator<Item = (WorkDimension, u64)>) -> Self {
        Self {
            consumed: consumed.into_iter().collect(),
        }
    }

    /// Adds a consumed amount to a dimension and returns the updated report.
    #[must_use]
    pub fn with_consumed(mut self, dimension: WorkDimension, amount: u64) -> Self {
        self.consumed.insert(dimension, amount);
        self
    }

    /// Returns consumed work for a dimension when present.
    #[must_use]
    pub fn consumed(&self, dimension: WorkDimension) -> Option<u64> {
        self.consumed.get(&dimension).copied()
    }

    /// Returns consumed work in stable dimension order.
    #[must_use]
    pub const fn entries(&self) -> &BTreeMap<WorkDimension, u64> {
        &self.consumed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_dimensions_are_closed_and_parseable() {
        for dimension in WorkDimension::all() {
            assert_eq!(
                WorkDimension::from_str_name(dimension.as_str()),
                Some(*dimension)
            );
        }

        assert_eq!(WorkDimension::from_str_name("canonical_json_byte"), None);
    }
}

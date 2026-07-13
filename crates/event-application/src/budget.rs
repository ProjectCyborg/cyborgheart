//! Evaluation budget and work accounting types.

use std::{
    collections::BTreeMap,
    error::Error,
    fmt::{self, Display},
};

/// Closed set of work dimensions that can be budgeted.
#[non_exhaustive]
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

impl Display for WorkDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
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

    /// Creates a budget from dimension limits, keeping the last value per dimension.
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

    /// Returns the number of configured dimensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.limits.len()
    }

    /// Returns whether no limits are configured.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.limits.is_empty()
    }

    /// Iterates over configured limits in stable dimension order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (WorkDimension, u64)> + '_ {
        self.limits.iter().map(|(dimension, limit)| (*dimension, *limit))
    }
}

/// Error produced while accumulating consumed work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkReportOverflow {
    dimension: WorkDimension,
    current: u64,
    additional: u64,
}

impl WorkReportOverflow {
    /// Work dimension whose counter overflowed.
    #[must_use]
    pub const fn dimension(self) -> WorkDimension {
        self.dimension
    }

    /// Counter value before the failed addition.
    #[must_use]
    pub const fn current(self) -> u64 {
        self.current
    }

    /// Amount that could not be added.
    #[must_use]
    pub const fn additional(self) -> u64 {
        self.additional
    }
}

impl Display for WorkReportOverflow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "work counter overflow for {}: {} + {}",
            self.dimension, self.current, self.additional
        )
    }
}

impl Error for WorkReportOverflow {}

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

    /// Creates a report by checked accumulation of consumed work.
    pub fn try_from_consumed(
        consumed: impl IntoIterator<Item = (WorkDimension, u64)>,
    ) -> Result<Self, WorkReportOverflow> {
        let mut report = Self::new();
        for (dimension, amount) in consumed {
            report.add_consumed(dimension, amount)?;
        }
        Ok(report)
    }

    /// Adds a consumed amount to a dimension.
    pub fn add_consumed(
        &mut self,
        dimension: WorkDimension,
        amount: u64,
    ) -> Result<(), WorkReportOverflow> {
        let current = self.consumed(dimension).unwrap_or_default();
        let updated = current.checked_add(amount).ok_or(WorkReportOverflow {
            dimension,
            current,
            additional: amount,
        })?;
        self.consumed.insert(dimension, updated);
        Ok(())
    }

    /// Adds a consumed amount and returns the updated report.
    pub fn with_consumed(
        mut self,
        dimension: WorkDimension,
        amount: u64,
    ) -> Result<Self, WorkReportOverflow> {
        self.add_consumed(dimension, amount)?;
        Ok(self)
    }

    /// Returns consumed work for a dimension when present.
    #[must_use]
    pub fn consumed(&self, dimension: WorkDimension) -> Option<u64> {
        self.consumed.get(&dimension).copied()
    }

    /// Returns the number of reported dimensions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.consumed.len()
    }

    /// Returns whether no work has been reported.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.consumed.is_empty()
    }

    /// Iterates over consumed work in stable dimension order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (WorkDimension, u64)> + '_ {
        self.consumed
            .iter()
            .map(|(dimension, amount)| (*dimension, *amount))
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

    #[test]
    fn work_report_accumulates_duplicate_dimensions() {
        let report = WorkReport::try_from_consumed([
            (WorkDimension::EventsInspected, 2),
            (WorkDimension::EventsInspected, 3),
        ])
        .expect("small work counts do not overflow");

        assert_eq!(report.consumed(WorkDimension::EventsInspected), Some(5));
    }

    #[test]
    fn work_report_rejects_overflow() {
        let error = WorkReport::try_from_consumed([
            (WorkDimension::GraphEdges, u64::MAX),
            (WorkDimension::GraphEdges, 1),
        ])
        .expect_err("overflow must be explicit");

        assert_eq!(error.dimension(), WorkDimension::GraphEdges);
        assert_eq!(error.current(), u64::MAX);
        assert_eq!(error.additional(), 1);
    }

    #[test]
    fn iteration_is_stable() {
        let report = WorkReport::try_from_consumed([
            (WorkDimension::GraphEdges, 2),
            (WorkDimension::CanonicalJsonBytes, 1),
        ])
        .expect("small work counts do not overflow");

        assert_eq!(
            report.iter().collect::<Vec<_>>(),
            vec![
                (WorkDimension::CanonicalJsonBytes, 1),
                (WorkDimension::GraphEdges, 2),
            ]
        );
    }
}

//! Candidate event and evaluation input types.

use ruma::{CanonicalJsonObject, MilliSecondsSinceUnixEpoch, OwnedEventId};

use crate::room_version::SupportedRoomVersion;

/// Untrusted canonical candidate event supplied to the event-application layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateEvent {
    canonical: CanonicalJsonObject,
    asserted_event_id: Option<OwnedEventId>,
}

impl CandidateEvent {
    /// Creates a candidate from Matrix canonical JSON and an optional asserted ID.
    #[must_use]
    pub fn new(canonical: CanonicalJsonObject, asserted_event_id: Option<OwnedEventId>) -> Self {
        Self {
            canonical,
            asserted_event_id,
        }
    }

    /// Canonical JSON object admitted into the event-application boundary.
    #[must_use]
    pub const fn canonical(&self) -> &CanonicalJsonObject {
        &self.canonical
    }

    /// Optional externally asserted event ID to compare with the derived ID later.
    #[must_use]
    pub const fn asserted_event_id(&self) -> Option<&OwnedEventId> {
        self.asserted_event_id.as_ref()
    }
}

/// How a candidate event entered the host system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventProvenance {
    /// Received from a remote Matrix server.
    Federated,
    /// Constructed locally by a future command runtime.
    LocallyConstructed,
    /// Re-evaluated from retained data.
    Replay,
    /// Supplied by an import or migration path.
    Import,
}

/// Evidence volume requested from evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceLevel {
    /// Standard evidence required by the public contract.
    Standard,
}

/// Options that affect evidence shape without changing Matrix semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationOptions {
    evidence_level: EvidenceLevel,
}

impl EvaluationOptions {
    /// Creates evaluation options with the selected evidence level.
    #[must_use]
    pub const fn new(evidence_level: EvidenceLevel) -> Self {
        Self { evidence_level }
    }

    /// Evidence level requested by the caller.
    #[must_use]
    pub const fn evidence_level(self) -> EvidenceLevel {
        self.evidence_level
    }
}

impl Default for EvaluationOptions {
    fn default() -> Self {
        Self::new(EvidenceLevel::Standard)
    }
}

/// Complete explicit input envelope for a future event evaluation call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationInput {
    event: CandidateEvent,
    room_version: SupportedRoomVersion,
    provenance: EventProvenance,
    options: EvaluationOptions,
    key_validity_reference_ts: MilliSecondsSinceUnixEpoch,
}

impl EvaluationInput {
    /// Creates an explicit evaluation input envelope.
    #[must_use]
    pub const fn new(
        event: CandidateEvent,
        room_version: SupportedRoomVersion,
        provenance: EventProvenance,
        options: EvaluationOptions,
        key_validity_reference_ts: MilliSecondsSinceUnixEpoch,
    ) -> Self {
        Self {
            event,
            room_version,
            provenance,
            options,
            key_validity_reference_ts,
        }
    }

    /// Candidate event.
    #[must_use]
    pub const fn event(&self) -> &CandidateEvent {
        &self.event
    }

    /// Supported room version selected at the capability boundary.
    #[must_use]
    pub const fn room_version(&self) -> SupportedRoomVersion {
        self.room_version
    }

    /// Candidate provenance.
    #[must_use]
    pub const fn provenance(&self) -> EventProvenance {
        self.provenance
    }

    /// Evaluation options.
    #[must_use]
    pub const fn options(&self) -> EvaluationOptions {
        self.options
    }

    /// Frozen timestamp for signing-key validity decisions.
    #[must_use]
    pub const fn key_validity_reference_ts(&self) -> MilliSecondsSinceUnixEpoch {
        self.key_validity_reference_ts
    }
}

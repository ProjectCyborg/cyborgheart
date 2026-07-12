//! Dependency request domain types.

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::{self, Display},
};

use ruma::{MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedServerName};

use crate::decision::LifecycleStage;

/// Stable machine code for an incomplete dependency request.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyCode {
    /// One or more event facts are required.
    Event,
    /// State-after facts for predecessor events are required.
    StateAfter,
    /// A frozen current-room-state fact is required.
    CurrentRoomState,
    /// Public signing-key facts are required.
    SigningKeys,
    /// A Policy Server recommendation fact is required.
    PolicyRecommendation,
}

impl DependencyCode {
    /// All dependency request codes in stable order.
    pub const ALL: [Self; 5] = [
        Self::Event,
        Self::StateAfter,
        Self::CurrentRoomState,
        Self::SigningKeys,
        Self::PolicyRecommendation,
    ];

    /// Stable dotted machine code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Event => "need.event",
            Self::StateAfter => "need.state_after",
            Self::CurrentRoomState => "need.current_room_state",
            Self::SigningKeys => "need.signing_keys",
            Self::PolicyRecommendation => "need.policy_recommendation",
        }
    }

    /// All dependency request codes in stable order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &Self::ALL
    }
}

/// Identity of an immutable fact needed to continue evaluation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dependency {
    /// Event fact by event ID.
    Event {
        /// Event ID to provide.
        event_id: OwnedEventId,
    },
    /// State-after fact by predecessor event ID.
    StateAfter {
        /// Event ID whose state-after map is required.
        event_id: OwnedEventId,
    },
    /// Current room-state fact.
    CurrentRoomState {
        /// Room ID for the frozen current state.
        room_id: OwnedRoomId,
        /// Optional expected revision identifier.
        revision: Option<String>,
    },
    /// Signing-key facts for one server and candidate key set.
    SigningKeys {
        /// Candidate event ID when already derivable.
        candidate_event_id: Option<OwnedEventId>,
        /// Server whose signing keys are required.
        server_name: OwnedServerName,
        /// Candidate key IDs in stable order.
        key_ids: BTreeSet<String>,
        /// Event origin timestamp.
        event_origin_server_ts: MilliSecondsSinceUnixEpoch,
        /// Frozen key-validity reference timestamp.
        key_validity_reference_ts: MilliSecondsSinceUnixEpoch,
    },
    /// Policy Server recommendation fact.
    PolicyRecommendation {
        /// Candidate event ID.
        candidate_event_id: OwnedEventId,
        /// Active policy event ID.
        policy_event_id: OwnedEventId,
        /// Policy fingerprint.
        policy_fingerprint: String,
    },
}

impl Dependency {
    /// Returns the stable dependency request code for this dependency.
    #[must_use]
    pub const fn code(&self) -> DependencyCode {
        match self {
            Self::Event { .. } => DependencyCode::Event,
            Self::StateAfter { .. } => DependencyCode::StateAfter,
            Self::CurrentRoomState { .. } => DependencyCode::CurrentRoomState,
            Self::SigningKeys { .. } => DependencyCode::SigningKeys,
            Self::PolicyRecommendation { .. } => DependencyCode::PolicyRecommendation,
        }
    }
}

/// Contract-construction error for dependency request values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyRequestBuildError {
    /// Request contains dependencies from more than one stable code family.
    MixedDependencyCodes {
        /// First dependency code accepted by the request.
        expected: DependencyCode,
        /// Dependency code that contradicted the request family.
        actual: DependencyCode,
    },
}

impl Display for DependencyRequestBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MixedDependencyCodes { expected, actual } => write!(
                formatter,
                "dependency request mixed {} with {}",
                expected.as_str(),
                actual.as_str()
            ),
        }
    }
}

impl Error for DependencyRequestBuildError {}

/// A deterministic non-empty dependency request discovered at one lifecycle stage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyRequest {
    stage: LifecycleStage,
    code: DependencyCode,
    dependencies: BTreeSet<Dependency>,
}

impl DependencyRequest {
    /// Creates a non-empty request and rejects mixed dependency-code families.
    pub fn new(
        stage: LifecycleStage,
        first: Dependency,
        remaining: impl IntoIterator<Item = Dependency>,
    ) -> Result<Self, DependencyRequestBuildError> {
        let code = first.code();
        let mut dependencies = BTreeSet::from([first]);

        for dependency in remaining {
            let actual = dependency.code();
            if actual != code {
                return Err(DependencyRequestBuildError::MixedDependencyCodes {
                    expected: code,
                    actual,
                });
            }
            dependencies.insert(dependency);
        }

        Ok(Self {
            stage,
            code,
            dependencies,
        })
    }

    /// Creates a request for one dependency.
    #[must_use]
    pub fn single(stage: LifecycleStage, dependency: Dependency) -> Self {
        let code = dependency.code();
        Self {
            stage,
            code,
            dependencies: BTreeSet::from([dependency]),
        }
    }

    /// Lifecycle stage that discovered this dependency request.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        self.stage
    }

    /// Stable dependency request code.
    #[must_use]
    pub const fn code(&self) -> DependencyCode {
        self.code
    }

    /// Dependency identities in stable order.
    #[must_use]
    pub const fn dependencies(&self) -> &BTreeSet<Dependency> {
        &self.dependencies
    }

    fn merge(&mut self, other: DependencyRequest) {
        debug_assert_eq!(self.stage, other.stage);
        debug_assert_eq!(self.code, other.code);
        self.dependencies.extend(other.dependencies);
    }
}

/// Deterministic non-empty set of dependency requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyRequestSet {
    requests: BTreeSet<DependencyRequest>,
}

impl DependencyRequestSet {
    /// Creates a non-empty set and merges duplicate `(stage, code)` request groups.
    #[must_use]
    pub fn new(
        first: DependencyRequest,
        remaining: impl IntoIterator<Item = DependencyRequest>,
    ) -> Self {
        let mut by_group: BTreeMap<(LifecycleStage, DependencyCode), DependencyRequest> =
            BTreeMap::new();

        for request in std::iter::once(first).chain(remaining) {
            match by_group.entry((request.stage(), request.code())) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(request);
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().merge(request);
                }
            }
        }

        Self {
            requests: by_group.into_values().collect(),
        }
    }

    /// Creates a request set for one request.
    #[must_use]
    pub fn single(request: DependencyRequest) -> Self {
        Self::new(request, [])
    }

    /// Requests in stable order.
    #[must_use]
    pub const fn requests(&self) -> &BTreeSet<DependencyRequest> {
        &self.requests
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ruma::{MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId, OwnedServerName};

    use super::*;

    fn event_id(id: &str) -> OwnedEventId {
        OwnedEventId::from_str(id).expect("valid event id")
    }

    fn room_id(id: &str) -> OwnedRoomId {
        OwnedRoomId::from_str(id).expect("valid room id")
    }

    fn server_name(name: &str) -> OwnedServerName {
        OwnedServerName::from_str(name).expect("valid server name")
    }

    fn event_dependency(id: &str) -> Dependency {
        Dependency::Event {
            event_id: event_id(id),
        }
    }

    fn state_after_dependency(id: &str) -> Dependency {
        Dependency::StateAfter {
            event_id: event_id(id),
        }
    }

    #[test]
    fn request_is_non_empty_and_has_direct_code() {
        let request = DependencyRequest::single(
            LifecycleStage::ClaimedAuth,
            event_dependency("$a:example.org"),
        );

        assert_eq!(request.code(), DependencyCode::Event);
        assert_eq!(request.dependencies().len(), 1);
    }

    #[test]
    fn duplicate_dependencies_are_deduplicated() {
        let request = DependencyRequest::new(
            LifecycleStage::ClaimedAuth,
            event_dependency("$a:example.org"),
            [event_dependency("$a:example.org")],
        )
        .expect("same dependency family");

        assert_eq!(request.dependencies().len(), 1);
    }

    #[test]
    fn mixed_dependency_families_are_rejected() {
        let result = DependencyRequest::new(
            LifecycleStage::ClaimedAuth,
            event_dependency("$a:example.org"),
            [state_after_dependency("$b:example.org")],
        );

        assert!(matches!(
            result,
            Err(DependencyRequestBuildError::MixedDependencyCodes {
                expected: DependencyCode::Event,
                actual: DependencyCode::StateAfter,
            })
        ));
    }

    #[test]
    fn request_sets_are_non_empty_and_merge_duplicate_groups() {
        let first = DependencyRequest::single(
            LifecycleStage::ClaimedAuth,
            event_dependency("$a:example.org"),
        );
        let second = DependencyRequest::single(
            LifecycleStage::ClaimedAuth,
            event_dependency("$b:example.org"),
        );
        let third = DependencyRequest::single(
            LifecycleStage::StateBefore,
            state_after_dependency("$c:example.org"),
        );

        let set = DependencyRequestSet::new(first, [third, second]);
        let requests = set.requests().iter().collect::<Vec<_>>();

        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].stage(), LifecycleStage::ClaimedAuth);
        assert_eq!(requests[0].code(), DependencyCode::Event);
        assert_eq!(requests[0].dependencies().len(), 2);
        assert_eq!(requests[1].stage(), LifecycleStage::StateBefore);
        assert_eq!(requests[1].code(), DependencyCode::StateAfter);
    }

    #[test]
    fn ordering_is_stable_under_shuffled_input() {
        let signing = Dependency::SigningKeys {
            candidate_event_id: None,
            server_name: server_name("example.org"),
            key_ids: BTreeSet::from(["ed25519:1".to_owned(), "ed25519:2".to_owned()]),
            event_origin_server_ts: MilliSecondsSinceUnixEpoch(1_u32.into()),
            key_validity_reference_ts: MilliSecondsSinceUnixEpoch(1_u32.into()),
        };
        let current = Dependency::CurrentRoomState {
            room_id: room_id("!room:example.org"),
            revision: Some("r1".to_owned()),
        };

        let a = DependencyRequestSet::new(
            DependencyRequest::single(LifecycleStage::SignerDiscovery, signing.clone()),
            [DependencyRequest::single(
                LifecycleStage::CurrentStateAuth,
                current.clone(),
            )],
        );
        let b = DependencyRequestSet::new(
            DependencyRequest::single(LifecycleStage::CurrentStateAuth, current),
            [DependencyRequest::single(
                LifecycleStage::SignerDiscovery,
                signing,
            )],
        );

        assert_eq!(
            a.requests().iter().collect::<Vec<_>>(),
            b.requests().iter().collect::<Vec<_>>()
        );
    }
}

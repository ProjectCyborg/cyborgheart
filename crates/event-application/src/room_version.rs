//! Supported room-version admission types.

use crate::decision::{CapabilityCode, LifecycleStage};

/// A Matrix room version explicitly supported by CyborgHeart event application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SupportedRoomVersion {
    /// Matrix room version 12.
    V12,
}

impl SupportedRoomVersion {
    /// Returns the Matrix room-version identifier for this supported version.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V12 => "12",
        }
    }

    /// Returns the ordered set of supported room-version identifiers.
    #[must_use]
    pub const fn supported_identifiers() -> &'static [&'static str] {
        &["12"]
    }

    /// Admits an external room-version identifier into the supported boundary.
    ///
    /// Unsupported versions fail before event evaluation and must not be mapped
    /// to a Matrix event disposition.
    pub fn admit(identifier: &str) -> Result<Self, UnsupportedRoomVersion> {
        match identifier {
            "12" => Ok(Self::V12),
            requested => Err(UnsupportedRoomVersion::new(requested)),
        }
    }
}

/// Capability-boundary error for an unsupported external room version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedRoomVersion {
    requested: String,
}

impl UnsupportedRoomVersion {
    /// Creates an unsupported room-version error for the requested identifier.
    #[must_use]
    pub fn new(requested: impl Into<String>) -> Self {
        Self {
            requested: requested.into(),
        }
    }

    /// Stable machine code for this capability error.
    #[must_use]
    pub const fn code(&self) -> CapabilityCode {
        CapabilityCode::UnsupportedRoomVersion
    }

    /// Lifecycle stage at which the capability error occurs.
    #[must_use]
    pub const fn stage(&self) -> LifecycleStage {
        LifecycleStage::RoomVersionAdmission
    }

    /// The external room-version identifier that was requested.
    #[must_use]
    pub fn requested(&self) -> &str {
        &self.requested
    }

    /// Supported room-version identifiers.
    #[must_use]
    pub const fn supported(&self) -> &'static [&'static str] {
        SupportedRoomVersion::supported_identifiers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_room_version_12_is_admitted() {
        assert_eq!(
            SupportedRoomVersion::admit("12"),
            Ok(SupportedRoomVersion::V12)
        );

        let unsupported = SupportedRoomVersion::admit("11").expect_err("11 is unsupported");
        assert_eq!(
            unsupported.code().as_str(),
            "capability.unsupported_room_version"
        );
        assert_eq!(unsupported.stage(), LifecycleStage::RoomVersionAdmission);
        assert_eq!(unsupported.supported(), ["12"]);
    }
}

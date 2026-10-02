//! Shared domain types for Abhilekh.
//!
//! The types in this crate are deliberately small and transport-friendly. IDs
//! are UUIDv7 values, timestamps are RFC 3339 strings on the wire, and errors
//! expose stable machine-readable categories for API and event-log consumers.

use std::{fmt, str::FromStr};

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use thiserror::Error;
use uuid::Uuid;

/// The name of this crate.
pub const CRATE_NAME: &str = "abhilekh-types";

macro_rules! define_id {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Creates a new UUIDv7 identifier using the current system time.
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            /// Wraps an existing UUID without changing it.
            #[must_use]
            pub const fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Returns the wrapped UUID by reference.
            #[must_use]
            pub const fn as_uuid(&self) -> &Uuid {
                &self.0
            }

            /// Returns the wrapped UUID.
            #[must_use]
            pub const fn into_uuid(self) -> Uuid {
                self.0
            }

            /// Parses a hyphenated or simple UUID string.
            pub fn parse(value: &str) -> Result<Self, uuid::Error> {
                value.parse().map(Self)
            }

            /// Returns whether this identifier contains the nil UUID.
            #[must_use]
            pub const fn is_nil(&self) -> bool {
                self.0.is_nil()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl AsRef<Uuid> for $name {
            fn as_ref(&self) -> &Uuid {
                self.as_uuid()
            }
        }

        impl From<Uuid> for $name {
            fn from(uuid: Uuid) -> Self {
                Self::from_uuid(uuid)
            }
        }

        impl From<$name> for Uuid {
            fn from(id: $name) -> Self {
                id.into_uuid()
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

define_id!(
    /// The identifier of an episode in the memory log.
    EpisodeId
);

define_id!(
    /// The identifier of a fact in the knowledge graph.
    FactId
);

define_id!(
    /// The identifier of an entity in the knowledge graph.
    EntityId
);

/// A UTC instant represented as RFC 3339 text when serialized.
///
/// Parsing accepts RFC 3339 offsets and normalizes the resulting value to UTC.
/// Serialization always uses `Z` for UTC, so equivalent offset timestamps have
/// one canonical representation in JSON and event-log records.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Returns the current UTC time.
    #[must_use]
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Wraps a UTC [`DateTime`].
    #[must_use]
    pub const fn from_datetime(datetime: DateTime<Utc>) -> Self {
        Self(datetime)
    }

    /// Parses an RFC 3339 timestamp.
    pub fn parse(value: &str) -> Result<Self, chrono::ParseError> {
        DateTime::parse_from_rfc3339(value).map(|datetime| Self(datetime.with_timezone(&Utc)))
    }

    /// Parses an RFC 3339 timestamp.
    pub fn from_rfc3339(value: &str) -> Result<Self, chrono::ParseError> {
        Self::parse(value)
    }

    /// Returns the wrapped UTC [`DateTime`].
    #[must_use]
    pub const fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Returns the wrapped UTC [`DateTime`].
    #[must_use]
    pub const fn into_datetime(self) -> DateTime<Utc> {
        self.0
    }

    /// Formats this timestamp as canonical RFC 3339 text.
    #[must_use]
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(SecondsFormat::AutoSi, true)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(datetime: DateTime<Utc>) -> Self {
        Self::from_datetime(datetime)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.into_datetime()
    }
}

impl FromStr for Timestamp {
    type Err = chrono::ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_rfc3339())
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_rfc3339())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// Machine-readable errors returned by Abhilekh operations.
///
/// The [`Self::code`] value is stable and suitable for API responses and
/// event-log records. The message is intended for humans and may change as
/// validation becomes more specific.
#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum AbhilekhError {
    /// The request contains invalid or incomplete data.
    #[error("invalid input: {0}")]
    #[serde(rename = "invalid_input")]
    InvalidInput(String),

    /// The requested resource does not exist.
    #[error("not found: {0}")]
    #[serde(rename = "not_found")]
    NotFound(String),

    /// The request conflicts with the current state.
    #[error("conflict: {0}")]
    #[serde(rename = "conflict")]
    Conflict(String),

    /// Another writer currently holds the resource lock.
    #[error("locked: {0}")]
    #[serde(rename = "locked")]
    Locked(String),

    /// An unexpected internal failure occurred.
    #[error("internal error: {0}")]
    #[serde(rename = "internal")]
    Internal(String),
}

impl AbhilekhError {
    /// Creates an [`AbhilekhError::InvalidInput`] error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput(message.into())
    }

    /// Creates an [`AbhilekhError::NotFound`] error.
    #[must_use]
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    /// Creates an [`AbhilekhError::Conflict`] error.
    #[must_use]
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    /// Creates an [`AbhilekhError::Locked`] error.
    #[must_use]
    pub fn locked(message: impl Into<String>) -> Self {
        Self::Locked(message.into())
    }

    /// Creates an [`AbhilekhError::Internal`] error.
    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Returns the stable machine-readable category.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidInput(_) => "invalid_input",
            Self::NotFound(_) => "not_found",
            Self::Conflict(_) => "conflict",
            Self::Locked(_) => "locked",
            Self::Internal(_) => "internal",
        }
    }

    /// Returns the human-readable detail associated with this error.
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidInput(message)
            | Self::NotFound(message)
            | Self::Conflict(message)
            | Self::Locked(message)
            | Self::Internal(message) => message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};

    #[test]
    fn ids_are_unique() {
        let ids: std::collections::HashSet<_> = (0..1_000).map(|_| EpisodeId::new()).collect();

        assert_eq!(ids.len(), 1_000);
    }

    #[test]
    fn ids_sort_by_creation_time() {
        let ids: Vec<_> = (0..32).map(|_| EpisodeId::new()).collect();

        assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(ids, {
            let mut sorted = ids.clone();
            sorted.sort();
            sorted
        });
    }

    #[test]
    fn timestamp_survives_json_round_trip() {
        let timestamp = Timestamp::from_datetime(
            Utc.with_ymd_and_hms(2026, 10, 2, 12, 34, 56)
                .single()
                .expect("valid timestamp")
                .with_nanosecond(123_000_000)
                .expect("valid nanoseconds"),
        );

        let json = serde_json::to_string(&timestamp).expect("timestamp serializes");
        assert_eq!(json, "\"2026-10-02T12:34:56.123Z\"");

        let decoded: Timestamp = serde_json::from_str(&json).expect("timestamp deserializes");
        assert_eq!(decoded, timestamp);
    }

    #[test]
    fn timestamp_rejects_invalid_json_input() {
        assert!(serde_json::from_str::<Timestamp>("\"not a timestamp\"").is_err());
        assert!(Timestamp::parse("not a timestamp").is_err());
    }

    #[test]
    fn errors_expose_stable_codes() {
        let errors = [
            AbhilekhError::invalid_input("bad value"),
            AbhilekhError::not_found("missing value"),
            AbhilekhError::conflict("already exists"),
            AbhilekhError::locked("writer is active"),
            AbhilekhError::internal("unexpected failure"),
        ];
        let codes: Vec<_> = errors.iter().map(AbhilekhError::code).collect();

        assert_eq!(
            codes,
            [
                "invalid_input",
                "not_found",
                "conflict",
                "locked",
                "internal"
            ]
        );
    }
}

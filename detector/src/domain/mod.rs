//! Code-level contracts for values the database stores as plain text.
//! SQLite keeps the text; these types are the single source of truth for what is allowed.
//! Nothing here touches the database or the filesystem.

pub mod plan;
pub mod time;

use std::collections::BTreeMap;
use std::path::PathBuf;

/// The one canonical shape every Scanner normalizes into,
/// regardless of what source or platform produced it.
#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveredTool {
    pub identifier: String,
    pub name: String,
    pub version: Option<String>,
    pub path: PathBuf,
    /// When the source says it was really installed (Unix seconds), if it knows.
    pub installed_at: Option<i64>,
}

/// Whether a tool we failed to find is really gone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Absence {
    Gone,
    StillThere,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Installed,
    Updated,
    Uninstalled,
}

impl EventType {
    pub fn as_str(self) -> &'static str {
        match self {
            EventType::Installed => "installed",
            EventType::Updated => "updated",
            EventType::Uninstalled => "uninstalled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "installed" => Some(EventType::Installed),
            "updated" => Some(EventType::Updated),
            "uninstalled" => Some(EventType::Uninstalled),
            _ => None,
        }
    }
}

/// Whether a tool or source is currently on the machine: the outcome of its latest event.
/// An `updated` event leaves it `Installed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Installed,
    Uninstalled,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Installed => "installed",
            Status::Uninstalled => "uninstalled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "installed" => Some(Status::Installed),
            "uninstalled" => Some(Status::Uninstalled),
            _ => None,
        }
    }
}

/// Why a scan ran. Only `Manual` exists until the other triggers
/// (watcher, scheduled, startup) are built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggeredBy {
    Manual,
    /// A value written by a newer version of Legacy that this build doesn't know.
    Unknown(String),
}

impl TriggeredBy {
    pub fn as_str(&self) -> &str {
        match self {
            TriggeredBy::Manual => "manual",
            TriggeredBy::Unknown(raw) => raw,
        }
    }

    pub fn parse(raw: &str) -> Self {
        match raw {
            "manual" => TriggeredBy::Manual,
            other => TriggeredBy::Unknown(other.to_string()),
        }
    }
}

/// A field whose change is worth recording. Version only, for now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrackedField {
    Version,
}

impl TrackedField {
    pub fn as_str(self) -> &'static str {
        match self {
            TrackedField::Version => "version",
        }
    }
}

/// What changed, as `{field: [old, new]}`. A missing side is null.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Changes(BTreeMap<TrackedField, (Option<String>, Option<String>)>);

impl Changes {
    fn version(old: Option<&str>, new: Option<&str>) -> Self {
        let mut map = BTreeMap::new();
        map.insert(
            TrackedField::Version,
            (old.map(String::from), new.map(String::from)),
        );
        Changes(map)
    }

    pub fn installed(version: Option<&str>) -> Self {
        Self::version(None, version)
    }

    pub fn updated(old: Option<&str>, new: Option<&str>) -> Self {
        Self::version(old, new)
    }

    pub fn uninstalled(last_version: Option<&str>) -> Self {
        Self::version(last_version, None)
    }

    pub fn to_json(&self) -> String {
        let by_name: BTreeMap<&str, &(Option<String>, Option<String>)> =
            self.0.iter().map(|(field, pair)| (field.as_str(), pair)).collect();
        serde_json::to_string(&by_name).expect("changes always serialize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_type_round_trips_and_rejects_unknown() {
        for event in [EventType::Installed, EventType::Updated, EventType::Uninstalled] {
            assert_eq!(EventType::parse(event.as_str()), Some(event));
        }
        assert_eq!(EventType::parse("renamed"), None);
    }

    #[test]
    fn status_round_trips_and_rejects_unknown() {
        for status in [Status::Installed, Status::Uninstalled] {
            assert_eq!(Status::parse(status.as_str()), Some(status));
        }
        assert_eq!(Status::parse("present"), None);
        assert_eq!(Status::parse("updated"), None);
    }

    #[test]
    fn triggered_by_keeps_unknown_values_instead_of_failing() {
        assert_eq!(TriggeredBy::parse("manual"), TriggeredBy::Manual);
        let future = TriggeredBy::parse("watcher");
        assert_eq!(future, TriggeredBy::Unknown("watcher".to_string()));
        assert_eq!(future.as_str(), "watcher");
    }

    #[test]
    fn changes_json_uses_old_new_pairs_with_null_for_a_missing_side() {
        assert_eq!(
            Changes::installed(Some("1.1")).to_json(),
            r#"{"version":[null,"1.1"]}"#
        );
        assert_eq!(
            Changes::updated(Some("1.0"), Some("1.1")).to_json(),
            r#"{"version":["1.0","1.1"]}"#
        );
        assert_eq!(
            Changes::uninstalled(Some("1.1")).to_json(),
            r#"{"version":["1.1",null]}"#
        );
    }

    #[test]
    fn changes_json_stays_valid_when_the_version_is_unknown() {
        assert_eq!(Changes::installed(None).to_json(), r#"{"version":[null,null]}"#);
    }
}

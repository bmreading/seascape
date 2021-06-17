//! Items specific to an Activity Log Entry
use serde::{Deserialize, Serialize};

/// Represents an entry in the activity log.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActivityLogEntry {
    /// The id of this instance.
    #[serde(rename = "Id")]
    pub id: i64,
    /// The name.
    #[serde(rename = "Name")]
    pub name: String,
    /// The type.
    #[serde(rename = "Type")]
    pub entry_type: String,
    /// The date
    #[serde(rename = "Date")]
    pub date: String,
    /// The user identifier.
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// The log severity.
    #[serde(rename = "Severity")]
    pub severity: ActivityLogSeverity,
}

/// Represents severity log levels.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ActivityLogSeverity {
    Trace = 0,
    Debug = 1,
    Information = 2,
    Warning = 3,
    Error = 4,
    Critical = 5,
    None = 6,
}

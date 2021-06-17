use serde::{Deserialize, Serialize};

/// SyncPlayUserAccessType : Enum SyncPlayUserAccessType.

/// Enum SyncPlayUserAccessType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyncPlayUserAccessType {
    #[serde(rename = "CreateAndJoinGroups")]
    CreateAndJoinGroups,
    #[serde(rename = "JoinGroups")]
    JoinGroups,
    #[serde(rename = "None")]
    None,
}

impl ToString for SyncPlayUserAccessType {
    fn to_string(&self) -> String {
        match self {
            Self::CreateAndJoinGroups => String::from("CreateAndJoinGroups"),
            Self::JoinGroups => String::from("JoinGroups"),
            Self::None => String::from("None"),
        }
    }
}

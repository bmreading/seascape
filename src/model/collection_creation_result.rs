//! Items specific to the result of a collection creation
use serde::{Deserialize, Serialize};

/// Represents the result of a collection creation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CollectionCreationResult {
    /// The newly created collection's unique identifier
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

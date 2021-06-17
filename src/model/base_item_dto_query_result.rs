use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemDtoQueryResult<T> {
    /// Gets or sets the items.
    #[serde(rename = "Items", skip_serializing_if = "Option::is_none")]
    //pub items: Option<Vec<crate::model::BaseItemDto>>,
    pub items: Option<Vec<T>>,
    /// The total number of records available.
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// The index of the first record in Items.
    #[serde(rename = "StartIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}

impl<T> BaseItemDtoQueryResult<T> {
    pub fn new() -> BaseItemDtoQueryResult<T> {
        BaseItemDtoQueryResult {
            items: None,
            total_record_count: None,
            start_index: None,
        }
    }
}

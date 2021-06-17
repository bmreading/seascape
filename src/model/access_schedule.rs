//! Items specific to an Access Schedule
use serde::{Deserialize, Serialize};

/// Represents a user's access schedule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessSchedule {
    /// The id of this instance.
    #[serde(rename = "Id")]
    pub id: i32,
    /// The id of the associated user.
    #[serde(rename = "UserId")]
    pub user_id: String,
    /// The day of week.
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: Box<crate::model::DynamicDayOfWeek>,
    /// The start hour.
    #[serde(rename = "StartHour")]
    pub start_hour: f64,
    /// The end hour.
    #[serde(rename = "EndHour")]
    pub end_hour: f64,
}

impl AccessSchedule {
    /// Returns a new access schedule.
    pub fn new(
        id: i32,
        user_id: String,
        day_of_week: crate::model::DynamicDayOfWeek,
        start_hour: f64,
        end_hour: f64,
    ) -> AccessSchedule {
        AccessSchedule {
            id,
            user_id,
            day_of_week: Box::new(day_of_week),
            start_hour,
            end_hour,
        }
    }
}

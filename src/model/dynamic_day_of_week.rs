use serde::{Deserialize, Serialize};

/// DynamicDayOfWeek : An enum that represents a day of the week, weekdays, weekends, or all days.

/// An enum that represents a day of the week, weekdays, weekends, or all days.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DynamicDayOfWeek {
    #[serde(rename = "Sunday")]
    Sunday,
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
    #[serde(rename = "Everyday")]
    Everyday,
    #[serde(rename = "Weekday")]
    Weekday,
    #[serde(rename = "Weekend")]
    Weekend,
}

impl ToString for DynamicDayOfWeek {
    fn to_string(&self) -> String {
        match self {
            Self::Sunday => String::from("Sunday"),
            Self::Monday => String::from("Monday"),
            Self::Tuesday => String::from("Tuesday"),
            Self::Wednesday => String::from("Wednesday"),
            Self::Thursday => String::from("Thursday"),
            Self::Friday => String::from("Friday"),
            Self::Saturday => String::from("Saturday"),
            Self::Everyday => String::from("Everyday"),
            Self::Weekday => String::from("Weekday"),
            Self::Weekend => String::from("Weekend"),
        }
    }
}

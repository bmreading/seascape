use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DayOfWeek {
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
}

impl ToString for DayOfWeek {
    fn to_string(&self) -> String {
        match self {
            Self::Sunday => String::from("Sunday"),
            Self::Monday => String::from("Monday"),
            Self::Tuesday => String::from("Tuesday"),
            Self::Wednesday => String::from("Wednesday"),
            Self::Thursday => String::from("Thursday"),
            Self::Friday => String::from("Friday"),
            Self::Saturday => String::from("Saturday"),
        }
    }
}

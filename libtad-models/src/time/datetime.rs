use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, Copy, PartialEq, Deserialize)]
#[serde(default)]
/// Date and time, split up into components.
pub struct DateTime {
    /// The year component of the timestamp.
    pub year: i32,

    /// The month component of the timestamp.
    pub month: i32,

    /// The day component of the timestamp.
    pub day: i32,

    /// The hour component of the timestamp.
    pub hour: i32,

    /// The minute component of the timestamp.
    pub minute: i32,

    /// The second component of the timestamp.
    pub second: i32,
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl DateTime {
    pub(crate) fn option_deserialize_from_str<'a, D>(
        deserializer: D,
    ) -> Result<Option<Self>, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(inner) = s {
            Ok(Some(DateTime::from(&inner)))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn deserialize_from_str<'a, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(DateTime::from(&s))
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl<A: AsRef<str>> From<A> for DateTime {
    fn from(s: A) -> Self {
        fn alt(s: &str) -> (&str, &str) {
            if let Some(d) = s.split_once('-') {
                return d;
            }

            if let Some(m) = s.split_once('T') {
                return m;
            }

            if let Some(t) = s.split_once(':') {
                return t;
            }

            (s, "")
        }

        let (year, rest) = alt(s.as_ref());
        let (month, rest) = alt(rest);
        let (day, rest) = alt(rest);
        let (hour, rest) = alt(rest);
        let (minute, rest) = alt(rest);
        let (second, _) = alt(rest);

        Self {
            year: year.parse().unwrap_or_default(),
            month: month.parse().unwrap_or_default(),
            day: day.parse().unwrap_or_default(),
            hour: hour.parse().unwrap_or_default(),
            minute: minute.parse().unwrap_or_default(),
            second: second.parse().unwrap_or_default(),
        }
    }
}

#[test]
fn parse_datetime() {
    let datetime_str = "2021-04-04T16:42:21";

    let datetime = DateTime::from(datetime_str.to_string());

    assert!(datetime.year == 2021);
    assert!(datetime.month == 4);
    assert!(datetime.day == 4);
    assert!(datetime.hour == 16);
    assert!(datetime.minute == 42);
    assert!(datetime.second == 21);
}

#[test]
fn parse_date_only() {
    let datetime_str = "2021-04-04";

    let datetime = DateTime::from(datetime_str.to_string());

    assert!(datetime.year == 2021);
    assert!(datetime.month == 4);
    assert!(datetime.day == 4);
    assert!(datetime.hour == 0);
    assert!(datetime.minute == 0);
    assert!(datetime.second == 0);
}

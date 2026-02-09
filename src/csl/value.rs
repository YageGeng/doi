use std::fmt;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{SeqAccess, Visitor},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CslValue {
    String(String),
    Number(i64),
}

/// year, month, day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClsDate(pub i64, pub Option<i64>, pub Option<i64>);

pub type ClsDatePart = Vec<ClsDate>;

impl Serialize for ClsDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match (self.1, self.2) {
            (None, None) => [self.0].serialize(serializer),
            (None, Some(_)) => [self.0].serialize(serializer),
            (Some(month), None) => [self.0, month].serialize(serializer),
            (Some(month), Some(day)) => [self.0, month, day].serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ClsDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClsDateVisitor;

        impl<'de> Visitor<'de> for ClsDateVisitor {
            type Value = ClsDate;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "an array of 1, 2 or 3 integers: [year, month?, day?]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let year: i64 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                let month: Option<i64> = seq.next_element()?;
                let day: Option<i64> = seq.next_element()?;

                Ok(ClsDate(year, month, day))
            }
        }

        deserializer.deserialize_seq(ClsDateVisitor)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize_cls_date() {
        let date = serde_json::from_str::<ClsDate>("[2019, 12, 1]");
        assert!(date.is_ok());
        let date = serde_json::from_str::<ClsDate>("[2019, 12]");
        assert!(date.is_ok());
        let date = serde_json::from_str::<ClsDate>("[2019]");
        assert!(date.is_ok());
    }

    #[test]
    fn test_serialize_cls_date() {
        let date = ClsDate(2019, Some(12), Some(1));
        let serialized = serde_json::to_string(&date);
        assert!(serialized.is_ok());
        if let Ok(serialized) = serialized {
            assert_eq!(serialized, "[2019,12,1]");
        }

        let date = ClsDate(2019, Some(12), None);
        let serialized = serde_json::to_string(&date);
        assert!(serialized.is_ok());
        if let Ok(serialized) = serialized {
            assert_eq!(serialized, "[2019,12]");
        }

        let date = ClsDate(2019, None, None);
        let serialized = serde_json::to_string(&date);
        assert!(serialized.is_ok());
        if let Ok(serialized) = serialized {
            assert_eq!(serialized, "[2019]");
        }

        let date = ClsDate(2019, None, Some(1));
        let serialized = serde_json::to_string(&date);
        assert!(serialized.is_ok());
        if let Ok(serialized) = serialized {
            assert_eq!(serialized, "[2019]");
        }
    }
}

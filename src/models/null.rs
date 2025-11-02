use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Null;

impl Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

impl<'de> Deserialize<'de> for Null {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        match value {
            serde_json::Value::Null => Ok(Null),
            _ => Err(serde::de::Error::custom("Field must be null")),
        }
    }
}

// impl<T> From<Null> for Option<T> {
//     fn from(_: Null) -> Self {
//         None
//     }
// }

impl<T> TryFrom<Option<T>> for Null {
    type Error = &'static str;

    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        match value {
            Some(_) => Err("Cannot convert Some(T) to Null"),
            None => Ok(Null),
        }
    }
}

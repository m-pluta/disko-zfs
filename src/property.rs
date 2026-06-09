use serde::{Deserialize, Serialize, de::Visitor};

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum PropertySource {
    #[serde(rename(deserialize = "LOCAL"))]
    Local { data: String },
    #[serde(rename(deserialize = "NONE"))]
    None { data: String },
    #[serde(rename(deserialize = "INHERITED"))]
    Inherited { data: String },
    #[serde(rename(deserialize = "DEFAULT"))]
    Default { data: String },
    #[serde(rename(deserialize = "TEMPORARY"))]
    Temporary { data: String },
    #[serde(rename(deserialize = "RECEIVED"))]
    Received { data: String },
}

impl PropertySource {
    pub fn is_local(&self) -> bool {
        match self {
            PropertySource::Local { .. } => true,
            PropertySource::Received { .. }
            | PropertySource::Temporary { .. }
            | PropertySource::None { .. }
            | PropertySource::Inherited { .. }
            | PropertySource::Default { .. } => false,
        }
    }
    pub fn user_managed(&self) -> bool {
        match self {
            PropertySource::Local { .. }
            | PropertySource::Inherited { .. }
            | PropertySource::Default { .. }
            | PropertySource::Received { .. } => true,
            PropertySource::Temporary { .. } | PropertySource::None { .. } => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyValue {
    Number(u64),
    String(String),
}

impl PropertyValue {
    pub fn to_string(&self) -> String {
        match self {
            PropertyValue::Number(num) => num.to_string(),
            PropertyValue::String(string) => string.clone(),
        }
    }
}

struct PropertyValueVisitor;

impl<'de> Visitor<'de> for PropertyValueVisitor {
    type Value = PropertyValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("either a numeric value or a string")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Self::Value::Number(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Self::Value::String(v.to_owned()))
    }
}

impl<'de> Deserialize<'de> for PropertyValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(PropertyValueVisitor)
    }
}

impl Serialize for PropertyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PropertyValue::Number(num) => num.serialize(serializer),
            PropertyValue::String(str) => str.serialize(serializer),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Property {
    pub value: PropertyValue,
    pub source: PropertySource,
}

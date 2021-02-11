use std::fmt;
use derive_more::From;

#[derive(From, Debug, Clone, Default)]
pub struct NewLineDelimited(Vec<String>);

// Serialize

impl serde::Serialize for NewLineDelimited {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        if self.0.len() > 0 {
            let body = self.0.join("\n");
            serializer.serialize_str(&body)
        } else {
            serializer.serialize_none()
        }
    }
}

// Deserialize

struct OptionalNewLineDelimitedVisitor;
struct NewLineDelimitedVisitor;

impl<'de> serde::Deserialize<'de> for NewLineDelimited {
    fn deserialize<D>(deserializer: D) -> Result<NewLineDelimited, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(Self(deserializer.deserialize_option(OptionalNewLineDelimitedVisitor)?))
    }
}

impl<'de> serde::de::Visitor<'de> for OptionalNewLineDelimitedVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null or string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        Ok(vec![])
    }

    fn visit_some<D>(self, value: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(value.deserialize_str(NewLineDelimitedVisitor)?)
    }
}

impl<'de> serde::de::Visitor<'de> for NewLineDelimitedVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        Ok(value.split('\n').map(String::from).collect::<Vec<String>>())
    }
}
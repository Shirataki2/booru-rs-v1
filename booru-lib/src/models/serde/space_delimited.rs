use std::fmt;
use derive_more::From;

#[derive(From, Debug, Clone, Default)]
pub struct SpaceDelimited(Vec<String>);

// Serialize

impl serde::Serialize for SpaceDelimited {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        if self.0.len() > 0 {
            let body = self.0.join(" ");
            serializer.serialize_str(&body)
        } else {
            serializer.serialize_none()
        }
    }
}
// Deserialize

struct OptionalSpaceDelimitedVisitor;
struct SpaceDelimitedVisitor;

impl<'de> serde::Deserialize<'de> for SpaceDelimited {
    fn deserialize<D>(deserializer: D) -> Result<SpaceDelimited, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(Self(deserializer.deserialize_option(OptionalSpaceDelimitedVisitor)?))
    }
}

impl<'de> serde::de::Visitor<'de> for OptionalSpaceDelimitedVisitor {
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
        Ok(value.deserialize_str(SpaceDelimitedVisitor)?)
    }
}

impl<'de> serde::de::Visitor<'de> for SpaceDelimitedVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        if value == "" {
            Ok(vec![])
        } else {
            Ok(value.split(' ').map(String::from).collect::<Vec<String>>())
        }
    }
}
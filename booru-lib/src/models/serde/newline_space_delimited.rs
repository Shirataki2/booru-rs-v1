use std::fmt;
use regex::Regex;
use derive_more::From;

#[derive(From, Debug, Clone, Default)]
pub struct NewLineSpaceDelimited(Vec<String>);

// Serialize

impl serde::Serialize for NewLineSpaceDelimited {
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

struct OptionalNewLineSpaceDelimitedVisitor;
struct NewLineSpaceDelimitedVisitor;

impl<'de> serde::Deserialize<'de> for NewLineSpaceDelimited {
    fn deserialize<D>(deserializer: D) -> Result<NewLineSpaceDelimited, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        Ok(Self(deserializer.deserialize_option(OptionalNewLineSpaceDelimitedVisitor)?))
    }
}

impl<'de> serde::de::Visitor<'de> for OptionalNewLineSpaceDelimitedVisitor {
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
        Ok(value.deserialize_str(NewLineSpaceDelimitedVisitor)?)
    }
}

impl<'de> serde::de::Visitor<'de> for NewLineSpaceDelimitedVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        let re = Regex::new(r"\n|\s").unwrap();
        Ok(re.split(value).map(String::from).collect::<Vec<String>>())
    }
}

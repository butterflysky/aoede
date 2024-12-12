use serde::{de::Visitor, Deserialize};
use std::fmt;

#[derive(Clone)]
pub struct SpotifyUsername(pub String);

// Custom visitor to handle both string and u64
struct SpotifyUsernameVisitor;

impl<'de> Visitor<'de> for SpotifyUsernameVisitor {
    type Value = SpotifyUsername;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or u64")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SpotifyUsername(value.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SpotifyUsername(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SpotifyUsername(value.to_string()))
    }
}

impl<'de> Deserialize<'de> for SpotifyUsername {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(SpotifyUsernameVisitor)
    }
}

impl From<SpotifyUsername> for String {
    fn from(username: SpotifyUsername) -> Self {
        username.0
    }
}

impl AsRef<str> for SpotifyUsername {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

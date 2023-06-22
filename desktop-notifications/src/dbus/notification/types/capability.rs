use std::str::FromStr;

use strum_macros::EnumString;
use zbus::{
    export::serde::Deserialize,
    zvariant::{Signature, Type},
};

#[derive(Debug, EnumString, Clone)]
#[strum(serialize_all = "kebab-case")]
pub enum Capability {
    ActionIcons,
    Actions,
    Body,
    BodyHyperlinks,
    BodyImages,
    BodyMarkup,
    IconMulti,
    IconStatic,
    Persistence,
    Sound,
    Unknown(String),
}

impl Type for Capability {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

impl<'de> Deserialize<'de> for Capability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: zbus::export::serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .map(|value| Capability::from_str(&value).unwrap_or(Capability::Unknown(value)))
    }
}

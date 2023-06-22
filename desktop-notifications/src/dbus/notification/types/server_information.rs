use derive_more::{Constructor, From};
use zbus::{export::serde::Deserialize, zvariant::Type};

#[derive(Debug, Clone, From, Constructor, Type)]
pub struct ServerInformation {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub spec_version: String,
}

impl<'de> Deserialize<'de> for ServerInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: zbus::export::serde::Deserializer<'de>,
    {
        <(String, String, String, String)>::deserialize(deserializer).map(
            |(name, vendor, version, spec_version)| {
                ServerInformation::new(name, vendor, version, spec_version)
            },
        )
    }
}

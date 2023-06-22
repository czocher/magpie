use derive_more::{Constructor, Deref, From, Into};
use strum_macros::FromRepr;
use zbus::{
    export::serde::{Deserialize, Deserializer, Serialize},
    zvariant::{OwnedObjectPath, Signature, Type},
};

#[derive(Debug, Into, Constructor, Deref)]
pub struct JobPath(pub OwnedObjectPath);

impl Type for JobPath {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked("o")
    }
}

impl<'de> Deserialize<'de> for JobPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        OwnedObjectPath::deserialize(deserializer).map(JobPath::new)
    }
}

#[derive(Debug, FromRepr, Default, Copy, Clone)]
#[repr(i32)]
pub enum Capability {
    #[default]
    None = 0,
    Killable = 1,
    Suspendable = 2,
}

#[derive(Debug, Deref, Default, From)]
#[from(forward)]
pub struct Capabilities(Vec<Capability>);

impl From<&Capabilities> for i32 {
    fn from(val: &Capabilities) -> Self {
        val.0.iter().fold(0, |acc, value| acc | (*value as i32))
    }
}

impl Serialize for Capabilities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        serializer.serialize_i32(self.into())
    }
}

impl Type for Capabilities {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked("i")
    }
}

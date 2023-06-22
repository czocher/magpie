use zbus::{
    export::serde::{Deserialize, Serialize},
    zvariant::{Signature, Type},
};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Default)]
pub enum NotificationId {
    #[default]
    None,
    Id(u32),
}

impl From<u32> for NotificationId {
    fn from(value: u32) -> Self {
        if value == 0 {
            NotificationId::None
        } else {
            NotificationId::Id(value)
        }
    }
}

impl From<&NotificationId> for u32 {
    fn from(val: &NotificationId) -> Self {
        match val {
            NotificationId::None => 0,
            NotificationId::Id(id) => *id,
        }
    }
}

impl Serialize for NotificationId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        serializer.serialize_u32(self.into())
    }
}

impl Type for NotificationId {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked("u")
    }
}

impl<'de> Deserialize<'de> for NotificationId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: zbus::export::serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(NotificationId::from)
    }
}

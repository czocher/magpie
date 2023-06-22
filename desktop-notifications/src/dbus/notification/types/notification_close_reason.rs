use strum_macros::{Display, FromRepr};
use zbus::{
    export::serde::Deserialize,
    zvariant::{Signature, Type},
};

#[derive(Display, Debug, Clone, FromRepr, Default)]
#[repr(u32)]
pub enum NotificationCloseReason {
    Expired = 1,
    Dismissed = 2,
    Closed = 3,
    #[default]
    Undefined = 4,
}

impl Type for NotificationCloseReason {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked("u")
    }
}

impl<'de> Deserialize<'de> for NotificationCloseReason {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: zbus::export::serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer)
            .map(|value| NotificationCloseReason::from_repr(value).unwrap_or_default())
    }
}

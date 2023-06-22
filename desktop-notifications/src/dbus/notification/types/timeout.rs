use std::time::Duration;

use strum_macros::{Display, FromRepr};
use zbus::{
    export::serde::Serialize,
    zvariant::{Signature, Type},
};

#[derive(Display, Debug, Clone, FromRepr, Default)]
pub enum Timeout {
    #[default]
    Never,
    Duration(Duration),
}

impl Serialize for Timeout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        let miliseconds = match self {
            Timeout::Never => 0i32,
            Timeout::Duration(duration) => duration.as_millis() as i32,
        };

        serializer.serialize_i32(miliseconds)
    }
}

impl Type for Timeout {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked("i")
    }
}

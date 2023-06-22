use strum_macros::Display;
use zbus::{
    export::serde::Serialize,
    zvariant::{Signature, Type},
};

#[derive(Display, Debug)]
pub enum Unit {
    #[strum(serialize = "bytes")]
    Bytes,
    #[strum(serialize = "files")]
    Files,
    #[strum(serialize = "dirs")]
    Dirs,
    #[strum(serialize = "items")]
    Items,
    #[strum(serialize = "units_count")]
    UnitsCount,
}

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl Type for Unit {
    fn signature() -> zbus::zvariant::Signature<'static> {
        Signature::from_string_unchecked("s".to_string())
    }
}

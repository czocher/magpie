use strum_macros::Display;
use url::Url;
use zbus::{
    export::serde::Serialize,
    zvariant::{Signature, Type},
};

#[derive(Debug, Clone, Display)]
pub enum ImagePath {
    Uri(Url),
    Name(String),
}

impl Default for ImagePath {
    fn default() -> Self {
        Self::Name("".to_string())
    }
}

impl Serialize for ImagePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        let value = match self {
            ImagePath::Uri(value) => value.to_string(),
            ImagePath::Name(value) => value.to_string(),
        };
        serializer.serialize_str(value.as_str())
    }
}

impl Type for ImagePath {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked("s")
    }
}

use derive_more::Deref;
use strum_macros::Display;

use zbus::{
    export::serde::Serialize,
    zvariant::{OwnedValue, Signature, Type, Value},
};

use super::{
    category::Category, image::Image, image_path::ImagePath, sound_context::SoundContext,
    urgency::Urgency,
};

#[derive(Debug, Display, Clone)]
#[strum(serialize_all = "kebab-case")]
pub enum Hint {
    ActionIcons(bool),
    Category(Category),
    DesktopEntry(String),
    ImageData(Image),
    ImagePath(ImagePath),
    Resident(bool),
    SoundFile(String),
    SoundName(SoundContext),
    SuppressSound(bool),
    Transient(bool),
    X(i32),
    Y(i32),
    Urgency(Urgency),
    Other(String, OwnedValue),
}

#[derive(Default, Deref)]
pub struct Hints(Vec<Hint>);

impl Serialize for Hints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        serializer.collect_map(self.0.iter().map(|hint| {
            (
                match hint {
                    Hint::Other(name, _) => name.to_owned(),
                    h => h.to_string(),
                },
                match hint {
                    Hint::ActionIcons(value) => Value::from(value),
                    Hint::Category(value) => Value::from(value.to_string()),
                    Hint::DesktopEntry(value) => Value::from(value.to_string()),
                    Hint::ImageData(value) => Value::from(value),
                    Hint::ImagePath(value) => Value::from(value.to_string()),
                    Hint::Resident(value) => Value::from(value),
                    Hint::SoundFile(value) => Value::from(value),
                    Hint::SoundName(value) => Value::from(value.to_string()),
                    Hint::SuppressSound(value) => Value::from(value),
                    Hint::Transient(value) => Value::from(value),
                    Hint::X(value) => Value::from(value),
                    Hint::Y(value) => Value::from(value),
                    Hint::Urgency(value) => Value::from(value.clone() as u32),
                    Hint::Other(_name, value) => value.into(),
                },
            )
        }))
    }
}

impl Type for Hints {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked("a{sv}")
    }
}

impl From<Vec<Hint>> for Hints {
    fn from(value: Vec<Hint>) -> Self {
        Self(value)
    }
}

use strum_macros::Display;
use strum_macros::FromRepr;

use zbus::zvariant::OwnedValue;

#[derive(Display, Debug, FromRepr, Default)]
#[repr(u32)]
pub enum Metered {
    #[default]
    Unknown = 0,
    Yes = 1,
    No = 2,
    GuessYes = 3,
    GuessNo = 4,
}

impl From<OwnedValue> for Metered {
    fn from(value: OwnedValue) -> Self {
        let value: Result<u32, _> = value.try_into();
        value
            .iter()
            .flat_map(|v| Metered::from_repr(*v))
            .next()
            .unwrap_or_default()
    }
}

#[derive(Display, Debug, FromRepr, Default)]
pub enum Connectivity {
    #[default]
    Unknown = 0,
    None = 1,
    Portal = 2,
    Limited = 3,
    Full = 4,
}

impl From<OwnedValue> for Connectivity {
    fn from(value: OwnedValue) -> Self {
        let value: Result<u32, _> = value.try_into();
        value
            .map(|v| v as usize)
            .iter()
            .flat_map(|v| Connectivity::from_repr(*v))
            .next()
            .unwrap_or_default()
    }
}

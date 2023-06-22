use strum_macros::{Display, FromRepr};

#[derive(Display, Debug, Clone, FromRepr, Default)]
#[repr(u32)]
pub enum Urgency {
    Low = 0,
    #[default]
    Normal = 1,
    Critical = 2,
}

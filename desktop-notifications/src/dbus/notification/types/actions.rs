use derive_more::{Constructor, Deref};
use zbus::{
    export::serde::Serialize,
    zvariant::{Signature, Type},
};

pub trait Action {
    fn key(&self) -> String;
    fn value(&self) -> String;
}

impl Action for (&str, &str) {
    fn key(&self) -> String {
        self.0.to_string()
    }

    fn value(&self) -> String {
        self.1.to_string()
    }
}

impl Action for &str {
    fn key(&self) -> String {
        self.to_string()
    }

    fn value(&self) -> String {
        self.to_string()
    }
}

#[derive(Constructor, Deref)]
pub struct Actions(Vec<String>);

impl<T: Action> From<Vec<T>> for Actions {
    fn from(value: Vec<T>) -> Self {
        let values = value
            .iter()
            .flat_map(|action| vec![action.key(), action.value()])
            .collect();
        Self::new(values)
    }
}

impl<I: Action> FromIterator<I> for Actions {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let values = iter
            .into_iter()
            .flat_map(|action| vec![action.key(), action.value()])
            .collect();
        Self::new(values)
    }
}

impl Serialize for Actions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        serializer.collect_seq(self.0.iter())
    }
}

impl Type for Actions {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked("as")
    }
}

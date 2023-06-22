use derive_more::Constructor;
use zbus::{
    export::serde::{ser::SerializeStruct, Serialize},
    zvariant::{Structure, StructureBuilder, Type},
};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Constructor, Type)]
pub struct Image {
    width: i32,
    height: i32,
    rowstride: i32,
    alpha: bool,
    bits_per_sample: i32,
    channels: i32,
    data: Vec<u8>,
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: zbus::export::serde::Serializer,
    {
        let mut image = serializer.serialize_struct("Image", 7)?;
        image.serialize_field("width", &self.width)?;
        image.serialize_field("height", &self.height)?;
        image.serialize_field("rowstride", &self.rowstride)?;
        image.serialize_field("alpha", &self.alpha)?;
        image.serialize_field("bits_per_sample", &self.bits_per_sample)?;
        image.serialize_field("channels", &self.channels)?;
        image.serialize_field("data", &self.data)?;
        image.end()
    }
}

impl<'a> Into<Structure<'a>> for &Image {
    fn into(self) -> Structure<'a> {
        StructureBuilder::new()
            .add_field(self.width)
            .add_field(self.height)
            .add_field(self.rowstride)
            .add_field(self.alpha)
            .add_field(self.bits_per_sample)
            .add_field(self.channels)
            .add_field(self.data.clone())
            .build()
    }
}

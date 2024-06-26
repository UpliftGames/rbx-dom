use std::io::{Read, Write};

use rbx_dom_weak::types::{UniqueId, UniqueIdError};

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, DecodeErrorKind, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for UniqueId {
    const XML_TAG_NAME: &'static str = "UniqueId";

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        let content = reader.read_characters()?;

        content.parse().map_err(|e| reader.error(e))
    }

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_value(&self.to_string())
    }
}

impl From<UniqueIdError> for DecodeErrorKind {
    fn from(value: UniqueIdError) -> Self {
        DecodeErrorKind::InvalidPropertyData {
            property_type: "UniqueId",
            error: value.to_string(),
        }
    }
}

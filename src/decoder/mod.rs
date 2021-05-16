mod decoder;
mod error;

use decoder::Decoder;
pub use error::DecodeError;

use crate::module::{Module, SectionType};
use std::io::Cursor;

/// wasmバイナリをデコードしてwasm moduleを返す
pub fn decode(buf: &[u8]) -> Result<Module, DecodeError> {
    let mut decoder = Decoder::new(Cursor::new(buf));

    let mut m = Module::default();

    decoder.validate_wasm_format()?;
    m.version = decoder.decode_version()?;

    loop {
        if decoder.is_end() {
            break;
        }

        let (section_type, section_size) = decoder.decode_section_type()?;

        let section = match section_type {
            SectionType::Custom => decoder.decode_custom_section(section_size)?,
            SectionType::Type => decoder.decode_type_section(section_size)?,
            SectionType::Import => decoder.decode_import_section(section_size)?,
            SectionType::Function => decoder.decode_function_section(section_size)?,
            SectionType::Table => decoder.decode_table_section(section_size)?,
            SectionType::Memory => decoder.decode_memory_section(section_size)?,
            SectionType::Global => decoder.decode_global_section(section_size)?,
            SectionType::Export => decoder.decode_export_section(section_size)?,
            SectionType::Start => decoder.decode_start_section(section_size)?,
            SectionType::Element => decoder.decode_element_section(section_size)?,
            SectionType::Code => decoder.decode_code_section(section_size)?,
            SectionType::Data => decoder.decode_data_section(section_size)?,
            SectionType::Unsuport => todo!(),
        };

        m.take_in(section);
    }

    Ok(m)
}

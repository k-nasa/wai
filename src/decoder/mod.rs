mod decoder;
mod error;

mod type_section;

use decoder::Decoder;
pub use error::DecodeError;

use crate::module::Module;
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
        let section = decoder.decode_section(section_type, section_size)?;

        m.take_in(section);
    }

    Ok(m)
}

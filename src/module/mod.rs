use crate::decoder;

pub struct Module {}

impl Module {
    pub fn from_byte(byte: impl AsRef<[u8]>) -> Result<Self, decoder::DecodeError> {
        decoder::decode(byte.as_ref())
    }
}

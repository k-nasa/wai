use crate::decoder;

#[derive(Debug)]
pub struct Module {
    pub(crate) version: u32,
    pub(crate) custom_section: Option<()>,
    pub(crate) type_section: Option<()>,
    pub(crate) import_section: Option<()>,
    pub(crate) function_section: Option<()>,
    pub(crate) table_section: Option<()>,
    pub(crate) memory_section: Option<()>,
    pub(crate) global_section: Option<()>,
    pub(crate) export_section: Option<()>,
    pub(crate) element_section: Option<()>,
    pub(crate) start_section: Option<()>,
    pub(crate) code_section: Option<()>,
    pub(crate) data_section: Option<()>,
}

impl Module {
    pub fn from_byte(byte: impl AsRef<[u8]>) -> Result<Self, decoder::DecodeError> {
        decoder::decode(byte.as_ref())
    }
}

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

impl Default for Module {
    fn default() -> Self {
        Module {
            version: 0,
            custom_section: None,
            type_section: None,
            import_section: None,
            function_section: None,
            table_section: None,
            memory_section: None,
            global_section: None,
            export_section: None,
            element_section: None,
            start_section: None,
            code_section: None,
            data_section: None,
        }
    }
}

impl Module {
    pub fn from_byte(byte: impl AsRef<[u8]>) -> Result<Self, decoder::DecodeError> {
        decoder::decode(byte.as_ref())
    }

    // TODO refactor, section_typeとsectionの両方を取る必要はない
    pub(crate) fn take_in(&mut self, section_type: SectionType, section: ()) {
        use SectionType::*;

        match section_type {
            Custom => self.custom_section = Some(section),
            Type => self.type_section = Some(section),
            Import => self.import_section = Some(section),
            Function => self.function_section = Some(section),
            Table => self.table_section = Some(section),
            Memory => self.memory_section = Some(section),
            Global => self.global_section = Some(section),
            Export => self.export_section = Some(section),
            Start => self.start_section = Some(section),
            Element => self.element_section = Some(section),
            Code => self.code_section = Some(section),
            Data => self.data_section = Some(section),
            Unsuport => (),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SectionType {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    Unsuport,
}

impl From<u8> for SectionType {
    fn from(x: u8) -> Self {
        use self::SectionType::*;
        match x {
            0x0 => Custom,
            0x1 => Type,
            0x2 => Import,
            0x3 => Function,
            0x4 => Table,
            0x5 => Memory,
            0x6 => Global,
            0x7 => Export,
            0x8 => Start,
            0x9 => Element,
            0xA => Code,
            0xB => Data,
            _ => Unsuport,
        }
    }
}

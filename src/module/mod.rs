use crate::decode;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) version: u32,
    pub(crate) custom_section: Option<()>,
    pub(crate) type_section: Option<TypeSection>,
    pub(crate) import_section: Option<()>,
    pub(crate) function_section: Option<FunctionSection>,
    pub(crate) table_section: Option<()>,
    pub(crate) memory_section: Option<()>,
    pub(crate) global_section: Option<()>,
    pub(crate) export_section: Option<ExportSection>,
    pub(crate) element_section: Option<()>,
    pub(crate) start_section: Option<()>,
    pub(crate) code_section: Option<CodeSection>,
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
    pub fn from_byte(byte: impl AsRef<[u8]>) -> Result<Self, decode::DecodeError> {
        decode::decode(byte.as_ref())
    }

    // TODO refactor, section_typeとsectionの両方を取る必要はない
    pub(crate) fn take_in(&mut self, section: Section) {
        use Section::*;

        match section {
            Custom(i) => self.custom_section = Some(i),
            Type(i) => self.type_section = Some(i),
            Import(i) => self.import_section = Some(i),
            Function(i) => self.function_section = Some(i),
            Table(i) => self.table_section = Some(i),
            Memory(i) => self.memory_section = Some(i),
            Global(i) => self.global_section = Some(i),
            Export(i) => self.export_section = Some(i),
            Start(i) => self.start_section = Some(i),
            Element(i) => self.element_section = Some(i),
            Code(i) => self.code_section = Some(i),
            Data(i) => self.data_section = Some(i),
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

#[derive(Clone, Debug)]
pub enum Section {
    Custom(()),
    Type(TypeSection),
    Import(()),
    Function(FunctionSection),
    Table(()),
    Memory(()),
    Global(()),
    Export(ExportSection),
    Start(()),
    Element(()),
    Code(CodeSection),
    Data(()),
}

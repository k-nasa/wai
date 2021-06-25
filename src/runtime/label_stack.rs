use crate::types::*;

pub type LabelStack = Vec<Label>;

pub struct Label {
    pc: usize,
    label_type: LabelType,
    result_type: BlockType,
}

impl Label {
    pub fn new(pc: usize, label_type: LabelType, result_type: BlockType) -> Self {
        Self {
            pc,
            label_type,
            result_type,
        }
    }

    pub fn new_block(pc: usize, result_type: BlockType) -> Self {
        Self::new(pc, LabelType::Block, result_type)
    }
}

pub enum LabelType {
    Block,
    Loop,
    If,
}

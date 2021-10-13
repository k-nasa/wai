use alloc::vec::Vec;

use crate::types::*;

pub type LabelStack = Vec<Label>;

#[derive(Debug)]
pub struct Label {
    pub pc: usize,
    pub label_type: LabelType,
    pub result_type: BlockType,
}

impl Label {
    pub fn new(pc: usize, label_type: LabelType, result_type: BlockType) -> Self {
        Self {
            pc,
            label_type,
            result_type,
        }
    }

    pub fn new_if(pc: usize, result_type: BlockType, condition: bool) -> Self {
        Self::new(pc, LabelType::If(condition), result_type)
    }
}

#[derive(Debug)]
pub enum LabelType {
    Block,
    Loop,
    If(bool),
}

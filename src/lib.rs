#![no_std]
#![feature(linked_list_cursors)]
#![feature(alloc_prelude)]

#[macro_use]
extern crate alloc;

use alloc::prelude::*;

mod decode;
mod from_le;
mod instance;
mod instruction;
mod module;
mod opcode;
mod runtime;
mod types;

pub use runtime::RuntimeValue;
pub use {instance::Instance, module::Module};

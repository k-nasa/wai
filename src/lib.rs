mod decode;
mod from_le;
mod instance;
mod instruction;
mod module;
mod opcode;
mod runtime;
mod types;

pub use runtime::{RuntimeError, RuntimeValue};
pub use {instance::Instance, module::Module};

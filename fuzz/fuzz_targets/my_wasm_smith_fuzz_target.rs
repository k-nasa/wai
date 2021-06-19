#![no_main]

use wasmi::*;

use libfuzzer_sys::fuzz_target;
use wasm_smith::Module as M;

fuzz_target!(|module: M| {
    let bytes = module.to_bytes();

    let m = Module::from_byte(bytes).unwrap();
    // dbg!(m);
});

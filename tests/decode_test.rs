use std::io::Read;
use wasmi::*;
use wast::WastDirective;

macro_rules! wasm_test {
    ($func:ident, $path:expr) => {
        #[test]
        fn $func() -> anyhow::Result<()> {
            assert_wasm($path)
        }
    };
}

wasm_test!(add, "wat/add.wat");
wasm_test!(address, "testsuite/address.wast");
wasm_test!(align, "testsuite/align.wast");
wasm_test!(binary, "testsuite/binary.wast");
wasm_test!(comments, "testsuite/comments.wast");
wasm_test!(consts, "testsuite/const.wast");
wasm_test!(custom, "testsuite/custom.wast");
wasm_test!(data, "testsuite/data.wast");
wasm_test!(elem, "testsuite/elem.wast");
wasm_test!(endianness, "testsuite/endianness.wast");
wasm_test!(_f32, "./testsuite/f32.wast");
wasm_test!(_i32, "testsuite/i32.wast");
wasm_test!(memory, "testsuite/memory.wast");
wasm_test!(_type, "testsuite/type.wast");

fn assert_wasm(filepath: &str) -> anyhow::Result<()> {
    let mut buf = vec![];
    let mut file = std::fs::File::open(filepath)?;
    file.read_to_end(&mut buf)?;
    let wast = String::from_utf8(buf).unwrap();

    let buf = wast::parser::ParseBuffer::new(&wast)?;
    let wast = wast::parser::parse::<wast::Wast>(&buf)?;

    for directive in wast.directives {
        match directive {
            WastDirective::Module(mut module) => {
                let module_binary = module.encode()?;
                assert!(Module::from_byte(module_binary).is_ok());
            }
            _ => {}
        }
    }

    Ok(())
}

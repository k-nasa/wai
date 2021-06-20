use std::io::Read;
use wabt::script::{Action, Command, CommandKind, ScriptParser, Value};
use wasmi::*;

macro_rules! wasm_test {
    ($func:ident, $path:expr) => {
        #[test]
        fn $func() -> anyhow::Result<()> {
            assert_wasm($path)
        }
    };
}

wasm_test!(add, "./wat/add.wat");
// wasm_test!(address, "./testsuite/address.wast");
// wasm_test!(align, "./testsuite/align.wast");
wasm_test!(binary, "./testsuite/binary.wast");
wasm_test!(comments, "./testsuite/comments.wast");
wasm_test!(consts, "./testsuite/const.wast");
wasm_test!(custom, "./testsuite/custom.wast");
wasm_test!(data, "./testsuite/data.wast");
// wasm_test!(elem, "./testsuite/elem.wast");
// wasm_test!(endianness, "./testsuite/endianness.wast");
// wasm_test!(f32, "./testsuite/f32.wast");
// wasm_test!(ii32, "./testsuite/i32.wast");
// wasm_test!(memory, "./testsuite/memory.wast");
wasm_test!(_type, "./testsuite/type.wast");

fn assert_wasm(filepath: &str) -> anyhow::Result<()> {
    let mut buf = vec![];
    let mut file = std::fs::File::open(filepath)?;
    file.read_to_end(&mut buf)?;
    let wast = String::from_utf8(buf).unwrap();

    let mut parser = ScriptParser::from_str(&wast).unwrap();

    let mut m = Module::default();
    while let Some(Command { kind, .. }) = parser.next()? {
        match kind {
            CommandKind::Module { module, .. } => {
                let module_binary = module.into_vec();
                m = Module::from_byte(module_binary)?;
            }
            CommandKind::AssertReturn { action, expected } => {
                let (invoke, args) = match action {
                    Action::Invoke { field, args, .. } => (field, args),
                    _ => unreachable!(),
                };

                let args = value_to_runtime_value(args);
                let instance = Instance::new(m.clone());
                let actual = instance.invoke(&invoke, args.clone())?;
                let expected = value_to_runtime_value(expected);

                assert_eq!(
                    expected, actual,
                    "\n=====failed assert {}=====\nargs:{:#?}\nexpect {:#?}, return value {:#?}",
                    invoke, args, expected, actual
                );
            }
            _ => {}
        }
    }

    Ok(())
}

fn value_to_runtime_value(args: Vec<Value>) -> Vec<RuntimeValue> {
    let values = args
        .iter()
        .map(|arg| match arg {
            Value::I32(v) => RuntimeValue::I32(*v),
            Value::I64(v) => RuntimeValue::I64(*v),
            Value::F32(v) => RuntimeValue::F32(*v),
            Value::F64(v) => RuntimeValue::F64(*v),
            Value::V128(v) => RuntimeValue::V128(*v),
        })
        .collect();

    values
}

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

wasm_test!(add, "./wasm/wat/add.wat");

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

                dbg!(format!("run {}", &invoke));
                let args = args_to_runtime_value(args);
                let instance = Instance::new(m.clone());
                let return_values = instance.invoke(&invoke, args.clone())?;
                let actual = return_value_to_wabt_values(return_values);

                assert_eq!(
                    expected, actual,
                    "failed assert {}\nargs:{:#?}\nexpect {:#?}, return value {:#?}",
                    invoke, args, expected, actual
                );
            }
            _ => {}
        }
    }

    Ok(())
}

fn args_to_runtime_value(args: Vec<Value>) -> Vec<RuntimeValue> {
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

fn return_value_to_wabt_values(runtime_values: Vec<RuntimeValue>) -> Vec<Value> {
    let values = runtime_values
        .iter()
        .map(|runtime_value| match runtime_value {
            RuntimeValue::I32(v) => Value::I32(*v),
            RuntimeValue::I64(v) => Value::I64(*v),
            RuntimeValue::F32(v) => Value::F32(*v),
            RuntimeValue::F64(v) => Value::F64(*v),
            RuntimeValue::V128(v) => Value::V128(*v),
        })
        .collect();

    values
}

use std::io::Read;
use wai::*;
use wast::WastDirective;

macro_rules! wasm_test {
    ($func:ident, $path:expr) => {
        #[test]
        fn $func() -> anyhow::Result<()> {
            assert_wasm($path)
        }
    };
}

wasm_test!(add, "./wat/add.wat");
wasm_test!(fib, "./wat/fib.wat");

wasm_test!(address, "./testsuite/address.wast");
wasm_test!(binary, "./testsuite/binary.wast");
wasm_test!(comments, "./testsuite/comments.wast");
wasm_test!(consts, "./testsuite/const.wast");
wasm_test!(custom, "./testsuite/custom.wast");
wasm_test!(data, "./testsuite/data.wast");
wasm_test!(_type, "./testsuite/type.wast");
wasm_test!(call, "./testsuite/call.wast");
// wasm_test!(select, "./testsuite/select.wast");
// wasm_test!(_if, "./testsuite/if.wast");
// wasm_test!(block, "./testsuite/block.wast");
// wasm_test!(call, "./testsuite/call.wast");

fn assert_wasm(filepath: &str) -> anyhow::Result<()> {
    let mut buf = vec![];
    let mut file = std::fs::File::open(filepath)?;
    file.read_to_end(&mut buf)?;
    let wast = String::from_utf8(buf).unwrap();

    let buf = wast::parser::ParseBuffer::new(&wast)?;
    let wast = wast::parser::parse::<wast::Wast>(&buf)?;

    let mut m = Module::default();
    for directive in wast.directives {
        match directive {
            WastDirective::Module(mut module) => {
                let module_binary = module.encode()?;
                m = Module::from_byte(module_binary)?;
                dbg!(&m);
            }
            WastDirective::AssertReturn { exec, results, .. } => {
                let (name, args) = match exec {
                    wast::WastExecute::Invoke(invoke) => (invoke.name, invoke.args),
                    _ => unreachable!(),
                };

                let args: Vec<RuntimeValue> = args.iter().map(args_to_runtime_value).collect();
                let instance = Instance::new(m.clone());
                let actual = match instance.invoke(&name, args.clone()) {
                    Ok(v) => v,
                    Err(e) => panic!("\n====== failed assert {}==========\nerror: {}, ", name, e),
                };

                let expected: Vec<RuntimeValue> =
                    results.iter().map(result_to_runtime_value).collect();
                let actual = actual
                    .iter()
                    .map(to_zero_nan)
                    .collect::<Vec<RuntimeValue>>();

                let expected = expected
                    .iter()
                    .map(to_zero_nan)
                    .collect::<Vec<RuntimeValue>>();

                assert_eq!(
                    expected, actual,
                    "\n=====failed assert {}=====\nargs:{:#?}\nexpect {:#?}, return value {:#?}",
                    name, args, expected, actual
                );
            }
            _ => {}
        }
    }

    Ok(())
}

fn to_zero_nan(v: &RuntimeValue) -> RuntimeValue {
    match v {
        RuntimeValue::F32(v) if v.is_nan() => RuntimeValue::F32(0.0),
        RuntimeValue::F64(v) if v.is_nan() => RuntimeValue::F64(0.0),
        v => *v,
    }
}

fn args_to_runtime_value(expr: &wast::Expression) -> RuntimeValue {
    match &expr.instrs[0] {
        wast::Instruction::I32Const(x) => RuntimeValue::I32(*x),
        wast::Instruction::I64Const(x) => RuntimeValue::I64(*x),
        wast::Instruction::F32Const(x) => RuntimeValue::F32(f32::from_bits(x.bits)),
        wast::Instruction::F64Const(x) => RuntimeValue::F64(f64::from_bits(x.bits)),
        _ => unreachable!(),
    }
}

fn result_to_runtime_value(expr: &wast::AssertExpression) -> RuntimeValue {
    match expr {
        wast::AssertExpression::I32(x) => RuntimeValue::I32(*x),
        wast::AssertExpression::I64(x) => RuntimeValue::I64(*x),
        wast::AssertExpression::F32(x) => RuntimeValue::F32(to_f32(x)),
        wast::AssertExpression::F64(x) => RuntimeValue::F64(to_f64(x)),
        _ => unreachable!(),
    }
}

fn to_f64(expr: &wast::NanPattern<wast::Float64>) -> f64 {
    match expr {
        &wast::NanPattern::CanonicalNan => 0.0,
        &wast::NanPattern::ArithmeticNan => 0.0,
        wast::NanPattern::Value(f) => f64::from_bits(f.bits),
    }
}

fn to_f32(expr: &wast::NanPattern<wast::Float32>) -> f32 {
    match expr {
        &wast::NanPattern::CanonicalNan => 0.0,
        &wast::NanPattern::ArithmeticNan => 0.0,
        wast::NanPattern::Value(f) => f32::from_bits(f.bits),
    }
}

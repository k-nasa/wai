use wasmi::*;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Plese speficy filepath")
    }

    let filename = &args[1];
    let bytes = std::fs::read(filename)?;

    let m = Module::from_byte(bytes)?;

    let instance = Instance::new(m);
    let values = instance.invoke("add", vec![RuntimeValue::I32(1), RuntimeValue::I32(1)])?;
    println!("return value is {:?}", values);

    Ok(())
}

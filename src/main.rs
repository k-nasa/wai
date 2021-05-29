use wasmi::*;

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Plese speficy filepath")
    }

    let filename = &args[1];
    let bytes = std::fs::read(filename)?;

    let m = Module::from_byte(bytes)?;
    log::debug!("module: {:#?}", m);

    let instance = Instance::new(m);
    let values = instance.invoke("add", vec![RuntimeValue::I32(1), RuntimeValue::I32(1)])?;
    log::info!("return value is {:?}", values);

    Ok(())
}

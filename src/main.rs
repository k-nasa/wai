use wasmi::*;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() {
        anyhow::bail!("Plese speficy filepath")
    }

    let filename = &args[1];
    let bytes = std::fs::read(filename)?;

    let m = Module::from_byte(bytes)?;
    dbg!(m);
    // let instance = Instance::new(m);
    // instance.invoke("hoge")?;

    Ok(())
}

use wasm_interpreter::*;

fn main() -> anyhow::Result<()> {
    let m = Module::from_byte(b"hoehoge")?;

    let instance = Instance::new(m);
    instance.invoke("hoge")?;

    Ok(())
}

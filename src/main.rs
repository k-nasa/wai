use wasm_interpreter::*;

fn main() -> std::io::Result<()> {
    let m = Module::from_byte(b"hoehoge");

    let instance = Instance::new(m);
    instance.invoke("hoge")?;

    Ok(())
}

use clap::Parser;
use wai::*;

#[derive(Parser)]
#[clap(
    version = "0.2.0",
    author = "k-nasa <htilcs1115@gmail.com>",
    about = "A simple wasm interpreter"
)]
struct Opts {
    file_path: String,

    #[clap(short, long)]
    invoke: String,

    #[clap(short, long)]
    args: Vec<RuntimeValue>,
}

fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let opts: Opts = Opts::parse();

    let filename = opts.file_path;
    let bytes = std::fs::read(filename)?;

    log::info!("start exec {:?}, args {:?}", opts.invoke, opts.args);

    let result = run_wasm(bytes, opts.invoke, opts.args)?;
    log::info!("return value is {:?}", result);

    Ok(())
}

fn run_wasm(
    wasm_bytes: Vec<u8>,
    entory_point: String,
    args: Vec<RuntimeValue>,
) -> anyhow::Result<Vec<RuntimeValue>> {
    let m = Module::from_byte(wasm_bytes)?;
    log::debug!("module: {:#?}", m);
    let instance = Instance::new(m);

    let values = instance.invoke(&entory_point, args)?;
    Ok(values)
}

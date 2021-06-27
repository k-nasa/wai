use clap::{AppSettings, Clap};
use wai::*;

#[derive(Clap)]
#[clap(
    version = "0.2.0",
    author = "k-nasa <htilcs1115@gmail.com>",
    about = "A simple wasm interpreter"
)]
#[clap(setting = AppSettings::ColoredHelp)]
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

    let m = Module::from_byte(bytes)?;
    let instance = Instance::new(m);

    // TODO implement custom argument
    let values = instance.invoke(&opts.invoke, opts.args)?;
    log::info!("return value is {:?}", values);

    Ok(())
}

use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Args {
    config_file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (config, config_file) = workbench::Config::load(args.config_file)?;
    let cwd = Path::parent(&config_file).expect("the config file must be in a directory");

    let runner = workbench::Runner::new(config, cwd);
    runner.run()
}
use anyhow::anyhow;
use std::env::{self, Args};

mod codegen;

fn main() -> Result<(), anyhow::Error> {
    let mut args = env::args();
    let command = args
        .nth(1)
        .ok_or_else(|| anyhow!("must specify a command"))?;

    run(&command, &mut args)
}

fn run(command: &str, args: &mut Args) -> Result<(), anyhow::Error> {
    match command {
        "codegen" => codegen::run(args),
        _ => Err(anyhow!("unrecognized command: {}", command)),
    }
}

use anyhow::anyhow;
use std::env;

mod codegen;

fn main() -> Result<(), anyhow::Error> {
    let command = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("must specify a command"))?;

    run(&command)
}

fn run(command: &str) -> Result<(), anyhow::Error> {
    match command {
        "codegen" => codegen::run(),
        _ => Err(anyhow!("unrecognized command: {}", command)),
    }
}

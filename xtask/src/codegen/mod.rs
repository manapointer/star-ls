use anyhow::anyhow;
use std::env::Args;

mod parser_tests;

pub fn run(args: &mut Args) -> Result<(), anyhow::Error> {
    let command = args
        .next()
        .ok_or_else(|| anyhow!("must specify a subcommand"))?;

    match command.as_str() {
        "parser-tests" => parser_tests::run(args),
        _ => Err(anyhow!("unrecognized subcommand: {}", command)),
    }
}

use star_syntax::parse_file;
use std::{
    collections::HashMap,
    env, mem,
    path::{Path, PathBuf},
};

#[derive(Default, Debug)]
struct CommentBlock {
    lines: Vec<String>,
}

#[derive(Debug)]
struct Test {
    name: String,
    text: String,
}

impl Test {
    fn new(name: String, text: String) -> Test {
        Test { name, text }
    }
}

fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

fn extract_comment_blocks(text: &str) -> Vec<CommentBlock> {
    let mut blocks = Vec::new();

    let prefix = "// ";
    let lines = text.lines().map(str::trim_start);

    let mut curr_block = <CommentBlock as Default>::default();
    for line in lines {
        let is_comment = line.starts_with(prefix);
        if is_comment {
            curr_block.lines.push(line[prefix.len()..].to_string());
        } else if !curr_block.lines.is_empty() {
            blocks.push(mem::take(&mut curr_block));
        }
    }
    if !curr_block.lines.is_empty() {
        blocks.push(curr_block);
    }
    blocks
}

fn add_tests_from_comment_blocks(tests: &mut HashMap<String, Test>, blocks: &[CommentBlock]) {
    for block in blocks {
        if block.lines.is_empty() {
            continue;
        }
        let header = block.lines[0].trim_start();
        let name = {
            let mut parts = header.split_ascii_whitespace();
            if parts.next() != Some("test") {
                continue;
            }
            match parts.next() {
                Some(name) => name,
                None => continue,
            }
        };

        let text = block.lines[1..].join("\n");
        if !text.is_empty() {
            tests.insert(name.to_string(), Test::new(name.to_string(), text));
        }
    }
}

pub fn run() -> Result<(), anyhow::Error> {
    let mut tests: HashMap<String, Test> = HashMap::new();

    // For each .rs file in crates/star_syntax/src/parser
    //   Parse all blocks

    let text = "
    
// x + y

// test def_stmt
// def foo():
//   pass";

    // println!("writing to {}", project_root().display());
    let blocks = extract_comment_blocks(text);

    add_tests_from_comment_blocks(&mut tests, &blocks);

    eprintln!("{:?}", tests);

    for test in tests.values() {
        let parse = parse_file(&test.text);
        println!("{}", star_syntax::render(parse.syntax()));
    }

    Ok(())
}

use std::{
    collections::HashMap,
    env, fs, mem,
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

        let mut lines = block.lines.iter().map(|s| s.as_str());

        let name = match {
            loop {
                match lines.next() {
                    Some(line) => {
                        let mut parts = line.trim_start().split_ascii_whitespace();
                        if let (Some("test"), Some(name)) = (parts.next(), parts.next()) {
                            break Some(name);
                        }
                    }
                    None => break None,
                }
            }
        } {
            Some(name) => name,
            None => continue,
        };

        let text = lines.collect::<Vec<_>>().join("\n");
        if !text.is_empty() {
            tests.insert(name.to_string(), Test::new(name.to_string(), text));
        }
    }
}

pub fn run() -> Result<(), anyhow::Error> {
    let mut tests: HashMap<String, Test> = HashMap::new();

    let dir = project_root().join(Path::new("crates/star_syntax/src/parser"));
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() != "rs" || !entry.file_type()?.is_file() {
            continue;
        }
        let code = fs::read_to_string(&path)?;
        let blocks = extract_comment_blocks(&code);

        add_tests_from_comment_blocks(&mut tests, &blocks);
    }

    let tests_dir = project_root().join(Path::new("crates/star_syntax/src/parser/test_data"));
    if !tests_dir.is_dir() {
        fs::create_dir(&tests_dir)?;
    }

    for test in tests.values() {
        let path = tests_dir.join(format!("{}.star", test.name));
        fs::write(path, &test.text)?;
    }

    Ok(())
}

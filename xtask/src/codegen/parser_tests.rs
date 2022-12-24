use std::{
    collections::{HashMap, HashSet},
    env::{self, Args},
    fs, mem,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

#[derive(Default, Debug)]
struct CommentBlock {
    lines: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum TestKind {
    Ok,
    Err,
}

#[derive(Debug)]
struct Test {
    kind: TestKind,
    name: String,
    text: String,
}

impl Test {
    fn new(kind: TestKind, name: String, text: String) -> Test {
        Test { kind, name, text }
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

fn add_tests_from_comment_blocks(
    tests: &mut HashMap<String, Test>,
    blocks: &[CommentBlock],
) -> Result<(), anyhow::Error> {
    for block in blocks {
        if block.lines.is_empty() {
            continue;
        }

        let mut lines = block.lines.iter().map(|s| s.as_str());

        let (kind, name) = match {
            loop {
                match lines.next() {
                    Some(line) => {
                        let mut parts = line.trim_start().split_ascii_whitespace();
                        if let (Some(test_type @ ("test" | "test_err")), Some(name)) =
                            (parts.next(), parts.next())
                        {
                            break Some((
                                if test_type == "test" {
                                    TestKind::Ok
                                } else {
                                    TestKind::Err
                                },
                                name,
                            ));
                        }
                    }
                    None => break None,
                }
            }
        } {
            Some(res) => res,
            None => continue,
        };
        if tests.contains_key(name) {
            return Err(anyhow!("duplicate test name: {}", name));
        }
        eprintln!("name: {}", name);
        let text = lines.collect::<Vec<_>>().join("\n");
        if !text.is_empty() {
            tests.insert(name.to_string(), Test::new(kind, name.to_string(), text));
        }
    }

    Ok(())
}

pub fn run(args: &mut Args) -> Result<(), anyhow::Error> {
    let update_tests = args.collect::<HashSet<_>>();
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

        add_tests_from_comment_blocks(&mut tests, &blocks)?;
    }

    let tests_dir = project_root().join(Path::new("crates/star_syntax/src/parser/test_data"));
    let ok_dir = tests_dir.join("ok");
    let err_dir = tests_dir.join("err");

    if !ok_dir.is_dir() {
        fs::create_dir_all(&ok_dir)?;
    }
    if !err_dir.is_dir() {
        fs::create_dir_all(&err_dir)?;
    }

    for test in tests
        .values()
        .filter(|&test| update_tests.is_empty() || update_tests.contains(&test.name))
    {
        let dir = match test.kind {
            TestKind::Ok => &ok_dir,
            TestKind::Err => &err_dir,
        };

        let path = dir.join(format!("{}.star", test.name));
        fs::write(path, &test.text)?;
    }

    Ok(())
}

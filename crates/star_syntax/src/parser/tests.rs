use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use expect_test::{expect_file, Expect};

use crate::{parse_file, render};

fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string()),
    )
    .ancestors()
    .nth(2)
    .unwrap()
    .to_path_buf()
}

fn collect_star_files(test_dir: &Path, subdir: &str) -> Result<Vec<(PathBuf, String)>, io::Error> {
    let mut res = Vec::new();
    let full_subdir = test_dir.join(subdir);
    for entry in fs::read_dir(&full_subdir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() != "star" || !entry.file_type()?.is_file() {
            continue;
        }
        let code = fs::read_to_string(&path)?;
        res.push((path, code));
    }
    Ok(res)
}

#[test]
fn dir_tests() {
    let star_files = collect_star_files(
        &project_root().join("crates/star_syntax/src/parser"),
        "test_data",
    )
    .unwrap();

    for (path, input) in star_files {
        let ast_path = path.with_extension("star.ast");
        check(&input, ast_path);
    }
}

fn check(input: &str, expect_path: PathBuf) {
    let parse = parse_file(input);
    let rendered = render(parse.syntax());
    let expect = expect_file![expect_path];
    expect.assert_eq(&rendered);
}

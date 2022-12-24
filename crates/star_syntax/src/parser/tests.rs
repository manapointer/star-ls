use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use expect_test::expect_file;

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

fn collect_star_files(test_dir: &Path) -> Result<Vec<(PathBuf, String, bool)>, io::Error> {
    let mut res = Vec::new();

    let ok_dir = test_dir.join("ok");
    let err_dir = test_dir.join("err");
    for entry in fs::read_dir(&ok_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() != "star" || !entry.file_type()?.is_file() {
            continue;
        }
        let code = fs::read_to_string(&path)?;
        res.push((path, code, true));
    }
    for entry in fs::read_dir(&err_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() != "star" || !entry.file_type()?.is_file() {
            continue;
        }
        let code = fs::read_to_string(&path)?;
        res.push((path, code, false));
    }
    Ok(res)
}

#[test]
fn dir_tests() {
    let star_files =
        collect_star_files(&project_root().join("crates/star_syntax/src/parser/test_data"))
            .unwrap();

    for (path, input, is_ok) in star_files {
        let ast_path = path.with_extension("star.ast");
        check(&input, ast_path, is_ok);
    }
}

fn check(input: &str, expect_path: PathBuf, is_ok: bool) {
    let parse = parse_file(input);
    let rendered = render(parse.syntax(), parse.errors);
    let expect = expect_file![expect_path];
    expect.assert_eq(&rendered);
}

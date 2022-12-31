use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use expect_test::expect_file;
use runfiles::find_runfiles_dir;

use crate::{parse_file, render};

fn project_root() -> PathBuf {
    find_runfiles_dir()
        .map(|p| p.join("star-ls"))
        .unwrap_or_else(|_| {
            Path::new(
                &env::var("CARGO_MANIFEST_DIR")
                    .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_string()),
            )
            .ancestors()
            .nth(2)
            .unwrap()
            .to_path_buf()
        })
}

fn collect_star_files(
    test_dir: &Path,
    filters: &[&str],
) -> Result<Vec<(PathBuf, String)>, io::Error> {
    let mut res = Vec::new();

    let ok_dir = test_dir.join("ok");
    let err_dir = test_dir.join("err");

    for entry in fs::read_dir(&ok_dir)?.chain(fs::read_dir(&err_dir)?) {
        let entry = entry?;
        let path = entry.path();
        if path.extension().unwrap_or_default() != "star" || entry.file_type()?.is_dir() {
            continue;
        }
        // Filter tests.
        if !filters.is_empty()
            && filters.iter().all(|&filter| {
                !path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(filter)
            })
        {
            continue;
        }
        let code = fs::read_to_string(&path)?;
        res.push((path, code));
    }
    Ok(res)
}

#[test]
fn dir_tests() {
    let filter_str = env::var("STAR_PARSER_TESTS").unwrap_or_else(|_| String::new());
    let filters: Vec<&str> = filter_str.split(',').map(str::trim).collect();

    eprintln!(
        "{}",
        project_root()
            .join("crates/star_syntax/src/parser/test_data")
            .display()
    );

    let star_files = collect_star_files(
        &project_root().join("crates/star_syntax/src/parser/test_data"),
        &filters,
    )
    .unwrap();

    for (path, input) in star_files {
        let ast_path = path.with_extension("star.ast");
        check(&input, ast_path);
    }
}

fn check(input: &str, expect_path: PathBuf) {
    let parse = parse_file(input);
    let rendered = render(parse.syntax(), parse.errors);
    let expect = expect_file![expect_path];
    expect.assert_eq(&rendered);
}

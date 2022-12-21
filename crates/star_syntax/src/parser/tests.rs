use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

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
        let name = entry.file_name();
        let name = name.to_str().unwrap();
        if !name.ends_with(".star") || !entry.file_type()?.is_file() {
            continue;
        }
        let path = full_subdir.join(name);
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

    for (_, code) in star_files {
        eprintln!("{}", code);
    }
}

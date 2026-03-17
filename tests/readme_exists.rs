#[test]
fn readme_exists_and_mentions_core_workflows() {
    let readme = include_str!("../README.md");

    assert!(readme.contains("# pgq"));
    assert!(readme.contains("cargo build --release"));
    assert!(readme.contains("scripts/release.sh"));
    assert!(readme.contains("postgresql-readonly-cli"));
}

#[test]
fn readme_exists_and_mentions_core_workflows() {
    let readme = include_str!("../README.md");

    assert!(readme.contains("# pgsql-query"));
    assert!(readme.contains("cargo build --release"));
    assert!(readme.contains("skills/pgsql-query/SKILL.md"));
    assert!(!readme.contains("scripts/release.sh"));
    assert!(!readme.contains("scripts/smoke.sh"));
    assert!(!readme.contains("scripts/package-release.sh"));
}

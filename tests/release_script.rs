#[test]
fn release_script_exists_and_mentions_dist_layout() {
    let script = include_str!("../scripts/release.sh");

    assert!(script.contains("dist/pgq"));
    assert!(script.contains("cargo build --release"));
    assert!(script.contains("skills/postgresql-readonly-cli/SKILL.md"));
    assert!(script.contains("README.md"));
    assert!(!script.contains("dist/pgq/scripts/smoke.sh"));
}

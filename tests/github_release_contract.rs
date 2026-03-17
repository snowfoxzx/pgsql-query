#[test]
fn package_script_exists_and_mentions_archive_naming() {
    let script = include_str!("../scripts/package-release.sh");

    assert!(script.contains("pgq-${VERSION}-${TARGET}"));
    assert!(script.contains(".tar.gz"));
    assert!(script.contains(".zip"));
    assert!(script.contains("README.md"));
    assert!(script.contains("skills/postgresql-readonly-cli/SKILL.md"));
}

#[test]
fn workflow_exists_and_mentions_release_matrix() {
    let workflow = include_str!("../.github/workflows/release.yml");

    assert!(workflow.contains("workflow_dispatch:"));
    assert!(workflow.contains("tags:"));
    assert!(workflow.contains("refs/tags/"));
    assert!(workflow.contains("x86_64-apple-darwin"));
    assert!(workflow.contains("aarch64-apple-darwin"));
    assert!(workflow.contains("x86_64-unknown-linux-gnu"));
    assert!(workflow.contains("aarch64-unknown-linux-gnu"));
    assert!(workflow.contains("x86_64-pc-windows-msvc"));
    assert!(workflow.contains("aarch64-pc-windows-msvc"));
    assert!(workflow.contains("softprops/action-gh-release"));
    assert!(workflow.contains("actions/upload-artifact"));
}

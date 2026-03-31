#[test]
fn package_script_exists_and_mentions_archive_naming() {
    let script = include_str!("../scripts/package-release.sh");

    assert!(script.contains("pgq-${OS}-${ARCH}"));
    assert!(script.contains(".tar.gz"));
    assert!(script.contains(".zip"));
    assert!(script.contains("README.md"));
    assert!(script.contains("skills/postgresql-readonly-cli"));
    assert!(script.contains("SHA256SUMS"));
}

#[test]
fn workflow_exists_and_mentions_release_matrix() {
    let workflow = include_str!("../.github/workflows/release.yml");

    assert!(workflow.contains("workflow_dispatch:"));
    assert!(workflow.contains("tags:"));
    assert!(workflow.contains("refs/tags/"));
    assert!(workflow.contains("macos"));
    assert!(workflow.contains("linux"));
    assert!(workflow.contains("windows"));
    assert!(workflow.contains("x86_64"));
    assert!(workflow.contains("aarch64"));
    assert!(workflow.contains("SHA256SUMS"));
    assert!(workflow.contains("softprops/action-gh-release"));
    assert!(workflow.contains("actions/upload-artifact"));
}

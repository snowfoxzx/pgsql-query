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
    assert!(workflow.contains("README.md"));
    assert!(workflow.contains("pgsql-query-${{ matrix.os_name }}-${{ matrix.arch_name }}"));
    assert!(workflow.contains("dist/releases/package/${{ matrix.binary_name }}"));
    assert!(workflow.contains("softprops/action-gh-release"));
    assert!(workflow.contains("actions/upload-artifact"));
}

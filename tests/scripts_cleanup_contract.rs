use std::path::Path;

#[test]
fn local_release_and_smoke_scripts_are_removed() {
    assert!(!Path::new("scripts/release.sh").exists());
    assert!(!Path::new("scripts/smoke.sh").exists());
    assert!(!Path::new("scripts/package-release.sh").exists());
}

#[test]
fn packaging_is_inlined_into_github_actions() {
    let workflow = include_str!("../.github/workflows/release.yml");

    assert!(workflow.contains("mkdir -p dist/releases/package"));
    assert!(workflow.contains("cp README.md dist/releases/package/README.md"));
    assert!(workflow.contains("cp \"target/${{ matrix.target }}/release/${{ matrix.binary_name }}\""));
}

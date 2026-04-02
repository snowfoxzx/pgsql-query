#[test]
fn crate_version_and_release_docs_are_in_sync() {
    let cargo_toml = include_str!("../Cargo.toml");
    let readme = include_str!("../README.md");
    let agents = include_str!("../AGENTS.md");

    assert!(cargo_toml.contains("version = \"0.1.2\""));
    assert!(readme.contains(
        "Before any release, update the crate version and ensure the `Cargo.toml` version matches the tag being published"
    ));
    assert!(agents.contains(
        "Before any release, update the crate version and ensure the `Cargo.toml` version matches the tag being published"
    ));
}

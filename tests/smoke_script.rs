#[test]
fn smoke_script_exists_and_has_help_text() {
    let script = include_str!("../scripts/smoke.sh");

    assert!(script.contains("Usage:"));
    assert!(script.contains("pgq ping"));
    assert!(script.contains("pgq schemas"));
}

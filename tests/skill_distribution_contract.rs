#[test]
fn skill_exists_in_top_level_skills_directory() {
    let skill = include_str!("../skills/postgresql-readonly-cli/SKILL.md");

    assert!(skill.contains("sh skills/postgresql-readonly-cli/scripts/install_pgq.sh"));
    assert!(skill.contains("skills/postgresql-readonly-cli/bin/pgq"));
    assert!(skill.contains("PGQ_REPO"));
    assert!(skill.contains("PGQ_VERSION"));
}

#[test]
fn installer_script_exists_and_mentions_release_resolution() {
    let script = include_str!("../skills/postgresql-readonly-cli/scripts/install_pgq.sh");

    assert!(script.contains("PGQ_REPO"));
    assert!(script.contains("PGQ_VERSION"));
    assert!(script.contains("PGQ_OS"));
    assert!(script.contains("PGQ_ARCH"));
    assert!(script.contains("PGQ_SHA256SUMS_FILE"));
    assert!(script.contains("SHA256SUMS"));
    assert!(script.contains("skills/postgresql-readonly-cli/bin"));
}

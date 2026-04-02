#[test]
fn skill_exists_in_top_level_skills_directory() {
    let skill = include_str!("../skills/pgsql-query/SKILL.md");

    assert!(skill.contains("name: pgsql-query"));
    assert!(skill.contains("sh \"$SKILL_DIR/scripts/install_pgsql_query.sh\""));
    assert!(skill.contains("\"$SKILL_DIR/bin/pgsql-query\""));
    assert!(!skill.contains("skills/pgsql-query/"));
    assert!(skill.contains("PGQ_REPO"));
    assert!(skill.contains("PGQ_VERSION"));
}

#[test]
fn installer_script_exists_and_mentions_release_resolution() {
    let script = include_str!("../skills/pgsql-query/scripts/install_pgsql_query.sh");

    assert!(script.contains("PGQ_REPO"));
    assert!(script.contains("PGQ_VERSION"));
    assert!(script.contains("PGQ_OS"));
    assert!(script.contains("PGQ_ARCH"));
    assert!(script.contains("PGQ_SHA256SUMS_FILE"));
    assert!(script.contains("SHA256SUMS"));
    assert!(script.contains("BIN_DIR=\"$SKILL_DIR/bin\""));
    assert!(script.contains("pgsql-query"));
    assert!(script.contains("find \"$temp_dir\" -name \"$binary\" -type f"));
}

#[test]
fn codex_plugin_manifest_exposes_top_level_skills_directory() {
    let manifest = include_str!("../.codex-plugin/plugin.json");

    assert!(manifest.contains("\"skills\": \"./skills/\""));
    assert!(manifest.contains("\"name\": \"pgsql-query\""));
    assert!(manifest.contains("\"description\": \"pgsql-query\""));
}

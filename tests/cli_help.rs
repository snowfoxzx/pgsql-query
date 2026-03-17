use clap::CommandFactory;
use pgq::cli::Cli;

#[test]
fn help_mentions_supported_environment_variables() {
    let mut command = Cli::command();
    let help = command.render_long_help().to_string();

    assert!(help.contains("PGQ_URL"));
    assert!(help.contains("PGQ_HOST"));
    assert!(help.contains("PGQ_PORT"));
    assert!(help.contains("PGQ_USER"));
    assert!(help.contains("PGQ_PASS"));
    assert!(help.contains("PGQ_DB"));
}

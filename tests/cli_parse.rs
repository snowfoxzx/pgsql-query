use clap::Parser;
use pgq::cli::{Cli, Commands};
use pgq::config::ConnectionConfig;

#[test]
fn parses_url_connection_options() {
    let cli = Cli::try_parse_from([
        "pgq",
        "--url",
        "postgres://demo:secret@localhost:5432/app",
        "ping",
    ])
    .expect("cli should parse");

    let resolved = ConnectionConfig::from_sources(&cli).expect("config should resolve");

    assert_eq!(
        resolved.url.as_deref(),
        Some("postgres://demo:secret@localhost:5432/app")
    );
    assert!(matches!(cli.command, Commands::Ping));
}

#[test]
fn parses_field_based_connection_options() {
    let cli = Cli::try_parse_from([
        "pgq",
        "--host",
        "db.internal",
        "--port",
        "5433",
        "--user",
        "reader",
        "--password",
        "secret",
        "--dbname",
        "analytics",
        "tables",
    ])
    .expect("cli should parse");

    let resolved = ConnectionConfig::from_sources(&cli).expect("config should resolve");

    assert_eq!(resolved.host.as_deref(), Some("db.internal"));
    assert_eq!(resolved.port, Some(5433));
    assert_eq!(resolved.user.as_deref(), Some("reader"));
    assert_eq!(resolved.password.as_deref(), Some("secret"));
    assert_eq!(resolved.dbname.as_deref(), Some("analytics"));
    assert!(matches!(cli.command, Commands::Tables { .. }));
}

#[test]
fn summarizes_url_connection_target_without_password() {
    let cli = Cli::try_parse_from([
        "pgq",
        "--url",
        "postgres://demo:secret@db.internal:5432/app",
        "ping",
    ])
    .expect("cli should parse");

    let resolved = ConnectionConfig::from_sources(&cli).expect("config should resolve");

    assert_eq!(
        resolved.target_summary().as_deref(),
        Some("postgres://demo@db.internal:5432/app")
    );
}

#[test]
fn summarizes_field_based_connection_target() {
    let cli = Cli::try_parse_from([
        "pgq",
        "--host",
        "db.internal",
        "--port",
        "5433",
        "--user",
        "reader",
        "--dbname",
        "analytics",
        "ping",
    ])
    .expect("cli should parse");

    let resolved = ConnectionConfig::from_sources(&cli).expect("config should resolve");

    assert_eq!(
        resolved.target_summary().as_deref(),
        Some("host=db.internal port=5433 user=reader dbname=analytics")
    );
}

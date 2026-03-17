use anyhow::{Result, anyhow};
use tokio_postgres::Config as PgConfig;

use crate::cli::Cli;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConnectionConfig {
    pub url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub dbname: Option<String>,
}

impl ConnectionConfig {
    pub fn from_sources(cli: &Cli) -> Result<Self> {
        let config = Self {
            url: cli.url.clone().or_else(|| std::env::var("DATABASE_URL").ok()),
            host: cli.host.clone().or_else(|| std::env::var("PGHOST").ok()),
            port: cli
                .port
                .or_else(|| std::env::var("PGPORT").ok().and_then(|value| value.parse().ok())),
            user: cli.user.clone().or_else(|| std::env::var("PGUSER").ok()),
            password: cli
                .password
                .clone()
                .or_else(|| std::env::var("PGPASSWORD").ok()),
            dbname: cli
                .dbname
                .clone()
                .or_else(|| std::env::var("PGDATABASE").ok()),
        };

        if config.url.is_none()
            && config.host.is_none()
            && config.port.is_none()
            && config.user.is_none()
            && config.password.is_none()
            && config.dbname.is_none()
        {
            return Err(anyhow!(
                "missing connection info, provide --url or PostgreSQL connection fields"
            ));
        }

        Ok(config)
    }

    pub fn to_pg_config(&self) -> Result<PgConfig> {
        if let Some(url) = &self.url {
            return url.parse().map_err(Into::into);
        }

        let mut config = PgConfig::new();
        if let Some(host) = &self.host {
            config.host(host);
        }
        if let Some(port) = self.port {
            config.port(port);
        }
        if let Some(user) = &self.user {
            config.user(user);
        }
        if let Some(password) = &self.password {
            config.password(password);
        }
        if let Some(dbname) = &self.dbname {
            config.dbname(dbname);
        }
        Ok(config)
    }

    pub fn target_summary(&self) -> Option<String> {
        if let Some(url) = &self.url {
            return summarize_url(url);
        }

        let mut parts = Vec::new();
        if let Some(host) = &self.host {
            parts.push(format!("host={host}"));
        }
        if let Some(port) = self.port {
            parts.push(format!("port={port}"));
        }
        if let Some(user) = &self.user {
            parts.push(format!("user={user}"));
        }
        if let Some(dbname) = &self.dbname {
            parts.push(format!("dbname={dbname}"));
        }

        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}

fn summarize_url(url: &str) -> Option<String> {
    let parsed = url.parse::<PgConfig>().ok()?;
    let host = parsed.get_hosts().first().map(|host| match host {
        tokio_postgres::config::Host::Tcp(name) => name.to_string(),
        tokio_postgres::config::Host::Unix(path) => path.display().to_string(),
    })?;
    let port = parsed.get_ports().first().copied().unwrap_or(5432);
    let user = parsed.get_user().map(str::to_owned);
    let dbname = parsed.get_dbname().map(str::to_owned);

    let mut summary = String::from("postgres://");
    if let Some(user) = user {
        summary.push_str(&user);
        summary.push('@');
    }
    summary.push_str(&host);
    summary.push(':');
    summary.push_str(&port.to_string());
    if let Some(dbname) = dbname {
        summary.push('/');
        summary.push_str(&dbname);
    }
    Some(summary)
}

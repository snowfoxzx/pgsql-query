use anyhow::{Context, Result, anyhow};
use tokio_postgres::{Client, NoTls, SimpleQueryMessage};

use crate::config::ConnectionConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Option<String>>>,
}

pub async fn connect(config: &ConnectionConfig) -> Result<Client> {
    let pg_config = config.to_pg_config()?;
    let target = config
        .target_summary()
        .unwrap_or_else(|| "unknown target".to_string());
    let (client, connection) = pg_config.connect(NoTls).await.with_context(|| {
        format!(
            "error connecting to server ({target}). If you are using a URL, verify the host name and URL-encode reserved characters in the username/password."
        )
    })?;
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("connection error: {error}");
        }
    });
    Ok(client)
}

pub async fn run_query(client: &Client, sql: &str) -> Result<QueryResult> {
    let messages = client.simple_query(sql).await?;
    let mut columns = Vec::new();
    let mut rows = Vec::new();

    for message in messages {
        if let SimpleQueryMessage::Row(row) = message {
            if columns.is_empty() {
                columns = row
                    .columns()
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect();
            }
            let values = (0..row.len()).map(|idx| row.get(idx).map(str::to_owned)).collect();
            rows.push(values);
        }
    }

    Ok(QueryResult { columns, rows })
}

pub async fn ping(client: &Client) -> Result<()> {
    let result = run_query(client, "select 1 as ok").await?;
    if result.rows.is_empty() {
        return Err(anyhow!("server did not return a ping result"));
    }
    Ok(())
}

pub fn quote_ident(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

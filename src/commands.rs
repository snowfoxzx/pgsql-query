use anyhow::{Result, anyhow, bail};
use tokio_postgres::Client;

use crate::{
    cli::{Cli, Commands},
    config::ConnectionConfig,
    db::{QueryResult, connect, ping, quote_ident, run_query},
    output::{print_message, print_result},
};

pub async fn run(cli: Cli) -> Result<()> {
    let config = ConnectionConfig::from_sources(&cli)?;
    let client = connect(&config).await?;

    match cli.command {
        Commands::Ping => {
            ping(&client).await?;
            print_message("ok", cli.json)?;
        }
        Commands::Databases => {
            let result = databases(&client).await?;
            print_result(&result, cli.json)?;
        }
        Commands::Schemas => {
            let result = schemas(&client).await?;
            print_result(&result, cli.json)?;
        }
        Commands::Tables { schema } => {
            let result = tables(&client, schema.as_deref()).await?;
            print_result(&result, cli.json)?;
        }
        Commands::Describe { table, schema } => {
            let result = describe(&client, schema.as_deref(), &table).await?;
            print_result(&result, cli.json)?;
        }
        Commands::Sample {
            table,
            schema,
            limit,
        } => {
            let result = sample(&client, schema.as_deref(), &table, limit).await?;
            print_result(&result, cli.json)?;
        }
        Commands::Query { sql } => {
            if !is_read_only_sql(&sql) {
                bail!("only read-only queries starting with SELECT, WITH, or EXPLAIN are allowed");
            }
            let result = run_query(&client, &sql).await?;
            print_result(&result, cli.json)?;
        }
    }

    Ok(())
}

pub fn is_read_only_sql(sql: &str) -> bool {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.contains(';') {
        return false;
    }

    let first = trimmed
        .split_whitespace()
        .next()
        .map(|token| token.to_ascii_lowercase())
        .unwrap_or_default();

    matches!(first.as_str(), "select" | "with" | "explain")
}

async fn databases(client: &Client) -> Result<QueryResult> {
    run_query(
        client,
        "select datname as database \
         from pg_database \
         where datistemplate = false \
         order by datname",
    )
    .await
}

async fn schemas(client: &Client) -> Result<QueryResult> {
    run_query(
        client,
        "select schema_name \
         from information_schema.schemata \
         where schema_name not in ('information_schema') \
           and schema_name not like 'pg_%' \
         order by schema_name",
    )
    .await
}

async fn tables(client: &Client, schema: Option<&str>) -> Result<QueryResult> {
    let sql = if let Some(schema) = schema {
        format!(
            "select table_schema, table_name, table_type \
             from information_schema.tables \
             where table_schema = '{}' \
             order by table_name",
            escape_literal(schema)
        )
    } else {
        "select table_schema, table_name, table_type \
         from information_schema.tables \
         where table_schema not in ('information_schema') \
           and table_schema not like 'pg_%' \
         order by table_schema, table_name"
            .to_string()
    };

    run_query(client, &sql).await
}

async fn describe(client: &Client, schema: Option<&str>, table: &str) -> Result<QueryResult> {
    let schema = schema.unwrap_or("public");
    let sql = format!(
        "select \
             cols.column_name, \
             cols.data_type, \
             cols.is_nullable, \
             cols.column_default, \
             case when tc.constraint_type = 'PRIMARY KEY' then 'YES' else 'NO' end as primary_key \
         from information_schema.columns cols \
         left join information_schema.key_column_usage kcu \
           on cols.table_schema = kcu.table_schema \
          and cols.table_name = kcu.table_name \
          and cols.column_name = kcu.column_name \
         left join information_schema.table_constraints tc \
           on kcu.constraint_name = tc.constraint_name \
          and kcu.table_schema = tc.table_schema \
          and tc.constraint_type = 'PRIMARY KEY' \
         where cols.table_schema = '{}' \
           and cols.table_name = '{}' \
         order by cols.ordinal_position",
        escape_literal(schema),
        escape_literal(table)
    );

    let result = run_query(client, &sql).await?;
    if result.columns.is_empty() && result.rows.is_empty() {
        return Err(anyhow!("table {schema}.{table} was not found"));
    }
    Ok(result)
}

async fn sample(client: &Client, schema: Option<&str>, table: &str, limit: u32) -> Result<QueryResult> {
    let schema = schema.unwrap_or("public");
    let safe_limit = limit.max(1);
    let sql = format!(
        "select * from {}.{} limit {}",
        quote_ident(schema),
        quote_ident(table),
        safe_limit
    );
    run_query(client, &sql).await
}

fn escape_literal(value: &str) -> String {
    value.replace('\'', "''")
}

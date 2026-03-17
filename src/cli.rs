use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pgq", version, about = "Read-only PostgreSQL CLI")]
pub struct Cli {
    #[arg(long, global = true)]
    pub url: Option<String>,
    #[arg(long, global = true)]
    pub host: Option<String>,
    #[arg(long, global = true)]
    pub port: Option<u16>,
    #[arg(long, global = true)]
    pub user: Option<String>,
    #[arg(long, global = true)]
    pub password: Option<String>,
    #[arg(long, global = true)]
    pub dbname: Option<String>,
    #[arg(long, global = true, help = "Render result sets as JSON")]
    pub json: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Ping,
    Databases,
    Schemas,
    Tables {
        #[arg(long)]
        schema: Option<String>,
    },
    Describe {
        table: String,
        #[arg(long)]
        schema: Option<String>,
    },
    Sample {
        table: String,
        #[arg(long)]
        schema: Option<String>,
        #[arg(long, default_value_t = 20)]
        limit: u32,
    },
    Query {
        #[arg(long)]
        sql: String,
    },
}

use anyhow::Result;
use comfy_table::{Cell, ContentArrangement, Table};
use serde_json::{Map, Value, json};

use crate::db::QueryResult;

pub fn print_result(result: &QueryResult, as_json: bool) -> Result<()> {
    if as_json {
        println!("{}", serde_json::to_string_pretty(&as_json_value(result))?);
    } else {
        println!("{}", render_table(result));
    }
    Ok(())
}

pub fn print_message(message: &str, as_json: bool) -> Result<()> {
    if as_json {
        println!("{}", serde_json::to_string_pretty(&json!({ "message": message }))?);
    } else {
        println!("{message}");
    }
    Ok(())
}

fn as_json_value(result: &QueryResult) -> Value {
    let rows = result
        .rows
        .iter()
        .map(|row| {
            let mut map = Map::new();
            for (index, column) in result.columns.iter().enumerate() {
                match row.get(index).cloned().flatten() {
                    Some(value) => {
                        map.insert(column.clone(), Value::String(value));
                    }
                    None => {
                        map.insert(column.clone(), Value::Null);
                    }
                }
            }
            Value::Object(map)
        })
        .collect::<Vec<_>>();

    Value::Array(rows)
}

fn render_table(result: &QueryResult) -> String {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    if !result.columns.is_empty() {
        table.set_header(result.columns.iter().map(Cell::new));
    }

    for row in &result.rows {
        let display_row = row
            .iter()
            .map(|value| Cell::new(value.clone().unwrap_or_else(|| "NULL".to_string())))
            .collect::<Vec<_>>();
        table.add_row(display_row);
    }

    table.to_string()
}

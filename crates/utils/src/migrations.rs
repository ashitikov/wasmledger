use std::collections::HashMap;

use wasmledger_sql_client::base::{
    bindings::wasm_sql::core::{
        query::{self, QueryExecutor},
        query_types::{SqlArguments, SqlQuery},
        util_types::Error as SqlError,
    },
    decode::DecodeRequired,
    row::RowPointer,
};
use wasmledger_sql_client::postgres::bindings::wasm_sql::postgres::codecs::{
    get_string, push_string,
};

#[derive(Debug, PartialEq, Eq)]
pub struct ColumnInfo {
    pub data_type: String,
    pub nullable: bool,
    pub default: Option<String>,
}

#[derive(Debug)]
pub struct TableInfo<'a> {
    name: &'a str,
    columns: HashMap<String, ColumnInfo>,
}

impl<'a> TryFrom<&RowPointer<'a>> for ColumnInfo {
    type Error = SqlError;

    fn try_from(value: &RowPointer<'a>) -> Result<Self, Self::Error> {
        Ok(ColumnInfo {
            data_type: value.column("data_type").decode(get_string).required()?,
            nullable: value.column("is_nullable").decode(get_string)? == Some("YES".to_string()),
            default: value.column("column_default").decode(get_string)?,
        })
    }
}

pub async fn load_table_schema<'a>(
    executor: QueryExecutor<'_>,
    table: &'a str,
) -> Result<TableInfo<'a>, SqlError> {
    let args = SqlArguments::new();
    push_string(Some(table), &args)?;

    let query = SqlQuery {
        sql: r#"
            SELECT
                column_name,
                data_type,
                is_nullable,
                column_default
            FROM information_schema.columns
            WHERE table_name = $1
        "#
        .to_string(),
        args: Some(args),
        persistent: None,
    };

    let query_results = query::fetch_all(query, executor).await?;

    let mut columns = HashMap::new();
    for row in query_results.into_iter() {
        let info = ColumnInfo::try_from(&row)?;
        let column_name = row.column("column_name").decode(get_string).required()?;

        columns.insert(column_name, info);
    }

    Ok(TableInfo {
        name: table,
        columns,
    })
}

#[derive(Debug)]
pub struct ColumnExpectationError {
    pub message: String,
}

impl From<String> for ColumnExpectationError {
    fn from(value: String) -> Self {
        ColumnExpectationError { message: value }
    }
}

impl From<ColumnExpectationError> for String {
    fn from(value: ColumnExpectationError) -> Self {
        value.message
    }
}

/// Generates a `Migration { id, sql }` from a file name.
/// Expects the SQL file at `../migrations/<name>.sql` relative to the calling file.
///
/// Usage: `sql_migration!("001_base")` — requires `Migration` to be in scope.
#[macro_export]
macro_rules! sql_migration {
    ($name:literal) => {
        Migration {
            id: $name.to_string(),
            sql: include_str!(concat!("../migrations/", $name, ".sql")).to_string(),
        }
    };
}

pub fn expect_column(
    table: &TableInfo<'_>,
    column: &str,
    expectation: ColumnInfo,
) -> Result<(), ColumnExpectationError> {
    let col = table.columns.get(&column.to_string()).ok_or_else(|| {
        ColumnExpectationError::from(format!("{}.{} does not exist", table.name, column))
    })?;

    if expectation != *col {
        return Err(ColumnExpectationError::from(format!(
            "{}.{} expected {:?}. got {:?}",
            table.name, column, expectation, col
        )));
    }

    Ok(())
}

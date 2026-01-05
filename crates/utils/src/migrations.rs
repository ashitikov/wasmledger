use std::collections::HashMap;

use wasmledger_sql_client::base::{
    bindings::wasmledger::sql::{
        query::{self, QueryExecutor},
        query_types::{SqlArguments, SqlQuery},
        util_types::Error as SqlError,
    },
    decode::DecodeRequired,
    row::RowPointer,
};
use wasmledger_sql_client::postgres::bindings::wasmledger::sql_postgres::postgres_codecs::{
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
        args: args,
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

pub struct ColumnExpectationError {
    pub message: String,
}

impl From<String> for ColumnExpectationError {
    fn from(value: String) -> Self {
        ColumnExpectationError { message: value }
    }
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

#[macro_export]
macro_rules! impl_expectation_to_schema_error {
    ($source:ty, $target:ty) => {
        impl From<$source> for $target {
            fn from(e: $source) -> Self {
                <$target>::SchemaInvalid(e.message)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_sql_to_schema_error {
    ($source:ty, $target:ty) => {
        impl From<$source> for $target {
            fn from(e: $source) -> Self {
                <$target>::Sql(e)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_migrations_guest_partially {
    ($bindings:ident, $migration_group:expr, $migrations:ident) => {
        fn get_migration_group() -> String {
            $migration_group.to_string()
        }

        async fn list_migrations() -> Vec<$bindings::migrations::MigrationId> {
            $migrations.0.clone()
        }

        async fn apply_migration(
            id: $bindings::migrations::MigrationId,
            executor: wasmledger_sql_client::base::bindings::wasmledger::sql::query::QueryExecutor<
                '_,
            >,
        ) -> Result<(), wasmledger_sql_client::base::bindings::wasmledger::sql::util_types::Error> {
            let query = $migrations
                .1
                .get(&id)
                .expect(&format!("Migration {} not found", id));

            wasmledger_sql_client::base::bindings::wasmledger::sql::query::execute_raw(
                query.to_string(),
                executor,
            )
            .await
        }
    };
}

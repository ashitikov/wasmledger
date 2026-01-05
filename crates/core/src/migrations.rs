use std::{collections::HashMap, sync::LazyLock};

use wasmledger_migrations_macro::load_migrations;
use wasmledger_sql_client::base::bindings::wasmledger::sql::{
    query::QueryExecutor, query_types::SqlString,
};
use wasmledger_utils::{
    impl_expectation_to_schema_error, impl_migrations_guest_partially, impl_sql_to_schema_error,
    migrations::{ColumnExpectationError, ColumnInfo, expect_column, load_table_schema},
};

use crate::{
    BindingsImpl,
    bindings::exports::wasmledger::plugin::migrations::{Error, MigrationId, SchemaError},
};

use crate::bindings::exports::wasmledger::plugin as module_bindings;

static MIGRATIONS: LazyLock<(Vec<MigrationId>, HashMap<MigrationId, SqlString>)> =
    LazyLock::new(|| load_migrations!("./src/migrations"));

impl crate::bindings::exports::wasmledger::plugin::migrations::Guest for BindingsImpl {
    impl_migrations_guest_partially!(module_bindings, "wasmledger_core", MIGRATIONS);

    async fn check_schema(executor: QueryExecutor<'_>) -> Result<(), SchemaError> {
        let t_accounts = load_table_schema(executor, "accounts").await?;
        let t_transfers = load_table_schema(executor, "transfers").await?;

        // -------- accounts --------
        expect_column(
            &t_accounts,
            "id",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_accounts,
            "bucket",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_accounts,
            "asset",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_accounts,
            "in_volume",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: false,
                default: Some("0".to_string()),
            },
        )?;

        expect_column(
            &t_accounts,
            "out_volume",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: false,
                default: Some("0".to_string()),
            },
        )?;

        expect_column(
            &t_accounts,
            "last_transfer_id",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: true,
                default: None,
            },
        )?;

        // -------- transfers --------
        expect_column(
            &t_transfers,
            "id",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "src",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "dst",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "src_bucket",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: true,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "dst_bucket",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: true,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "asset",
            ColumnInfo {
                data_type: "TEXT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "amount",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: false,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "src_bucket_in_volume",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: true,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "src_bucket_out_volume",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: true,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "dst_bucket_in_volume",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: true,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "dst_bucket_out_volume",
            ColumnInfo {
                data_type: "BIGINT".to_string(),
                nullable: true,
                default: None,
            },
        )?;

        expect_column(
            &t_transfers,
            "created_at",
            ColumnInfo {
                data_type: "TIMESTAMP WITHOUT TIME ZONE".to_string(),
                nullable: false,
                default: Some("now()".to_string()),
            },
        )?;

        Ok(())
    }
}

impl_expectation_to_schema_error!(ColumnExpectationError, SchemaError);
impl_sql_to_schema_error!(Error, SchemaError);

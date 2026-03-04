use wasmledger_sql_client::base::bindings::wasm_sql::core::query::QueryExecutor;
use wasmledger_utils::migrations::{ColumnInfo, expect_column, load_table_schema};

use crate::BindingsImpl;
use crate::bindings::exports::wasmledger::extension::lifecycle::Guest;

impl Guest for BindingsImpl {
    async fn on_init() {
        check_schema().await.expect("schema validation failed");
    }

    async fn on_shutdown() {}
}

async fn check_schema() -> Result<(), String> {
    let t_accounts = load_table_schema(QueryExecutor::Pool, "accounts")
        .await
        .map_err(|e| format!("failed to load accounts schema: {e}"))?;

    let t_transfers = load_table_schema(QueryExecutor::Pool, "transfers")
        .await
        .map_err(|e| format!("failed to load transfers schema: {e}"))?;

    // -------- accounts --------
    expect_column(
        &t_accounts,
        "id",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_accounts,
        "bucket",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_accounts,
        "currency",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_accounts,
        "precision",
        ColumnInfo {
            data_type: "smallint".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_accounts,
        "in_volume",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: false,
            default: Some("0".to_string()),
        },
    )?;

    expect_column(
        &t_accounts,
        "out_volume",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: false,
            default: Some("0".to_string()),
        },
    )?;

    expect_column(
        &t_accounts,
        "last_transfer_id",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: true,
            default: None,
        },
    )?;

    // -------- transfers --------
    expect_column(
        &t_transfers,
        "id",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "src",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "dst",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "src_bucket",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: true,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "dst_bucket",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: true,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "currency",
        ColumnInfo {
            data_type: "text".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "precision",
        ColumnInfo {
            data_type: "smallint".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "amount",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: false,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "src_bucket_in_volume",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: true,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "src_bucket_out_volume",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: true,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "dst_bucket_in_volume",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: true,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "dst_bucket_out_volume",
        ColumnInfo {
            data_type: "bigint".to_string(),
            nullable: true,
            default: None,
        },
    )?;

    expect_column(
        &t_transfers,
        "created_at",
        ColumnInfo {
            data_type: "timestamp with time zone".to_string(),
            nullable: false,
            default: Some("now()".to_string()),
        },
    )?;

    Ok(())
}

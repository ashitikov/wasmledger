#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod bindings {
    #[allow(unfulfilled_lint_expectations, unused_imports)]
    use wasmledger_sql_client::base::bindings::wasmledger::sql::util_types as __with_name0;
    #[allow(unfulfilled_lint_expectations, unused_imports)]
    use wasmledger_sql_client::base::bindings::wasmledger::sql::transaction as __with_name1;
    #[allow(unfulfilled_lint_expectations, unused_imports)]
    use wasmledger_sql_client::base::bindings::wasmledger::sql::query_types as __with_name2;
    #[allow(unfulfilled_lint_expectations, unused_imports)]
    use wasmledger_sql_client::base::bindings::wasmledger::sql::query as __with_name3;
    #[allow(unfulfilled_lint_expectations, unused_imports)]
    use wasmledger_sql_client::base::bindings::wasmledger::sql::codecs as __with_name4;
    #[allow(unfulfilled_lint_expectations, unused_imports)]
    use wasmledger_sql_client::postgres::bindings::wasmledger::sql_postgres::postgres_codecs as __with_name5;
    #[allow(dead_code, clippy::all)]
    pub mod exports {
        pub mod wasmledger {
            pub mod module {
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod migrations {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::super::_rt;
                    pub type QueryExecutor<'a> = super::super::super::super::__with_name3::QueryExecutor<
                        'a,
                    >;
                    pub type Error = super::super::super::super::__with_name0::Error;
                    pub type MigrationId = _rt::String;
                    pub enum SchemaError {
                        SchemaInvalid(_rt::String),
                        Sql(Error),
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for SchemaError {
                        #[inline]
                        fn clone(&self) -> SchemaError {
                            match self {
                                SchemaError::SchemaInvalid(__self_0) => {
                                    SchemaError::SchemaInvalid(
                                        ::core::clone::Clone::clone(__self_0),
                                    )
                                }
                                SchemaError::Sql(__self_0) => {
                                    SchemaError::Sql(::core::clone::Clone::clone(__self_0))
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Debug for SchemaError {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            match self {
                                SchemaError::SchemaInvalid(e) => {
                                    f.debug_tuple("SchemaError::SchemaInvalid")
                                        .field(e)
                                        .finish()
                                }
                                SchemaError::Sql(e) => {
                                    f.debug_tuple("SchemaError::Sql").field(e).finish()
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Display for SchemaError {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.write_fmt(format_args!("{0:?}", self))
                        }
                    }
                    impl ::core::error::Error for SchemaError {}
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_unsafe)]
                    pub unsafe fn _export_get_module_id_cabi<T: Guest>() -> *mut u8 {
                        unsafe {
                            let result0 = { T::get_module_id() };
                            let ptr1 = (&raw mut _RET_AREA.0).cast::<u8>();
                            let vec2 = (result0.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<usize>() = len2;
                            *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                            ptr1
                        }
                    }
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    pub unsafe fn __post_return_get_module_id<T: Guest>(arg0: *mut u8) {
                        unsafe {
                            let l0 = *arg0.add(0).cast::<*mut u8>();
                            let l1 = *arg0
                                .add(::core::mem::size_of::<*const u8>())
                                .cast::<usize>();
                            _rt::cabi_dealloc(l0, l1, 1);
                        }
                    }
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_unsafe)]
                    pub unsafe fn _export_async_list_migrations_cabi<T: Guest>() -> i32 {
                        unsafe {
                            wit_bindgen::rt::async_support::start_task(async move {
                                let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                let result0 = &{ T::list_migrations().await };
                                let vec2 = result0;
                                let len2 = vec2.len();
                                let layout2 = _rt::alloc::Layout::from_size_align(
                                        vec2.len() * (2 * ::core::mem::size_of::<*const u8>()),
                                        ::core::mem::size_of::<*const u8>(),
                                    )
                                    .unwrap();
                                let (result2, _cleanup2) = wit_bindgen::rt::Cleanup::new(
                                    layout2,
                                );
                                for (i, e) in vec2.into_iter().enumerate() {
                                    let base = result2
                                        .add(i * (2 * ::core::mem::size_of::<*const u8>()));
                                    {
                                        let vec1 = e;
                                        let ptr1 = vec1.as_ptr().cast::<u8>();
                                        let len1 = vec1.len();
                                        *base
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>() = len1;
                                        *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                    }
                                }
                                unsafe extern "C" fn wit_import3(_: *mut u8, _: usize) {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _task_cancel.forget();
                                wit_import3(result2, len2);
                            })
                        }
                    }
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    pub unsafe fn __callback_async_list_migrations(
                        event0: u32,
                        event1: u32,
                        event2: u32,
                    ) -> u32 {
                        unsafe {
                            wit_bindgen::rt::async_support::callback(
                                event0,
                                event1,
                                event2,
                            )
                        }
                    }
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_unsafe)]
                    pub unsafe fn _export_async_apply_migration_cabi<T: Guest>(
                        arg0: *mut u8,
                        arg1: usize,
                        arg2: i32,
                        arg3: i32,
                    ) -> i32 {
                        unsafe {
                            wit_bindgen::rt::async_support::start_task(async move {
                                let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                let result3 = &{
                                    let handle1;
                                    let len0 = arg1;
                                    let bytes0 = _rt::Vec::from_raw_parts(
                                        arg0.cast(),
                                        len0,
                                        len0,
                                    );
                                    use super::super::super::super::__with_name3::QueryExecutor as V2;
                                    let v2 = match arg2 {
                                        0 => V2::Pool,
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            let e2 = {
                                                handle1 = super::super::super::super::__with_name1::Transaction::from_handle(
                                                    arg3 as u32,
                                                );
                                                &handle1
                                            };
                                            V2::Transaction(e2)
                                        }
                                    };
                                    T::apply_migration(_rt::string_lift(bytes0), v2).await
                                };
                                let (result9_0, result9_1, result9_2, result9_3) = match result3 {
                                    Ok(_) => (0i32, 0i32, ::core::ptr::null_mut(), 0usize),
                                    Err(e) => {
                                        use super::super::super::super::__with_name0::Error as V7;
                                        let (result8_0, result8_1, result8_2) = match e {
                                            V7::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                            V7::Encode(e) => {
                                                let vec4 = e;
                                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                                let len4 = vec4.len();
                                                (1i32, ptr4.cast_mut(), len4)
                                            }
                                            V7::Decode(e) => {
                                                let vec5 = e;
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                (2i32, ptr5.cast_mut(), len5)
                                            }
                                            V7::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                            V7::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                            V7::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                            V7::Unexpected(e) => {
                                                let vec6 = e;
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                (6i32, ptr6.cast_mut(), len6)
                                            }
                                        };
                                        (1i32, result8_0, result8_1, result8_2)
                                    }
                                };
                                unsafe extern "C" fn wit_import10(
                                    _: i32,
                                    _: i32,
                                    _: *mut u8,
                                    _: usize,
                                ) {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _task_cancel.forget();
                                wit_import10(result9_0, result9_1, result9_2, result9_3);
                            })
                        }
                    }
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    pub unsafe fn __callback_async_apply_migration(
                        event0: u32,
                        event1: u32,
                        event2: u32,
                    ) -> u32 {
                        unsafe {
                            wit_bindgen::rt::async_support::callback(
                                event0,
                                event1,
                                event2,
                            )
                        }
                    }
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_unsafe)]
                    pub unsafe fn _export_async_check_schema_cabi<T: Guest>(
                        arg0: i32,
                        arg1: i32,
                    ) -> i32 {
                        unsafe {
                            wit_bindgen::rt::async_support::start_task(async move {
                                let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                let result2 = &{
                                    let handle0;
                                    use super::super::super::super::__with_name3::QueryExecutor as V1;
                                    let v1 = match arg0 {
                                        0 => V1::Pool,
                                        n => {
                                            if true {
                                                match (&n, &1) {
                                                    (left_val, right_val) => {
                                                        if !(*left_val == *right_val) {
                                                            let kind = ::core::panicking::AssertKind::Eq;
                                                            ::core::panicking::assert_failed(
                                                                kind,
                                                                &*left_val,
                                                                &*right_val,
                                                                ::core::option::Option::Some(
                                                                    format_args!("invalid enum discriminant"),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                };
                                            }
                                            let e1 = {
                                                handle0 = super::super::super::super::__with_name1::Transaction::from_handle(
                                                    arg1 as u32,
                                                );
                                                &handle0
                                            };
                                            V1::Transaction(e1)
                                        }
                                    };
                                    T::check_schema(v1).await
                                };
                                let (
                                    result10_0,
                                    result10_1,
                                    result10_2,
                                    result10_3,
                                    result10_4,
                                ) = match result2 {
                                    Ok(_) => {
                                        (
                                            0i32,
                                            0i32,
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                        )
                                    }
                                    Err(e) => {
                                        let (result9_0, result9_1, result9_2, result9_3) = match e {
                                            SchemaError::SchemaInvalid(e) => {
                                                let vec3 = e;
                                                let ptr3 = vec3.as_ptr().cast::<u8>();
                                                let len3 = vec3.len();
                                                (0i32, ptr3.cast_mut(), len3 as *mut u8, 0usize)
                                            }
                                            SchemaError::Sql(e) => {
                                                use super::super::super::super::__with_name0::Error as V7;
                                                let (result8_0, result8_1, result8_2) = match e {
                                                    V7::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                                    V7::Encode(e) => {
                                                        let vec4 = e;
                                                        let ptr4 = vec4.as_ptr().cast::<u8>();
                                                        let len4 = vec4.len();
                                                        (1i32, ptr4.cast_mut(), len4)
                                                    }
                                                    V7::Decode(e) => {
                                                        let vec5 = e;
                                                        let ptr5 = vec5.as_ptr().cast::<u8>();
                                                        let len5 = vec5.len();
                                                        (2i32, ptr5.cast_mut(), len5)
                                                    }
                                                    V7::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                                    V7::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                                    V7::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                                    V7::Unexpected(e) => {
                                                        let vec6 = e;
                                                        let ptr6 = vec6.as_ptr().cast::<u8>();
                                                        let len6 = vec6.len();
                                                        (6i32, ptr6.cast_mut(), len6)
                                                    }
                                                };
                                                (1i32, result8_0 as *mut u8, result8_1, result8_2)
                                            }
                                        };
                                        (1i32, result9_0, result9_1, result9_2, result9_3)
                                    }
                                };
                                unsafe extern "C" fn wit_import11(
                                    _: i32,
                                    _: i32,
                                    _: *mut u8,
                                    _: *mut u8,
                                    _: usize,
                                ) {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _task_cancel.forget();
                                wit_import11(
                                    result10_0,
                                    result10_1,
                                    result10_2,
                                    result10_3,
                                    result10_4,
                                );
                            })
                        }
                    }
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    pub unsafe fn __callback_async_check_schema(
                        event0: u32,
                        event1: u32,
                        event2: u32,
                    ) -> u32 {
                        unsafe {
                            wit_bindgen::rt::async_support::callback(
                                event0,
                                event1,
                                event2,
                            )
                        }
                    }
                    pub trait Guest {
                        #[allow(async_fn_in_trait)]
                        fn get_module_id() -> _rt::String;
                        #[allow(async_fn_in_trait)]
                        async fn list_migrations() -> _rt::Vec<MigrationId>;
                        #[allow(async_fn_in_trait)]
                        async fn apply_migration(
                            id: MigrationId,
                            executor: QueryExecutor<'_>,
                        ) -> Result<(), Error>;
                        #[allow(async_fn_in_trait)]
                        async fn check_schema(
                            executor: QueryExecutor<'_>,
                        ) -> Result<(), SchemaError>;
                    }
                    #[doc(hidden)]
                    pub(crate) use __export_wasmledger_module_migrations_cabi;
                    #[repr(align(8))]
                    struct _RetArea(
                        [::core::mem::MaybeUninit<
                            u8,
                        >; 2 * ::core::mem::size_of::<*const u8>()],
                    );
                    static mut _RET_AREA: _RetArea = _RetArea(
                        [::core::mem::MaybeUninit::uninit(); 2
                            * ::core::mem::size_of::<*const u8>()],
                    );
                }
            }
        }
    }
    mod _rt {
        #![allow(dead_code, clippy::all)]
        pub use alloc_crate::string::String;
        pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
            if size == 0 {
                return;
            }
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                alloc::dealloc(ptr, layout);
            }
        }
        pub use alloc_crate::alloc;
        pub use alloc_crate::vec::Vec;
        pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
            if true {
                String::from_utf8(bytes).unwrap()
            } else {
                unsafe { String::from_utf8_unchecked(bytes) }
            }
        }
        extern crate alloc as alloc_crate;
    }
    #[doc(inline)]
    pub(crate) use __export_core_impl as export;
    #[inline(never)]
    #[doc(hidden)]
    pub fn __link_custom_section_describing_imports() {
        wit_bindgen::rt::maybe_link_cabi_realloc();
    }
    const _: &[u8] = b"package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  begin-transaction: async func() -> result<transaction, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}";
    const _: &[u8] = b"package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
    const _: &[u8] = b"package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// no rows returned by a query that expected to return at least one row.\n    row-not-found,\n\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results {\n    row-count: func() -> u64;\n  }\n}";
    const _: &[u8] = b"package wasmledger:sql;\n\ninterface codecs {\n  use util-types.{error};\n\n  type push-result = result<_, error>;\n\n  type row-index = u64;\n\n  variant column-index {\n    index(u64),\n    column-name(string)\n  }\n\n  type value-position = tuple<row-index, column-index>;\n}";
    const _: &[u8] = b"package wasmledger:sql-postgres;\n\ninterface postgres-codecs {\n  use wasmledger:sql/query-types.{sql-arguments, query-results};\n  use wasmledger:sql/util-types.{error};\n  use wasmledger:sql/codecs.{push-result, value-position};\n\n  push-int16: func(value: option<s16>, to: borrow<sql-arguments>) -> push-result;\n  get-int16: func(%result: borrow<query-results>, position: value-position) -> result<option<s16>, error>;\n\n  push-int32: func(value: option<s32>, to: borrow<sql-arguments>) -> push-result;\n  get-int32: func(%result: borrow<query-results>, position: value-position) -> result<option<s32>, error>;\n\n  push-int64: func(value: option<s64>, to: borrow<sql-arguments>) -> push-result;\n  get-int64: func(%result: borrow<query-results>, position: value-position) -> result<option<s64>, error>;\n\n  push-float32: func(value: option<f32>, to: borrow<sql-arguments>) -> push-result;\n  get-float32: func(%result: borrow<query-results>, position: value-position) -> result<option<f32>, error>;\n\n  push-float64: func(value: option<f64>, to: borrow<sql-arguments>) -> push-result;\n  get-float64: func(%result: borrow<query-results>, position: value-position) -> result<option<f64>, error>;\n\n  push-string: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-string: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n  push-bool: func(value: option<bool>, to: borrow<sql-arguments>) -> push-result;\n  get-bool: func(%result: borrow<query-results>, position: value-position) -> result<option<bool>, error>;\n\n  push-json: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-json: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n  type uuid = string;\n\n  push-uuid: func(value: option<uuid>, to: borrow<sql-arguments>) -> push-result;\n  get-uuid: func(%result: borrow<query-results>, position: value-position) -> result<option<uuid>, error>;\n\n\n  // variant pg-value {\n  //   /// SQL: NULL\n  //   null,\n\n  //   /// SQL: BIGINT, INT8\n  //   int64(s64),\n  //   /// SQL: BIGINT[], INT8[]\n  //   int64-array(list<s64>),\n\n  //   /// SQL: INTEGER, INT, INT4\n  //   int32(s32),\n  //   /// SQL: INTEGER[], INT4[]\n  //   int32-array(list<s32>),\n\n  //   /// SQL: SMALLINT, INT2\n  //   int2(s16),\n  //   /// SQL: SMALLINT[], INT2[]\n  //   int2-array(list<s16>),\n\n  //   /// SQL: DOUBLE PRECISION, FLOAT8\n  //   float8(hashable-f64),\n  //   /// SQL: DOUBLE PRECISION[], FLOAT8[]\n  //   float8-array(list<hashable-f64>),\n\n  //   /// SQL: REAL, FLOAT4\n  //   float4(hashable-f32),\n  //   /// SQL: REAL[], FLOAT4[]\n  //   float4-array(list<hashable-f32>),\n\n  //   /// SQL: BOOLEAN, BOOL\n  //   %bool(bool),\n  //   /// SQL: BOOLEAN[], BOOL[]\n  //   %bool-array(list<bool>),\n\n  //   /// SQL: NUMERIC, DECIMAL\n  //   numeric(numeric),\n  //   /// SQL: NUMERIC[], DECIMAL[]\n  //   numeric-array(list<numeric>),\n\n  //   /// SQL: BIT(n)\n  //   bit(list<u8>),\n  //   /// SQL: BIT(n)[]\n  //   bit-array(list<list<u8>>),\n\n  //   /// SQL: VARBIT(n)\n  //   varbit(list<u8>),\n  //   /// SQL: BIT VARYING[], VARBIT[]\n  //   varbit-array(list<list<u8>>),\n\n  //   /// SQL: BYTEA\n  //   bytea(list<u8>),\n  //   /// SQL: BYTEA[]\n  //   bytea-array(list<list<u8>>),\n\n  //   /// SQL: CHAR(n), CHARACTER(n)\n  //   %char(string),\n  //   /// SQL: CHAR(n)[]\n  //   %char-array(list<string>),\n\n  //   /// SQL: VARCHAR(n), CHARACTER VARYING(n)\n  //   varchar(string),\n  //   /// SQL: VARCHAR(n)[]\n  //   varchar-array(list<string>),\n\n  //   // Networking\n  //   /// SQL: CIDR\n  //   cidr(string),\n  //   /// SQL: CIDR[]\n  //   cidr-array(list<string>),\n\n  //   /// SQL: INET\n  //   inet(string),\n  //   /// SQL: INET[]\n  //   inet-array(list<string>),\n\n  //   /// SQL: MACADDR (EUI-48)\n  //   macaddr(mac-address-eui48),\n  //   /// SQL: MACADDR[]\n  //   macaddr-array(list<mac-address-eui48>),\n\n  //   /// SQL: MACADDR8 (EUI-64, deprecated)\n  //   macaddr8(mac-address-eui64),\n  //   /// SQL: MACADDR8[]\n  //   macaddr8-array(list<mac-address-eui64>),\n\n  //   // Date-time\n  //   /// SQL: DATE\n  //   date(date),\n  //   /// SQL: DATE[]\n  //   date-array(list<date>),\n\n  //   /// SQL: INTERVAL\n  //   interval(interval),\n  //   /// SQL: INTERVAL[]\n  //   interval-array(list<interval>),\n\n  //   /// SQL: TIME WITHOUT TIME ZONE\n  //   time(time),\n  //   /// SQL: TIME[]\n  //   time-array(list<time>),\n\n  //   /// SQL: TIME WITH TIME ZONE\n  //   time-tz(time-tz),\n  //   /// SQL: TIMETZ[]\n  //   time-tz-array(list<time-tz>),\n\n  //   /// SQL: TIMESTAMP WITHOUT TIME ZONE\n  //   timestamp(timestamp),\n  //   /// SQL: TIMESTAMP[]\n  //   timestamp-array(list<timestamp>),\n\n  //   /// SQL: TIMESTAMP WITH TIME ZONE, TIMESTAMPTZ\n  //   timestamp-tz(timestamp-tz),\n  //   /// SQL: TIMESTAMPTZ[]\n  //   timestamp-tz-array(list<timestamp-tz>),\n\n  //   // JSON\n  //   /// SQL: JSON\n  //   json(string),\n  //   /// SQL: JSON[]\n  //   json-array(list<string>),\n\n  //   /// SQL: JSONB\n  //   jsonb(string),\n  //   /// SQL: JSONB[]\n  //   jsonb-array(list<string>),\n\n  //   /// SQL: MONEY (internal fixed-point type)\n  //   money(numeric),\n  //   /// SQL: MONEY[]\n  //   money-array(list<numeric>),\n\n  //   // Text\n  //   /// SQL: NAME (system identifier type)\n  //   name(string),\n  //   /// SQL: NAME[]\n  //   name-array(list<string>),\n\n  //   /// SQL: TEXT\n  //   text(string),\n  //   /// SQL: TEXT[]\n  //   text-array(list<string>),\n\n  //   /// SQL: XML\n  //   xml(string),\n  //   /// SQL: XML[]\n  //   xml-array(list<string>),\n\n  //   // UUIDs\n  //   /// SQL: UUID\n  //   uuid(string),\n  //   /// SQL: UUID[]\n  //   uuid-array(list<string>),\n\n  //   // Containers\n  //   /// SQL: HSTORE (extension)\n  //   hstore(list<tuple<string, option<string>>>),\n  // }\n}";
    const _: &[u8] = b"package wasmledger:sql-postgres;\n\ninterface postgres-codecs-ext {\n  use wasmledger:sql/query-types.{sql-arguments, query-results};\n  use wasmledger:sql/util-types.{error};\n  use wasmledger:sql/codecs.{push-result, value-position};\n\n  type hstore = list<tuple<string, option<string>>>;\n\n  push-hstore: func(value: option<hstore>, to: borrow<sql-arguments>) -> push-result;\n  get-hstore: func(%result: borrow<query-results>, position: value-position) -> result<option<hstore>, error>;\n}";
    const _: &[u8] = b"package wasmledger:module;\n\ninterface migrations {\n  use wasmledger:sql/query.{query-executor};\n  use wasmledger:sql/util-types.{error};\n\n  type migration-id = string;\n\n  variant schema-error {\n    schema-invalid(string),\n    sql(error)\n  }\n\n  get-module-id: func() -> string;\n\n  list-migrations: async func() -> list<migration-id>;\n  apply-migration: async func(id: migration-id, executor: query-executor) -> result<_, error>;\n\n  check-schema: async func(executor: query-executor) -> result<_, schema-error>;\n}";
    const _: &[u8] = b"package wasmledger:module;\n\ninterface module {\n  get-module-id: func() -> string;\n}";
    const _: &[u8] = b"package wasmledger:core;\n\nworld core {\n  import wasmledger:sql/transaction;\n  import wasmledger:sql/query;\n  import wasmledger:sql-postgres/postgres-codecs;\n  \n  export wasmledger:module/migrations;\n}";
    use super::BindingsImpl;
    const _: () = {
        #[unsafe(export_name = "wasmledger:module/migrations#get-module-id")]
        unsafe extern "C" fn export_get_module_id() -> *mut u8 {
            unsafe {
                self::exports::wasmledger::module::migrations::_export_get_module_id_cabi::<
                    BindingsImpl,
                >()
            }
        }
        #[unsafe(export_name = "cabi_post_wasmledger:module/migrations#get-module-id")]
        unsafe extern "C" fn _post_return_get_module_id(arg0: *mut u8) {
            unsafe {
                self::exports::wasmledger::module::migrations::__post_return_get_module_id::<
                    BindingsImpl,
                >(arg0)
            }
        }
        #[unsafe(
            export_name = "[async-lift]wasmledger:module/migrations#[async]list-migrations"
        )]
        unsafe extern "C" fn export_async_list_migrations() -> i32 {
            unsafe {
                self::exports::wasmledger::module::migrations::_export_async_list_migrations_cabi::<
                    BindingsImpl,
                >()
            }
        }
        #[unsafe(
            export_name = "[callback][async-lift]wasmledger:module/migrations#[async]list-migrations"
        )]
        unsafe extern "C" fn _callback_async_list_migrations(
            event0: u32,
            event1: u32,
            event2: u32,
        ) -> u32 {
            unsafe {
                self::exports::wasmledger::module::migrations::__callback_async_list_migrations(
                    event0,
                    event1,
                    event2,
                )
            }
        }
        #[unsafe(
            export_name = "[async-lift]wasmledger:module/migrations#[async]apply-migration"
        )]
        unsafe extern "C" fn export_async_apply_migration(
            arg0: *mut u8,
            arg1: usize,
            arg2: i32,
            arg3: i32,
        ) -> i32 {
            unsafe {
                self::exports::wasmledger::module::migrations::_export_async_apply_migration_cabi::<
                    BindingsImpl,
                >(arg0, arg1, arg2, arg3)
            }
        }
        #[unsafe(
            export_name = "[callback][async-lift]wasmledger:module/migrations#[async]apply-migration"
        )]
        unsafe extern "C" fn _callback_async_apply_migration(
            event0: u32,
            event1: u32,
            event2: u32,
        ) -> u32 {
            unsafe {
                self::exports::wasmledger::module::migrations::__callback_async_apply_migration(
                    event0,
                    event1,
                    event2,
                )
            }
        }
        #[unsafe(
            export_name = "[async-lift]wasmledger:module/migrations#[async]check-schema"
        )]
        unsafe extern "C" fn export_async_check_schema(arg0: i32, arg1: i32) -> i32 {
            unsafe {
                self::exports::wasmledger::module::migrations::_export_async_check_schema_cabi::<
                    BindingsImpl,
                >(arg0, arg1)
            }
        }
        #[unsafe(
            export_name = "[callback][async-lift]wasmledger:module/migrations#[async]check-schema"
        )]
        unsafe extern "C" fn _callback_async_check_schema(
            event0: u32,
            event1: u32,
            event2: u32,
        ) -> u32 {
            unsafe {
                self::exports::wasmledger::module::migrations::__callback_async_check_schema(
                    event0,
                    event1,
                    event2,
                )
            }
        }
    };
}
mod migrations {
    use std::{collections::HashMap, sync::LazyLock};
    use wasmledger_migrations_macro::load_migrations;
    use wasmledger_sql_client::base::bindings::wasmledger::sql::{
        query::QueryExecutor, query_types::SqlString,
    };
    use wasmledger_utils::{
        impl_expectation_to_schema_error, impl_migrations_guest_partially,
        impl_sql_to_schema_error,
        migrations::{
            ColumnExpectationError, ColumnInfo, expect_column, load_table_schema,
        },
    };
    use crate::{
        BindingsImpl,
        bindings::exports::wasmledger::module::migrations::{
            Error, MigrationId, SchemaError,
        },
    };
    use crate::bindings::exports::wasmledger::module as module_bindings;
    static MIGRATIONS: LazyLock<(Vec<MigrationId>, HashMap<MigrationId, SqlString>)> = LazyLock::new(||
    {
        let ids: ::std::vec::Vec<::std::string::String> = <[_]>::into_vec(
            ::alloc::boxed::box_new([::std::string::String::from("001_base")]),
        );
        let queries: ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        > = {
            let mut m = ::std::collections::HashMap::new();
            m.insert(
                ::std::string::String::from("001_base"),
                ::std::string::String::from(
                    "CREATE TABLE\n  accounts (\n    id TEXT NOT NULL, -- account identifier (external, user-defined)\n    bucket TEXT NOT NULL, -- bucket / balance / color\n    asset TEXT NOT NULL, -- currency + precision, e.g. USD/2\n    in_volume BIGINT NOT NULL DEFAULT 0,\n    out_volume BIGINT NOT NULL DEFAULT 0,\n    last_transfer_id BIGINT NULL, -- last applied transfer (snowflake)\n    PRIMARY KEY (id, bucket, asset)\n  );\n\nCREATE TABLE\n  transfers (\n    id BIGINT NOT NULL, -- snowflake, generated by application\n    src TEXT NOT NULL, -- source account id\n    dst TEXT NOT NULL, -- destination account id\n    src_bucket TEXT NULL, -- NULL = ephemeral\n    dst_bucket TEXT NULL, -- NULL = ephemeral\n    asset TEXT NOT NULL,\n    amount BIGINT NOT NULL, -- minimal units\n    -- snapshots AFTER transfer\n    src_bucket_in_volume BIGINT NULL, -- NULL = ephemeral\n    src_bucket_out_volume BIGINT NULL, -- NULL = ephemeral\n    dst_bucket_in_volume BIGINT NULL, -- NULL = ephemeral\n    dst_bucket_out_volume BIGINT NULL, -- NULL = ephemeral\n    created_at TIMESTAMP NOT NULL DEFAULT now ()\n  );\n\nCREATE INDEX idx_transfers_id ON transfers (id);",
                ),
            );
            m
        };
        (ids, queries)
    });
    impl crate::bindings::exports::wasmledger::module::migrations::Guest
    for BindingsImpl {
        fn get_module_id() -> String {
            "wasmledger_core".to_string()
        }
        async fn list_migrations() -> Vec<module_bindings::migrations::MigrationId> {
            MIGRATIONS.0.clone()
        }
        async fn apply_migration(
            id: module_bindings::migrations::MigrationId,
            executor: wasmledger_sql_client::base::bindings::wasmledger::sql::query::QueryExecutor<
                '_,
            >,
        ) -> Result<
            (),
            wasmledger_sql_client::base::bindings::wasmledger::sql::util_types::Error,
        > {
            let query = MIGRATIONS
                .1
                .get(&id)
                .expect(
                    &::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Migration {0} not found", id))
                    }),
                );
            wasmledger_sql_client::base::bindings::wasmledger::sql::query::execute_raw(
                    query.to_string(),
                    executor,
                )
                .await
        }
        async fn check_schema(executor: QueryExecutor<'_>) -> Result<(), SchemaError> {
            let t_accounts = load_table_schema(executor, "accounts").await?;
            let t_transfers = load_table_schema(executor, "transfers").await?;
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
    impl From<ColumnExpectationError> for SchemaError {
        fn from(e: ColumnExpectationError) -> Self {
            <SchemaError>::SchemaInvalid(e.message)
        }
    }
    impl From<Error> for SchemaError {
        fn from(e: Error) -> Self {
            <SchemaError>::Sql(e)
        }
    }
}
pub struct BindingsImpl;

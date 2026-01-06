#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::sync::Arc;
use axum::{Router, routing::post};
mod api {
    pub mod execute {
        use std::sync::Arc;
        use axum::{extract::State, Json, extract::Multipart};
        use crate::{app_state::AppState, execution::{FunctionExecutor, ExecutionError}};
        /// Handler for POST /execute endpoint
        ///
        /// Accepts multipart/form-data with the following fields:
        /// - `wasm` (required): WASM function bytes
        /// - `input` (optional): JSON input parameters for the function (future use)
        ///
        /// Returns JSON response with function output and execution metadata.
        pub async fn execute_function_handler(
            State(app_state): State<Arc<AppState>>,
            mut multipart: Multipart,
        ) -> Result<Json<crate::execution::ExecutionResult>, ExecutionError> {
            let mut wasm_bytes: Option<Vec<u8>> = None;
            let mut _input: Option<String> = None;
            while let Some(field) = multipart
                .next_field()
                .await
                .map_err(|e| {
                    ExecutionError::ValidationError(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("Failed to read multipart field: {0}", e),
                            )
                        }),
                    )
                })?
            {
                let name = field.name().unwrap_or("").to_string();
                match name.as_str() {
                    "wasm" => {
                        let data = field
                            .bytes()
                            .await
                            .map_err(|e| {
                                ExecutionError::ValidationError(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("Failed to read WASM bytes: {0}", e),
                                        )
                                    }),
                                )
                            })?;
                        wasm_bytes = Some(data.to_vec());
                    }
                    "input" => {
                        let data = field
                            .text()
                            .await
                            .map_err(|e| {
                                ExecutionError::ValidationError(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("Failed to read input: {0}", e),
                                        )
                                    }),
                                )
                            })?;
                        _input = Some(data);
                    }
                    _ => {}
                }
            }
            let wasm_bytes = wasm_bytes
                .ok_or_else(|| {
                    ExecutionError::ValidationError(
                        "Missing required field 'wasm'".to_string(),
                    )
                })?;
            let executor = FunctionExecutor::new(app_state.clone());
            let result = executor
                .execute_function(wasm_bytes, &app_state.config.execution)
                .await?;
            Ok(Json(result))
        }
    }
    pub use execute::execute_function_handler;
}
mod config {
    use serde::Deserialize;
    use std::path::PathBuf;
    use std::time::Duration;
    use crate::plugin::registry::PluginEntry;
    /// Configuration for function execution (limits and timeouts)
    #[serde(default)]
    pub struct ExecutionConfig {
        /// Fuel limit (computational budget)
        /// Default: 1,000,000,000
        pub fuel_limit: Option<u64>,
        /// Memory limit in bytes
        /// Default: 100 MB
        pub memory_limit_bytes: Option<usize>,
        /// Execution timeout in seconds
        /// Default: 30 seconds
        pub timeout_seconds: u64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ExecutionConfig {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ExecutionConfig",
                "fuel_limit",
                &self.fuel_limit,
                "memory_limit_bytes",
                &self.memory_limit_bytes,
                "timeout_seconds",
                &&self.timeout_seconds,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ExecutionConfig {
        #[inline]
        fn clone(&self) -> ExecutionConfig {
            ExecutionConfig {
                fuel_limit: ::core::clone::Clone::clone(&self.fuel_limit),
                memory_limit_bytes: ::core::clone::Clone::clone(
                    &self.memory_limit_bytes,
                ),
                timeout_seconds: ::core::clone::Clone::clone(&self.timeout_seconds),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ExecutionConfig
        where
            ExecutionConfig: _serde::__private228::Default,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "fuel_limit" => _serde::__private228::Ok(__Field::__field0),
                            "memory_limit_bytes" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            "timeout_seconds" => {
                                _serde::__private228::Ok(__Field::__field2)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"fuel_limit" => _serde::__private228::Ok(__Field::__field0),
                            b"memory_limit_bytes" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            b"timeout_seconds" => {
                                _serde::__private228::Ok(__Field::__field2)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de>
                where
                    ExecutionConfig: _serde::__private228::Default,
                {
                    marker: _serde::__private228::PhantomData<ExecutionConfig>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de>
                where
                    ExecutionConfig: _serde::__private228::Default,
                {
                    type Value = ExecutionConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct ExecutionConfig",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __default: Self::Value = _serde::__private228::Default::default();
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<u64>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => __default.fuel_limit,
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<usize>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => __default.memory_limit_bytes,
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            u64,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => __default.timeout_seconds,
                        };
                        _serde::__private228::Ok(ExecutionConfig {
                            fuel_limit: __field0,
                            memory_limit_bytes: __field1,
                            timeout_seconds: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<Option<u64>> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<Option<usize>> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<u64> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "fuel_limit",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<u64>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "memory_limit_bytes",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<usize>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "timeout_seconds",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __default: Self::Value = _serde::__private228::Default::default();
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => __default.fuel_limit,
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => __default.memory_limit_bytes,
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => __default.timeout_seconds,
                        };
                        _serde::__private228::Ok(ExecutionConfig {
                            fuel_limit: __field0,
                            memory_limit_bytes: __field1,
                            timeout_seconds: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "fuel_limit",
                    "memory_limit_bytes",
                    "timeout_seconds",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ExecutionConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<ExecutionConfig>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for ExecutionConfig {
        fn default() -> Self {
            Self {
                fuel_limit: Some(1_000_000_000),
                memory_limit_bytes: Some(100 * 1024 * 1024),
                timeout_seconds: 30,
            }
        }
    }
    impl ExecutionConfig {
        /// Get timeout as Duration
        pub fn timeout(&self) -> Duration {
            Duration::from_secs(self.timeout_seconds)
        }
    }
    /// Main host configuration
    #[serde(default)]
    pub struct HostConfig {
        /// Plugins to load at startup
        pub plugins: Vec<PluginEntry>,
        /// Execution configuration for functions
        pub execution: ExecutionConfig,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for HostConfig {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "HostConfig",
                "plugins",
                &self.plugins,
                "execution",
                &&self.execution,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for HostConfig
        where
            HostConfig: _serde::__private228::Default,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "plugins" => _serde::__private228::Ok(__Field::__field0),
                            "execution" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"plugins" => _serde::__private228::Ok(__Field::__field0),
                            b"execution" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de>
                where
                    HostConfig: _serde::__private228::Default,
                {
                    marker: _serde::__private228::PhantomData<HostConfig>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de>
                where
                    HostConfig: _serde::__private228::Default,
                {
                    type Value = HostConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct HostConfig",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __default: Self::Value = _serde::__private228::Default::default();
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<PluginEntry>,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => __default.plugins,
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            ExecutionConfig,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => __default.execution,
                        };
                        _serde::__private228::Ok(HostConfig {
                            plugins: __field0,
                            execution: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<
                            Vec<PluginEntry>,
                        > = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<
                            ExecutionConfig,
                        > = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "plugins",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<PluginEntry>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "execution",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<
                                            ExecutionConfig,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __default: Self::Value = _serde::__private228::Default::default();
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => __default.plugins,
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => __default.execution,
                        };
                        _serde::__private228::Ok(HostConfig {
                            plugins: __field0,
                            execution: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["plugins", "execution"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "HostConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<HostConfig>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for HostConfig {
        fn default() -> Self {
            Self {
                plugins: Vec::new(),
                execution: ExecutionConfig::default(),
            }
        }
    }
    impl HostConfig {
        /// Load configuration from YAML file
        ///
        /// Reads from `config.yaml` in the current directory by default,
        /// or from the path specified in the `CONFIG_PATH` environment variable.
        pub fn load() -> anyhow::Result<Self> {
            let config_path = std::env::var("CONFIG_PATH")
                .unwrap_or_else(|_| "config.yaml".to_string());
            let path = PathBuf::from(&config_path);
            if !path.exists() {
                return Ok(Self::default());
            }
            let config_content = std::fs::read_to_string(&path)
                .map_err(|e| ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Failed to read config file {0}: {1}",
                                config_path,
                                e,
                            ),
                        )
                    }),
                ))?;
            let config: Self = serde_yaml::from_str(&config_content)
                .map_err(|e| ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("Failed to parse YAML config: {0}", e),
                        )
                    }),
                ))?;
            Ok(config)
        }
    }
}
mod app_state {
    use std::sync::Arc;
    use wasmtime::Engine;
    use crate::{
        config::HostConfig, engine::create_engine, plugin::registry::PluginRegistry,
    };
    /// Shared application state
    pub struct AppState {
        /// Wasmtime engine (shared across all requests)
        pub engine: Engine,
        /// Plugin registry (loaded at startup)
        pub plugin_registry: Arc<PluginRegistry>,
        /// Host configuration (plugins, execution limits, etc.)
        pub config: Arc<HostConfig>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AppState {
        #[inline]
        fn clone(&self) -> AppState {
            AppState {
                engine: ::core::clone::Clone::clone(&self.engine),
                plugin_registry: ::core::clone::Clone::clone(&self.plugin_registry),
                config: ::core::clone::Clone::clone(&self.config),
            }
        }
    }
    impl AppState {
        pub async fn initialize() -> anyhow::Result<Self> {
            let engine = create_engine()?;
            let config = HostConfig::load()?;
            let plugin_registry = PluginRegistry::load_from_config(&engine, &config)
                .await?;
            Ok(Self {
                engine,
                plugin_registry: Arc::new(plugin_registry),
                config: Arc::new(config),
            })
        }
    }
}
mod capabilities {
    use crate::engine::CoreState;
    use wasmtime::component::Linker;
    pub(crate) mod sql {
        use std::{env, str::FromStr};
        use wasmledger_sql::{core::bindings::BindingsImplState, sqldb::{SqlDB, sqlx}};
        pub(crate) async fn create_sql_state() -> anyhow::Result<BindingsImplState> {
            let pool_options = sqlx::postgres::PgPoolOptions::default();
            let connect_options = {
                let env_pgurl = env::var("PGURL");
                let opts = match env_pgurl {
                    Ok(url) => sqlx::postgres::PgConnectOptions::from_str(&url)?,
                    Err(env::VarError::NotPresent) => {
                        sqlx::postgres::PgConnectOptions::default()
                    }
                    Err(e) => return anyhow::Result::Err(e.into()),
                };
                opts
            };
            let pool = pool_options.connect_with(connect_options).await?;
            Ok(BindingsImplState::new(SqlDB::new(pool)))
        }
    }
    pub(crate) mod wasi {
        use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView};
        pub(crate) struct WasiState {
            ctx: WasiCtx,
            table: ResourceTable,
        }
        pub(crate) fn create_wasi_state() -> WasiState {
            let ctx = WasiCtxBuilder::new().inherit_stdout().inherit_stderr().build();
            let table = ResourceTable::new();
            WasiState { ctx, table }
        }
        impl WasiState {
            pub fn to_view(&mut self) -> WasiCtxView<'_> {
                WasiCtxView {
                    ctx: &mut self.ctx,
                    table: &mut self.table,
                }
            }
        }
    }
    /// Add capabilities to the linker
    pub fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
        {
            ::std::io::_print(format_args!("Add to linker\n"));
        };
        wasmtime_wasi::p2::add_to_linker_async(linker)?;
        wasmledger_sql::core::bindings::Host_::add_to_linker::<
            CoreState,
            wasmledger_sql::core::bindings::BindingsImplState,
        >(linker, |s| &mut s.sql)?;
        wasmledger_sql::postgres::bindings::CodecsPostgres::add_to_linker::<
            CoreState,
            wasmledger_sql::core::bindings::BindingsImplState,
        >(linker, |s| &mut s.sql)?;
        {
            ::std::io::_print(format_args!("Add to linker success\n"));
        };
        Ok(())
    }
}
mod engine {
    use wasmtime::{Config, Engine};
    use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
    use crate::capabilities::{
        sql::create_sql_state, wasi::{WasiState, create_wasi_state},
    };
    /// State holding all host capabilities available to WASM components
    pub struct CoreState {
        pub sql: wasmledger_sql::core::bindings::BindingsImplState,
        pub wasi: WasiState,
    }
    /// Create a configured Wasmtime engine with component model support
    pub fn create_engine() -> anyhow::Result<Engine> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);
        config.wasm_component_model_async(true);
        config.consume_fuel(true);
        config.epoch_interruption(true);
        config.memory_init_cow(true);
        Ok(Engine::new(&config)?)
    }
    pub async fn create_core_state() -> anyhow::Result<CoreState> {
        let sql = create_sql_state().await?;
        let wasi = create_wasi_state();
        Ok(CoreState { sql, wasi })
    }
    impl WasiView for CoreState {
        fn ctx(&mut self) -> WasiCtxView<'_> {
            self.wasi.to_view()
        }
    }
}
mod execution {
    pub mod error {
        use axum::{
            http::StatusCode, response::{IntoResponse, Response},
            Json,
        };
        use serde_json::json;
        use std::time::Duration;
        /// Errors that can occur during module execution
        pub enum ExecutionError {
            #[error("Module compilation failed: {0}")]
            CompilationError(String),
            #[error("Module validation failed: {0}")]
            ValidationError(String),
            #[error("Module instantiation failed: {0}")]
            InstantiationError(String),
            #[error("Module execution trapped: {0}")]
            Trap(String),
            #[error("Module execution timed out after {0:?}")]
            Timeout(Duration),
            #[error("Fuel exhausted (limit: {0})")]
            FuelExhausted(u64),
            #[error("Dependency not found: {0}")]
            DependencyNotFound(String),
            #[error("Internal error: {0}")]
            Internal(#[from] anyhow::Error),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ExecutionError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ExecutionError::CompilationError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "CompilationError",
                            &__self_0,
                        )
                    }
                    ExecutionError::ValidationError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ValidationError",
                            &__self_0,
                        )
                    }
                    ExecutionError::InstantiationError(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "InstantiationError",
                            &__self_0,
                        )
                    }
                    ExecutionError::Trap(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Trap",
                            &__self_0,
                        )
                    }
                    ExecutionError::Timeout(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Timeout",
                            &__self_0,
                        )
                    }
                    ExecutionError::FuelExhausted(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "FuelExhausted",
                            &__self_0,
                        )
                    }
                    ExecutionError::DependencyNotFound(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "DependencyNotFound",
                            &__self_0,
                        )
                    }
                    ExecutionError::Internal(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Internal",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl ::thiserror::__private17::Error for ExecutionError {
            fn source(
                &self,
            ) -> ::core::option::Option<
                &(dyn ::thiserror::__private17::Error + 'static),
            > {
                use ::thiserror::__private17::AsDynError as _;
                #[allow(deprecated)]
                match self {
                    ExecutionError::CompilationError { .. } => {
                        ::core::option::Option::None
                    }
                    ExecutionError::ValidationError { .. } => {
                        ::core::option::Option::None
                    }
                    ExecutionError::InstantiationError { .. } => {
                        ::core::option::Option::None
                    }
                    ExecutionError::Trap { .. } => ::core::option::Option::None,
                    ExecutionError::Timeout { .. } => ::core::option::Option::None,
                    ExecutionError::FuelExhausted { .. } => ::core::option::Option::None,
                    ExecutionError::DependencyNotFound { .. } => {
                        ::core::option::Option::None
                    }
                    ExecutionError::Internal { 0: source, .. } => {
                        ::core::option::Option::Some(source.as_dyn_error())
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        #[automatically_derived]
        impl ::core::fmt::Display for ExecutionError {
            fn fmt(
                &self,
                __formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                use ::thiserror::__private17::AsDisplay as _;
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    ExecutionError::CompilationError(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("Module compilation failed: {0}", __display0),
                                    )
                            }
                        }
                    }
                    ExecutionError::ValidationError(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("Module validation failed: {0}", __display0),
                                    )
                            }
                        }
                    }
                    ExecutionError::InstantiationError(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("Module instantiation failed: {0}", __display0),
                                    )
                            }
                        }
                    }
                    ExecutionError::Trap(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("Module execution trapped: {0}", __display0),
                                    )
                            }
                        }
                    }
                    ExecutionError::Timeout(_0) => {
                        match (_0,) {
                            (__field0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!(
                                            "Module execution timed out after {0:?}",
                                            __field0,
                                        ),
                                    )
                            }
                        }
                    }
                    ExecutionError::FuelExhausted(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("Fuel exhausted (limit: {0})", __display0),
                                    )
                            }
                        }
                    }
                    ExecutionError::DependencyNotFound(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(
                                        format_args!("Dependency not found: {0}", __display0),
                                    )
                            }
                        }
                    }
                    ExecutionError::Internal(_0) => {
                        match (_0.as_display(),) {
                            (__display0,) => {
                                __formatter
                                    .write_fmt(format_args!("Internal error: {0}", __display0))
                            }
                        }
                    }
                }
            }
        }
        #[allow(
            deprecated,
            unused_qualifications,
            clippy::elidable_lifetime_names,
            clippy::needless_lifetimes,
        )]
        #[automatically_derived]
        impl ::core::convert::From<anyhow::Error> for ExecutionError {
            fn from(source: anyhow::Error) -> Self {
                ExecutionError::Internal {
                    0: source,
                }
            }
        }
        impl ExecutionError {
            /// Get the appropriate HTTP status code for this error
            pub fn status_code(&self) -> StatusCode {
                match self {
                    Self::CompilationError(_) | Self::ValidationError(_) => {
                        StatusCode::BAD_REQUEST
                    }
                    Self::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
                    Self::FuelExhausted(_) => StatusCode::TOO_MANY_REQUESTS,
                    Self::DependencyNotFound(_) => StatusCode::BAD_REQUEST,
                    Self::InstantiationError(_) | Self::Trap(_) => {
                        StatusCode::UNPROCESSABLE_ENTITY
                    }
                    Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
        }
        impl IntoResponse for ExecutionError {
            fn into_response(self) -> Response {
                let status = self.status_code();
                let message = self.to_string();
                (
                    status,
                    Json(
                        ::serde_json::Value::Object({
                            let mut object = ::serde_json::Map::new();
                            let _ = object
                                .insert(
                                    ("error").into(),
                                    ::serde_json::to_value(&message).unwrap(),
                                );
                            object
                        }),
                    ),
                )
                    .into_response()
            }
        }
    }
    pub mod executor {
        use std::sync::Arc;
        use wasmtime::{Store, component::{Component, Linker}};
        use serde_json::Value;
        use crate::{
            app_state::AppState, engine::create_core_state, capabilities,
            config::ExecutionConfig,
        };
        use super::error::ExecutionError;
        /// Result of function execution
        pub struct ExecutionResult {
            /// Output from the function (JSON serialized)
            pub output: Value,
            /// Fuel consumed during execution
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fuel_consumed: Option<u64>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ExecutionResult {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ExecutionResult",
                    "output",
                    &self.output,
                    "fuel_consumed",
                    &&self.fuel_consumed,
                )
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ExecutionResult {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ExecutionResult",
                        false as usize + 1
                            + if Option::is_none(&self.fuel_consumed) { 0 } else { 1 },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "output",
                        &self.output,
                    )?;
                    if !Option::is_none(&self.fuel_consumed) {
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "fuel_consumed",
                            &self.fuel_consumed,
                        )?;
                    } else {
                        _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "fuel_consumed",
                        )?;
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        /// Function executor for running ledger functions
        pub struct FunctionExecutor {
            app_state: Arc<AppState>,
        }
        impl FunctionExecutor {
            /// Create a new function executor
            pub fn new(app_state: Arc<AppState>) -> Self {
                Self { app_state }
            }
            /// Execute a ledger function
            pub async fn execute_function(
                &self,
                wasm_bytes: Vec<u8>,
                config: &ExecutionConfig,
            ) -> Result<ExecutionResult, ExecutionError> {
                let component = Component::from_binary(
                        &self.app_state.engine,
                        &wasm_bytes,
                    )
                    .map_err(|e| ExecutionError::CompilationError(e.to_string()))?;
                let store_state = create_core_state()
                    .await
                    .map_err(|e| ExecutionError::Internal(e))?;
                let mut store = Store::new(&self.app_state.engine, store_state);
                let initial_fuel = if let Some(fuel) = config.fuel_limit {
                    store.set_fuel(fuel).map_err(ExecutionError::Internal)?;
                    Some(fuel)
                } else {
                    None
                };
                let mut linker = Linker::new(&self.app_state.engine);
                capabilities::add_to_linker(&mut linker)
                    .map_err(|e| ExecutionError::Internal(e))?;
                let _instance = linker
                    .instantiate_async(&mut store, &component)
                    .await
                    .map_err(|e| ExecutionError::InstantiationError(e.to_string()))?;
                let fuel_consumed = if let Some(initial) = initial_fuel {
                    let remaining = store.get_fuel().map_err(ExecutionError::Internal)?;
                    Some(initial - remaining)
                } else {
                    None
                };
                Ok(ExecutionResult {
                    output: ::serde_json::Value::Object({
                        let mut object = ::serde_json::Map::new();
                        let _ = object
                            .insert(
                                ("message").into(),
                                ::serde_json::to_value(
                                        &"Function instantiated successfully",
                                    )
                                    .unwrap(),
                            );
                        let _ = object
                            .insert(
                                ("note").into(),
                                ::serde_json::to_value(
                                        &"Actual invocation will be implemented in Phase 3",
                                    )
                                    .unwrap(),
                            );
                        object
                    }),
                    fuel_consumed,
                })
            }
        }
    }
    pub use error::ExecutionError;
    pub use executor::{FunctionExecutor, ExecutionResult};
}
mod plugin {
    use crate::engine::CoreState;
    use std::{collections::HashSet, sync::{Arc, Mutex}};
    use wasmtime::{Store, component::{Component, Instance}};
    pub mod client {
        use wasmtime::component::{Component, Linker};
        use crate::engine::CoreState;
        pub(crate) mod migrations {
            use anyhow::{Context, Ok, Result};
            use wasmtime::component::{Component, Linker};
            use crate::{
                engine::CoreState,
                plugin::{LoadedPlugin, client::PluginClient, registry::PluginRegistry},
            };
            pub(crate) mod bindings {
                #[doc(hidden)]
                pub use wasmledger_sql::core::bindings::wasmledger::sql::connection as __with_name0;
                #[doc(hidden)]
                pub use wasmledger_sql::core::bindings::wasmledger::sql::query as __with_name1;
                #[doc(hidden)]
                pub use wasmledger_sql::core::bindings::wasmledger::sql::query_types as __with_name2;
                #[doc(hidden)]
                pub use wasmledger_sql::core::bindings::wasmledger::sql::transaction as __with_name3;
                #[doc(hidden)]
                pub use wasmledger_sql::core::bindings::wasmledger::sql::util_types as __with_name4;
                /// Auto-generated bindings for a pre-instantiated version of a
                /// component which implements the world `client`.
                ///
                /// This structure is created through [`ClientPre::new`] which
                /// takes a [`InstancePre`](wasmtime::component::InstancePre) that
                /// has been created through a [`Linker`](wasmtime::component::Linker).
                ///
                /// For more information see [`Client`] as well.
                pub struct ClientPre<T: 'static> {
                    instance_pre: wasmtime::component::InstancePre<T>,
                    indices: ClientIndices,
                }
                impl<T: 'static> Clone for ClientPre<T> {
                    fn clone(&self) -> Self {
                        Self {
                            instance_pre: self.instance_pre.clone(),
                            indices: self.indices.clone(),
                        }
                    }
                }
                impl<_T: 'static> ClientPre<_T> {
                    /// Creates a new copy of `ClientPre` bindings which can then
                    /// be used to instantiate into a particular store.
                    ///
                    /// This method may fail if the component behind `instance_pre`
                    /// does not have the required exports.
                    pub fn new(
                        instance_pre: wasmtime::component::InstancePre<_T>,
                    ) -> wasmtime::Result<Self> {
                        let indices = ClientIndices::new(&instance_pre)?;
                        Ok(Self { instance_pre, indices })
                    }
                    pub fn engine(&self) -> &wasmtime::Engine {
                        self.instance_pre.engine()
                    }
                    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
                        &self.instance_pre
                    }
                    /// Instantiates a new instance of [`Client`] within the
                    /// `store` provided.
                    ///
                    /// This function will use `self` as the pre-instantiated
                    /// instance to perform instantiation. Afterwards the preloaded
                    /// indices in `self` are used to lookup all exports on the
                    /// resulting instance.
                    pub fn instantiate(
                        &self,
                        mut store: impl wasmtime::AsContextMut<Data = _T>,
                    ) -> wasmtime::Result<Client> {
                        let mut store = store.as_context_mut();
                        let instance = self.instance_pre.instantiate(&mut store)?;
                        self.indices.load(&mut store, &instance)
                    }
                }
                impl<_T: Send + 'static> ClientPre<_T> {
                    /// Same as [`Self::instantiate`], except with `async`.
                    pub async fn instantiate_async(
                        &self,
                        mut store: impl wasmtime::AsContextMut<Data = _T>,
                    ) -> wasmtime::Result<Client> {
                        let mut store = store.as_context_mut();
                        let instance = self
                            .instance_pre
                            .instantiate_async(&mut store)
                            .await?;
                        self.indices.load(&mut store, &instance)
                    }
                }
                /// Auto-generated bindings for index of the exports of
                /// `client`.
                ///
                /// This is an implementation detail of [`ClientPre`] and can
                /// be constructed if needed as well.
                ///
                /// For more information see [`Client`] as well.
                pub struct ClientIndices {
                    interface0: exports::wasmledger::plugin::migrations::GuestIndices,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ClientIndices {
                    #[inline]
                    fn clone(&self) -> ClientIndices {
                        ClientIndices {
                            interface0: ::core::clone::Clone::clone(&self.interface0),
                        }
                    }
                }
                /// Auto-generated bindings for an instance a component which
                /// implements the world `client`.
                ///
                /// This structure can be created through a number of means
                /// depending on your requirements and what you have on hand:
                ///
                /// * The most convenient way is to use
                ///   [`Client::instantiate`] which only needs a
                ///   [`Store`], [`Component`], and [`Linker`].
                ///
                /// * Alternatively you can create a [`ClientPre`] ahead of
                ///   time with a [`Component`] to front-load string lookups
                ///   of exports once instead of per-instantiation. This
                ///   method then uses [`ClientPre::instantiate`] to
                ///   create a [`Client`].
                ///
                /// * If you've instantiated the instance yourself already
                ///   then you can use [`Client::new`].
                ///
                /// These methods are all equivalent to one another and move
                /// around the tradeoff of what work is performed when.
                ///
                /// [`Store`]: wasmtime::Store
                /// [`Component`]: wasmtime::component::Component
                /// [`Linker`]: wasmtime::component::Linker
                pub struct Client {
                    interface0: exports::wasmledger::plugin::migrations::Guest,
                }
                const _: () = {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::anyhow;
                    impl ClientIndices {
                        /// Creates a new copy of `ClientIndices` bindings which can then
                        /// be used to instantiate into a particular store.
                        ///
                        /// This method may fail if the component does not have the
                        /// required exports.
                        pub fn new<_T>(
                            _instance_pre: &wasmtime::component::InstancePre<_T>,
                        ) -> wasmtime::Result<Self> {
                            let _component = _instance_pre.component();
                            let _instance_type = _instance_pre.instance_type();
                            let interface0 = exports::wasmledger::plugin::migrations::GuestIndices::new(
                                _instance_pre,
                            )?;
                            Ok(ClientIndices { interface0 })
                        }
                        /// Uses the indices stored in `self` to load an instance
                        /// of [`Client`] from the instance provided.
                        ///
                        /// Note that at this time this method will additionally
                        /// perform type-checks of all exports.
                        pub fn load(
                            &self,
                            mut store: impl wasmtime::AsContextMut,
                            instance: &wasmtime::component::Instance,
                        ) -> wasmtime::Result<Client> {
                            let _ = &mut store;
                            let _instance = instance;
                            let interface0 = self
                                .interface0
                                .load(&mut store, &_instance)?;
                            Ok(Client { interface0 })
                        }
                    }
                    impl Client {
                        /// Convenience wrapper around [`ClientPre::new`] and
                        /// [`ClientPre::instantiate`].
                        pub fn instantiate<_T>(
                            store: impl wasmtime::AsContextMut<Data = _T>,
                            component: &wasmtime::component::Component,
                            linker: &wasmtime::component::Linker<_T>,
                        ) -> wasmtime::Result<Client> {
                            let pre = linker.instantiate_pre(component)?;
                            ClientPre::new(pre)?.instantiate(store)
                        }
                        /// Convenience wrapper around [`ClientIndices::new`] and
                        /// [`ClientIndices::load`].
                        pub fn new(
                            mut store: impl wasmtime::AsContextMut,
                            instance: &wasmtime::component::Instance,
                        ) -> wasmtime::Result<Client> {
                            let indices = ClientIndices::new(
                                &instance.instance_pre(&store),
                            )?;
                            indices.load(&mut store, instance)
                        }
                        /// Convenience wrapper around [`ClientPre::new`] and
                        /// [`ClientPre::instantiate_async`].
                        pub async fn instantiate_async<_T>(
                            store: impl wasmtime::AsContextMut<Data = _T>,
                            component: &wasmtime::component::Component,
                            linker: &wasmtime::component::Linker<_T>,
                        ) -> wasmtime::Result<Client>
                        where
                            _T: Send,
                        {
                            let pre = linker.instantiate_pre(component)?;
                            ClientPre::new(pre)?.instantiate_async(store).await
                        }
                        pub fn add_to_linker<T, D>(
                            linker: &mut wasmtime::component::Linker<T>,
                            host_getter: fn(&mut T) -> D::Data<'_>,
                        ) -> wasmtime::Result<()>
                        where
                            D: __with_name2::HostWithStore + __with_name4::HostWithStore
                                + __with_name3::HostWithStore + __with_name0::HostWithStore
                                + __with_name1::HostWithStore,
                            for<'a> D::Data<
                                'a,
                            >: __with_name2::Host + __with_name4::Host
                                + __with_name3::Host + __with_name0::Host
                                + __with_name1::Host,
                            T: 'static + Send,
                        {
                            __with_name2::add_to_linker::<T, D>(linker, host_getter)?;
                            __with_name4::add_to_linker::<T, D>(linker, host_getter)?;
                            __with_name3::add_to_linker::<T, D>(linker, host_getter)?;
                            __with_name0::add_to_linker::<T, D>(linker, host_getter)?;
                            __with_name1::add_to_linker::<T, D>(linker, host_getter)?;
                            Ok(())
                        }
                        pub fn wasmledger_plugin_migrations(
                            &self,
                        ) -> &exports::wasmledger::plugin::migrations::Guest {
                            &self.interface0
                        }
                    }
                };
                pub mod wasmledger {
                    pub mod sql {
                        pub mod query_types {
                            #[allow(unused_imports)]
                            pub use super::super::super::__with_name2::*;
                        }
                        pub mod util_types {
                            #[allow(unused_imports)]
                            pub use super::super::super::__with_name4::*;
                        }
                        pub mod transaction {
                            #[allow(unused_imports)]
                            pub use super::super::super::__with_name3::*;
                        }
                        pub mod connection {
                            #[allow(unused_imports)]
                            pub use super::super::super::__with_name0::*;
                        }
                        pub mod query {
                            #[allow(unused_imports)]
                            pub use super::super::super::__with_name1::*;
                        }
                    }
                }
                pub mod exports {
                    pub mod wasmledger {
                        pub mod plugin {
                            #[allow(clippy::all)]
                            pub mod migrations {
                                #[allow(unused_imports)]
                                use wasmtime::component::__internal::{anyhow, Box};
                                pub type QueryExecutor = super::super::super::super::__with_name1::QueryExecutor;
                                const _: () = {
                                    if !(8
                                        == <QueryExecutor as wasmtime::component::ComponentType>::SIZE32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 8 == <QueryExecutor as wasmtime::component::ComponentType>::SIZE32",
                                        )
                                    }
                                    if !(4
                                        == <QueryExecutor as wasmtime::component::ComponentType>::ALIGN32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 4 == <QueryExecutor as wasmtime::component::ComponentType>::ALIGN32",
                                        )
                                    }
                                };
                                pub type Error = super::super::super::super::__with_name4::Error;
                                const _: () = {
                                    if !(12
                                        == <Error as wasmtime::component::ComponentType>::SIZE32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                                        )
                                    }
                                    if !(4
                                        == <Error as wasmtime::component::ComponentType>::ALIGN32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                                        )
                                    }
                                };
                                pub type MigrationId = wasmtime::component::__internal::String;
                                const _: () = {
                                    if !(8
                                        == <MigrationId as wasmtime::component::ComponentType>::SIZE32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 8 == <MigrationId as wasmtime::component::ComponentType>::SIZE32",
                                        )
                                    }
                                    if !(4
                                        == <MigrationId as wasmtime::component::ComponentType>::ALIGN32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 4 == <MigrationId as wasmtime::component::ComponentType>::ALIGN32",
                                        )
                                    }
                                };
                                #[component(variant)]
                                pub enum SchemaError {
                                    #[component(name = "schema-invalid")]
                                    SchemaInvalid(wasmtime::component::__internal::String),
                                    #[component(name = "sql")]
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
                                unsafe impl wasmtime::component::Lower for SchemaError {
                                    #[inline]
                                    fn linear_lower_to_flat<T>(
                                        &self,
                                        cx: &mut wasmtime::component::__internal::LowerContext<
                                            '_,
                                            T,
                                        >,
                                        ty: wasmtime::component::__internal::InterfaceType,
                                        dst: &mut core::mem::MaybeUninit<Self::Lower>,
                                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                                        let ty = match ty {
                                            wasmtime::component::__internal::InterfaceType::Variant(
                                                i,
                                            ) => &cx.types[i],
                                            _ => wasmtime::component::__internal::bad_type_info(),
                                        };
                                        match self {
                                            Self::SchemaInvalid(value) => {
                                                {
                                                    #[allow(unused_unsafe, reason = "macro-generated code")]
                                                    {
                                                        unsafe {
                                                            use ::wasmtime::MaybeUninitExt;
                                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                                            m.map(|p| &raw mut (*p).tag)
                                                        }
                                                    }
                                                }
                                                    .write(wasmtime::ValRaw::u32(0u32));
                                                unsafe {
                                                    wasmtime::component::__internal::lower_payload(
                                                        {
                                                            #[allow(unused_unsafe, reason = "macro-generated code")]
                                                            {
                                                                unsafe {
                                                                    use ::wasmtime::MaybeUninitExt;
                                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                                    m.map(|p| &raw mut (*p).payload)
                                                                }
                                                            }
                                                        },
                                                        |payload| {
                                                            #[allow(unused_unsafe, reason = "macro-generated code")]
                                                            {
                                                                unsafe {
                                                                    use ::wasmtime::MaybeUninitExt;
                                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                                    m.map(|p| &raw mut (*p).SchemaInvalid)
                                                                }
                                                            }
                                                        },
                                                        |dst| {
                                                            value
                                                                .linear_lower_to_flat(
                                                                    cx,
                                                                    ty
                                                                        .cases[0usize]
                                                                        .unwrap_or_else(
                                                                            wasmtime::component::__internal::bad_type_info,
                                                                        ),
                                                                    dst,
                                                                )
                                                        },
                                                    )
                                                }
                                            }
                                            Self::Sql(value) => {
                                                {
                                                    #[allow(unused_unsafe, reason = "macro-generated code")]
                                                    {
                                                        unsafe {
                                                            use ::wasmtime::MaybeUninitExt;
                                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                                            m.map(|p| &raw mut (*p).tag)
                                                        }
                                                    }
                                                }
                                                    .write(wasmtime::ValRaw::u32(1u32));
                                                unsafe {
                                                    wasmtime::component::__internal::lower_payload(
                                                        {
                                                            #[allow(unused_unsafe, reason = "macro-generated code")]
                                                            {
                                                                unsafe {
                                                                    use ::wasmtime::MaybeUninitExt;
                                                                    let m: &mut core::mem::MaybeUninit<_> = dst;
                                                                    m.map(|p| &raw mut (*p).payload)
                                                                }
                                                            }
                                                        },
                                                        |payload| {
                                                            #[allow(unused_unsafe, reason = "macro-generated code")]
                                                            {
                                                                unsafe {
                                                                    use ::wasmtime::MaybeUninitExt;
                                                                    let m: &mut core::mem::MaybeUninit<_> = payload;
                                                                    m.map(|p| &raw mut (*p).Sql)
                                                                }
                                                            }
                                                        },
                                                        |dst| {
                                                            value
                                                                .linear_lower_to_flat(
                                                                    cx,
                                                                    ty
                                                                        .cases[1usize]
                                                                        .unwrap_or_else(
                                                                            wasmtime::component::__internal::bad_type_info,
                                                                        ),
                                                                    dst,
                                                                )
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    }
                                    #[inline]
                                    fn linear_lower_to_memory<T>(
                                        &self,
                                        cx: &mut wasmtime::component::__internal::LowerContext<
                                            '_,
                                            T,
                                        >,
                                        ty: wasmtime::component::__internal::InterfaceType,
                                        mut offset: usize,
                                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                                        let ty = match ty {
                                            wasmtime::component::__internal::InterfaceType::Variant(
                                                i,
                                            ) => &cx.types[i],
                                            _ => wasmtime::component::__internal::bad_type_info(),
                                        };
                                        if true {
                                            if !(offset
                                                % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                                    as usize) == 0)
                                            {
                                                ::core::panicking::panic(
                                                    "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                                )
                                            }
                                        }
                                        match self {
                                            Self::SchemaInvalid(value) => {
                                                *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                                value
                                                    .linear_lower_to_memory(
                                                        cx,
                                                        ty
                                                            .cases[0usize]
                                                            .unwrap_or_else(
                                                                wasmtime::component::__internal::bad_type_info,
                                                            ),
                                                        offset
                                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                                    )
                                            }
                                            Self::Sql(value) => {
                                                *cx.get::<1usize>(offset) = 1u8.to_le_bytes();
                                                value
                                                    .linear_lower_to_memory(
                                                        cx,
                                                        ty
                                                            .cases[1usize]
                                                            .unwrap_or_else(
                                                                wasmtime::component::__internal::bad_type_info,
                                                            ),
                                                        offset
                                                            + <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32,
                                                    )
                                            }
                                        }
                                    }
                                }
                                unsafe impl wasmtime::component::Lift for SchemaError {
                                    #[inline]
                                    fn linear_lift_from_flat(
                                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                                        ty: wasmtime::component::__internal::InterfaceType,
                                        src: &Self::Lower,
                                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                                        let ty = match ty {
                                            wasmtime::component::__internal::InterfaceType::Variant(
                                                i,
                                            ) => &cx.types[i],
                                            _ => wasmtime::component::__internal::bad_type_info(),
                                        };
                                        Ok(
                                            match src.tag.get_u32() {
                                                0u32 => {
                                                    Self::SchemaInvalid(
                                                        <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_flat(
                                                            cx,
                                                            ty
                                                                .cases[0usize]
                                                                .unwrap_or_else(
                                                                    wasmtime::component::__internal::bad_type_info,
                                                                ),
                                                            unsafe { &src.payload.SchemaInvalid },
                                                        )?,
                                                    )
                                                }
                                                1u32 => {
                                                    Self::Sql(
                                                        <Error as wasmtime::component::Lift>::linear_lift_from_flat(
                                                            cx,
                                                            ty
                                                                .cases[1usize]
                                                                .unwrap_or_else(
                                                                    wasmtime::component::__internal::bad_type_info,
                                                                ),
                                                            unsafe { &src.payload.Sql },
                                                        )?,
                                                    )
                                                }
                                                discrim => {
                                                    return ::anyhow::__private::Err(
                                                        ::anyhow::Error::msg(
                                                            ::alloc::__export::must_use({
                                                                ::alloc::fmt::format(
                                                                    format_args!("unexpected discriminant: {0}", discrim),
                                                                )
                                                            }),
                                                        ),
                                                    );
                                                }
                                            },
                                        )
                                    }
                                    #[inline]
                                    fn linear_lift_from_memory(
                                        cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                                        ty: wasmtime::component::__internal::InterfaceType,
                                        bytes: &[u8],
                                    ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                                        let align = <Self as wasmtime::component::ComponentType>::ALIGN32;
                                        if true {
                                            if !((bytes.as_ptr() as usize) % (align as usize) == 0) {
                                                ::core::panicking::panic(
                                                    "assertion failed: (bytes.as_ptr() as usize) % (align as usize) == 0",
                                                )
                                            }
                                        }
                                        let discrim = bytes[0];
                                        let payload_offset = <Self as wasmtime::component::__internal::ComponentVariant>::PAYLOAD_OFFSET32;
                                        let payload = &bytes[payload_offset..];
                                        let ty = match ty {
                                            wasmtime::component::__internal::InterfaceType::Variant(
                                                i,
                                            ) => &cx.types[i],
                                            _ => wasmtime::component::__internal::bad_type_info(),
                                        };
                                        Ok(
                                            match discrim {
                                                0u8 => {
                                                    Self::SchemaInvalid(
                                                        <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_memory(
                                                            cx,
                                                            ty
                                                                .cases[0usize]
                                                                .unwrap_or_else(
                                                                    wasmtime::component::__internal::bad_type_info,
                                                                ),
                                                            &payload[..<wasmtime::component::__internal::String as wasmtime::component::ComponentType>::SIZE32],
                                                        )?,
                                                    )
                                                }
                                                1u8 => {
                                                    Self::Sql(
                                                        <Error as wasmtime::component::Lift>::linear_lift_from_memory(
                                                            cx,
                                                            ty
                                                                .cases[1usize]
                                                                .unwrap_or_else(
                                                                    wasmtime::component::__internal::bad_type_info,
                                                                ),
                                                            &payload[..<Error as wasmtime::component::ComponentType>::SIZE32],
                                                        )?,
                                                    )
                                                }
                                                discrim => {
                                                    return ::anyhow::__private::Err(
                                                        ::anyhow::Error::msg(
                                                            ::alloc::__export::must_use({
                                                                ::alloc::fmt::format(
                                                                    format_args!("unexpected discriminant: {0}", discrim),
                                                                )
                                                            }),
                                                        ),
                                                    );
                                                }
                                            },
                                        )
                                    }
                                }
                                const _: () = {
                                    #[doc(hidden)]
                                    #[repr(C)]
                                    pub struct LowerSchemaError<T0: Copy, T1: Copy> {
                                        tag: wasmtime::ValRaw,
                                        payload: LowerPayloadSchemaError<T0, T1>,
                                    }
                                    #[automatically_derived]
                                    impl<
                                        T0: ::core::clone::Clone + Copy,
                                        T1: ::core::clone::Clone + Copy,
                                    > ::core::clone::Clone for LowerSchemaError<T0, T1> {
                                        #[inline]
                                        fn clone(&self) -> LowerSchemaError<T0, T1> {
                                            LowerSchemaError {
                                                tag: ::core::clone::Clone::clone(&self.tag),
                                                payload: ::core::clone::Clone::clone(&self.payload),
                                            }
                                        }
                                    }
                                    #[automatically_derived]
                                    impl<
                                        T0: ::core::marker::Copy + Copy,
                                        T1: ::core::marker::Copy + Copy,
                                    > ::core::marker::Copy for LowerSchemaError<T0, T1> {}
                                    #[doc(hidden)]
                                    #[allow(non_snake_case)]
                                    #[repr(C)]
                                    union LowerPayloadSchemaError<T0: Copy, T1: Copy> {
                                        SchemaInvalid: T0,
                                        Sql: T1,
                                    }
                                    #[automatically_derived]
                                    #[allow(non_snake_case)]
                                    impl<
                                        T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
                                        T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                                    > ::core::clone::Clone for LowerPayloadSchemaError<T0, T1> {
                                        #[inline]
                                        fn clone(&self) -> LowerPayloadSchemaError<T0, T1> {
                                            let _: ::core::clone::AssertParamIsCopy<Self>;
                                            *self
                                        }
                                    }
                                    #[automatically_derived]
                                    #[allow(non_snake_case)]
                                    impl<
                                        T0: ::core::marker::Copy + Copy,
                                        T1: ::core::marker::Copy + Copy,
                                    > ::core::marker::Copy for LowerPayloadSchemaError<T0, T1> {}
                                    unsafe impl wasmtime::component::ComponentType
                                    for SchemaError {
                                        type Lower = LowerSchemaError<
                                            <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::Lower,
                                            <Error as wasmtime::component::ComponentType>::Lower,
                                        >;
                                        #[inline]
                                        fn typecheck(
                                            ty: &wasmtime::component::__internal::InterfaceType,
                                            types: &wasmtime::component::__internal::InstanceType<'_>,
                                        ) -> wasmtime::component::__internal::anyhow::Result<()> {
                                            wasmtime::component::__internal::typecheck_variant(
                                                ty,
                                                types,
                                                &[
                                                    (
                                                        "schema-invalid",
                                                        Some(
                                                            <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::typecheck,
                                                        ),
                                                    ),
                                                    (
                                                        "sql",
                                                        Some(
                                                            <Error as wasmtime::component::ComponentType>::typecheck,
                                                        ),
                                                    ),
                                                ],
                                            )
                                        }
                                        const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                                            &[
                                                Some(
                                                    <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                                ),
                                                Some(<Error as wasmtime::component::ComponentType>::ABI),
                                            ],
                                        );
                                    }
                                    unsafe impl wasmtime::component::__internal::ComponentVariant
                                    for SchemaError {
                                        const CASES: &'static [Option<
                                            wasmtime::component::__internal::CanonicalAbiInfo,
                                        >] = &[
                                            Some(
                                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                            ),
                                            Some(<Error as wasmtime::component::ComponentType>::ABI),
                                        ];
                                    }
                                };
                                impl core::fmt::Debug for SchemaError {
                                    fn fmt(
                                        &self,
                                        f: &mut core::fmt::Formatter<'_>,
                                    ) -> core::fmt::Result {
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
                                impl core::fmt::Display for SchemaError {
                                    fn fmt(
                                        &self,
                                        f: &mut core::fmt::Formatter<'_>,
                                    ) -> core::fmt::Result {
                                        f.write_fmt(format_args!("{0:?}", self))
                                    }
                                }
                                impl core::error::Error for SchemaError {}
                                const _: () = {
                                    if !(16
                                        == <SchemaError as wasmtime::component::ComponentType>::SIZE32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 16 == <SchemaError as wasmtime::component::ComponentType>::SIZE32",
                                        )
                                    }
                                    if !(4
                                        == <SchemaError as wasmtime::component::ComponentType>::ALIGN32)
                                    {
                                        ::core::panicking::panic(
                                            "assertion failed: 4 == <SchemaError as wasmtime::component::ComponentType>::ALIGN32",
                                        )
                                    }
                                };
                                pub struct Guest {
                                    get_migration_group: wasmtime::component::Func,
                                    list_migrations: wasmtime::component::Func,
                                    apply_migration: wasmtime::component::Func,
                                    check_schema: wasmtime::component::Func,
                                }
                                pub struct GuestIndices {
                                    get_migration_group: wasmtime::component::ComponentExportIndex,
                                    list_migrations: wasmtime::component::ComponentExportIndex,
                                    apply_migration: wasmtime::component::ComponentExportIndex,
                                    check_schema: wasmtime::component::ComponentExportIndex,
                                }
                                #[automatically_derived]
                                impl ::core::clone::Clone for GuestIndices {
                                    #[inline]
                                    fn clone(&self) -> GuestIndices {
                                        GuestIndices {
                                            get_migration_group: ::core::clone::Clone::clone(
                                                &self.get_migration_group,
                                            ),
                                            list_migrations: ::core::clone::Clone::clone(
                                                &self.list_migrations,
                                            ),
                                            apply_migration: ::core::clone::Clone::clone(
                                                &self.apply_migration,
                                            ),
                                            check_schema: ::core::clone::Clone::clone(
                                                &self.check_schema,
                                            ),
                                        }
                                    }
                                }
                                impl GuestIndices {
                                    /// Constructor for [`GuestIndices`] which takes a
                                    /// [`Component`](wasmtime::component::Component) as input and can be executed
                                    /// before instantiation.
                                    ///
                                    /// This constructor can be used to front-load string lookups to find exports
                                    /// within a component.
                                    pub fn new<_T>(
                                        _instance_pre: &wasmtime::component::InstancePre<_T>,
                                    ) -> wasmtime::Result<GuestIndices> {
                                        let instance = _instance_pre
                                            .component()
                                            .get_export_index(None, "wasmledger:plugin/migrations")
                                            .ok_or_else(|| ::anyhow::__private::must_use({
                                                let error = ::anyhow::__private::format_err(
                                                    format_args!(
                                                        "no exported instance named `wasmledger:plugin/migrations`",
                                                    ),
                                                );
                                                error
                                            }))?;
                                        let mut lookup = move |name| {
                                            _instance_pre
                                                .component()
                                                .get_export_index(Some(&instance), name)
                                                .ok_or_else(|| {
                                                    ::anyhow::__private::must_use({
                                                        let error = ::anyhow::__private::format_err(
                                                            format_args!(
                                                                "instance export `wasmledger:plugin/migrations` does not have export `{0}`",
                                                                name,
                                                            ),
                                                        );
                                                        error
                                                    })
                                                })
                                        };
                                        let _ = &mut lookup;
                                        let get_migration_group = lookup("get-migration-group")?;
                                        let list_migrations = lookup("list-migrations")?;
                                        let apply_migration = lookup("apply-migration")?;
                                        let check_schema = lookup("check-schema")?;
                                        Ok(GuestIndices {
                                            get_migration_group,
                                            list_migrations,
                                            apply_migration,
                                            check_schema,
                                        })
                                    }
                                    pub fn load(
                                        &self,
                                        mut store: impl wasmtime::AsContextMut,
                                        instance: &wasmtime::component::Instance,
                                    ) -> wasmtime::Result<Guest> {
                                        let _instance = instance;
                                        let _instance_pre = _instance.instance_pre(&store);
                                        let _instance_type = _instance_pre.instance_type();
                                        let mut store = store.as_context_mut();
                                        let _ = &mut store;
                                        let get_migration_group = *_instance
                                            .get_typed_func::<
                                                (),
                                                (wasmtime::component::__internal::String,),
                                            >(&mut store, &self.get_migration_group)?
                                            .func();
                                        let list_migrations = *_instance
                                            .get_typed_func::<
                                                (),
                                                (wasmtime::component::__internal::Vec<MigrationId>,),
                                            >(&mut store, &self.list_migrations)?
                                            .func();
                                        let apply_migration = *_instance
                                            .get_typed_func::<
                                                (&MigrationId, QueryExecutor),
                                                (Result<(), Error>,),
                                            >(&mut store, &self.apply_migration)?
                                            .func();
                                        let check_schema = *_instance
                                            .get_typed_func::<
                                                (QueryExecutor,),
                                                (Result<(), SchemaError>,),
                                            >(&mut store, &self.check_schema)?
                                            .func();
                                        Ok(Guest {
                                            get_migration_group,
                                            list_migrations,
                                            apply_migration,
                                            check_schema,
                                        })
                                    }
                                }
                                impl Guest {
                                    pub fn call_get_migration_group<S: wasmtime::AsContextMut>(
                                        &self,
                                        mut store: S,
                                    ) -> wasmtime::Result<
                                        wasmtime::component::__internal::String,
                                    > {
                                        let callee = unsafe {
                                            wasmtime::component::TypedFunc::<
                                                (),
                                                (wasmtime::component::__internal::String,),
                                            >::new_unchecked(self.get_migration_group)
                                        };
                                        let (ret0,) = callee.call(store.as_context_mut(), ())?;
                                        callee.post_return(store.as_context_mut())?;
                                        Ok(ret0)
                                    }
                                    pub async fn call_list_migrations<_T, _D>(
                                        &self,
                                        accessor: &wasmtime::component::Accessor<_T, _D>,
                                    ) -> wasmtime::Result<
                                        wasmtime::component::__internal::Vec<MigrationId>,
                                    >
                                    where
                                        _T: Send,
                                        _D: wasmtime::component::HasData,
                                    {
                                        let callee = unsafe {
                                            wasmtime::component::TypedFunc::<
                                                (),
                                                (wasmtime::component::__internal::Vec<MigrationId>,),
                                            >::new_unchecked(self.list_migrations)
                                        };
                                        let ((ret0,), _) = callee
                                            .call_concurrent(accessor, ())
                                            .await?;
                                        Ok(ret0)
                                    }
                                    pub async fn call_apply_migration<_T, _D>(
                                        &self,
                                        accessor: &wasmtime::component::Accessor<_T, _D>,
                                        arg0: MigrationId,
                                        arg1: QueryExecutor,
                                    ) -> wasmtime::Result<Result<(), Error>>
                                    where
                                        _T: Send,
                                        _D: wasmtime::component::HasData,
                                    {
                                        let callee = unsafe {
                                            wasmtime::component::TypedFunc::<
                                                (MigrationId, QueryExecutor),
                                                (Result<(), Error>,),
                                            >::new_unchecked(self.apply_migration)
                                        };
                                        let ((ret0,), _) = callee
                                            .call_concurrent(accessor, (arg0, arg1))
                                            .await?;
                                        Ok(ret0)
                                    }
                                    pub async fn call_check_schema<_T, _D>(
                                        &self,
                                        accessor: &wasmtime::component::Accessor<_T, _D>,
                                        arg0: QueryExecutor,
                                    ) -> wasmtime::Result<Result<(), SchemaError>>
                                    where
                                        _T: Send,
                                        _D: wasmtime::component::HasData,
                                    {
                                        let callee = unsafe {
                                            wasmtime::component::TypedFunc::<
                                                (QueryExecutor,),
                                                (Result<(), SchemaError>,),
                                            >::new_unchecked(self.check_schema)
                                        };
                                        let ((ret0,), _) = callee
                                            .call_concurrent(accessor, (arg0,))
                                            .await?;
                                        Ok(ret0)
                                    }
                                }
                            }
                        }
                    }
                }
                const _: &str = "package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n  use connection.{connection};\n\n  begin-transaction: async func() -> result<transaction, error>;\n  acquire-connection: async func() -> result<connection, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}\n\ninterface connection {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  resource connection {\n    begin-transaction: async func() -> result<transaction, error>;\n  }\n}";
                const _: &str = "package wasmledger:sql;\n\ninterface codecs {\n  use util-types.{error};\n\n  type push-result = result<_, error>;\n\n  type row-index = u64;\n\n  variant column-index {\n    index(u64),\n    column-name(string)\n  }\n\n  type value-position = tuple<row-index, column-index>;\n}";
                const _: &str = "package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use connection.{connection};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    connection(borrow<connection>),\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
                const _: &str = "package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// attempted to perform operation on already closed transaction\n    transaction-closed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results {\n    row-count: func() -> u64;\n  }\n}";
                const _: &str = "package wasmledger:plugin;\n\ninterface migrations {\n  use wasmledger:sql/query.{query-executor};\n  use wasmledger:sql/util-types.{error};\n\n  type migration-id = string;\n\n  variant schema-error {\n    schema-invalid(string),\n    sql(error)\n  }\n\n  get-migration-group: func() -> string;\n\n  list-migrations: async func() -> list<migration-id>;\n  apply-migration: async func(id: migration-id, executor: query-executor) -> result<_, error>;\n\n  check-schema: async func(executor: query-executor) -> result<_, schema-error>;\n}";
                const _: &str = "package wasmledger:plugin-client;\n\nworld client {\n  export wasmledger:plugin/migrations;\n}";
            }
            /// Migrations interface implementation
            pub struct MigrationsPluginClient;
            const SUPPORTED_INTERFACES: [&str; 1] = ["wasmledger:plugin/migrations"];
            impl PluginClient for MigrationsPluginClient {
                fn is_interface_supported(interface: &str) -> bool {
                    SUPPORTED_INTERFACES.contains(&interface)
                }
                fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()> {
                    bindings::Client::add_to_linker::<
                        CoreState,
                        wasmledger_sql::core::bindings::BindingsImplState,
                    >(linker, |state: &mut CoreState| &mut state.sql)
                        .context("Component doesn't support migrations interface")?;
                    Ok(())
                }
            }
            impl MigrationsPluginClient {
                /// Run migrations for all plugins that support the migrations interface
                ///
                /// Execution order: config file order
                /// Failure policy: stop on first error
                /// Logging: info for each plugin, silent skip for non-supporting plugins
                pub async fn run_migrations(registry: &PluginRegistry) -> Result<()> {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/host/src/plugin/client/migrations.rs:54",
                                    "wasmledger_host::plugin::client::migrations",
                                    ::tracing::Level::INFO,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "crates/host/src/plugin/client/migrations.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(54u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "wasmledger_host::plugin::client::migrations",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                                if match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &value_set,
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set_all(
                                        &[
                                            (::tracing::__macro_support::Option::Some(
                                                &format_args!("Starting plugin migrations...")
                                                    as &dyn ::tracing::field::Value,
                                            )),
                                        ],
                                    )
                            });
                        } else {
                            if match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let meta = __CALLSITE.metadata();
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target(meta.target())
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                ::tracing::__macro_support::__tracing_log(
                                                    meta,
                                                    logger,
                                                    log_meta,
                                                    &{
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display, Value};
                                                        __CALLSITE
                                                            .metadata()
                                                            .fields()
                                                            .value_set_all(
                                                                &[
                                                                    (::tracing::__macro_support::Option::Some(
                                                                        &format_args!("Starting plugin migrations...")
                                                                            as &dyn ::tracing::field::Value,
                                                                    )),
                                                                ],
                                                            )
                                                    },
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                        }
                    };
                    for plugin in registry.all() {
                        let mut store = plugin.store.lock().unwrap();
                        if let Some(client) = bindings::Client::new(
                                &mut *store,
                                &plugin.instance,
                            )
                            .ok()
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/host/src/plugin/client/migrations.rs:61",
                                            "wasmledger_host::plugin::client::migrations",
                                            ::tracing::Level::INFO,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/host/src/plugin/client/migrations.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(61u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "wasmledger_host::plugin::client::migrations",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &[
                                                    "message",
                                                    {
                                                        const NAME: ::tracing::__macro_support::FieldName<
                                                            { ::tracing::__macro_support::FieldName::len("plugin") },
                                                        > = ::tracing::__macro_support::FieldName::new("plugin");
                                                        NAME.as_str()
                                                    },
                                                ],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                        if match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &value_set,
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set_all(
                                                &[
                                                    (::tracing::__macro_support::Option::Some(
                                                        &format_args!("Running migrations")
                                                            as &dyn ::tracing::field::Value,
                                                    )),
                                                    (::tracing::__macro_support::Option::Some(
                                                        &::tracing::field::display(&plugin.id)
                                                            as &dyn ::tracing::field::Value,
                                                    )),
                                                ],
                                            )
                                    });
                                } else {
                                    if match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &{
                                                                #[allow(unused_imports)]
                                                                use ::tracing::field::{debug, display, Value};
                                                                __CALLSITE
                                                                    .metadata()
                                                                    .fields()
                                                                    .value_set_all(
                                                                        &[
                                                                            (::tracing::__macro_support::Option::Some(
                                                                                &format_args!("Running migrations")
                                                                                    as &dyn ::tracing::field::Value,
                                                                            )),
                                                                            (::tracing::__macro_support::Option::Some(
                                                                                &::tracing::field::display(&plugin.id)
                                                                                    as &dyn ::tracing::field::Value,
                                                                            )),
                                                                        ],
                                                                    )
                                                            },
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                }
                            };
                            let migrator = client.wasmledger_plugin_migrations();
                            store
                                .run_concurrent(async |accessor| -> anyhow::Result<()> {
                                    let migrations = migrator
                                        .call_list_migrations(accessor)
                                        .await?;
                                    let t = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("migrations {0}", migrations.join(",")),
                                        )
                                    });
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/host/src/plugin/client/migrations.rs:68",
                                                    "wasmledger_host::plugin::client::migrations",
                                                    ::tracing::Level::INFO,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/host/src/plugin/client/migrations.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(68u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "wasmledger_host::plugin::client::migrations",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &[
                                                            {
                                                                const NAME: ::tracing::__macro_support::FieldName<
                                                                    { ::tracing::__macro_support::FieldName::len("plugin") },
                                                                > = ::tracing::__macro_support::FieldName::new("plugin");
                                                                NAME.as_str()
                                                            },
                                                            {
                                                                const NAME: ::tracing::__macro_support::FieldName<
                                                                    { ::tracing::__macro_support::FieldName::len("t") },
                                                                > = ::tracing::__macro_support::FieldName::new("t");
                                                                NAME.as_str()
                                                            },
                                                        ],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set_all(
                                                        &[
                                                            (::tracing::__macro_support::Option::Some(
                                                                &::tracing::field::display(&plugin.id)
                                                                    as &dyn ::tracing::field::Value,
                                                            )),
                                                            (::tracing::__macro_support::Option::Some(
                                                                &t as &dyn ::tracing::field::Value,
                                                            )),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set_all(
                                                                                &[
                                                                                    (::tracing::__macro_support::Option::Some(
                                                                                        &::tracing::field::display(&plugin.id)
                                                                                            as &dyn ::tracing::field::Value,
                                                                                    )),
                                                                                    (::tracing::__macro_support::Option::Some(
                                                                                        &t as &dyn ::tracing::field::Value,
                                                                                    )),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                    Ok(())
                                })
                                .await;
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/host/src/plugin/client/migrations.rs:74",
                                            "wasmledger_host::plugin::client::migrations",
                                            ::tracing::Level::INFO,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/host/src/plugin/client/migrations.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(74u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "wasmledger_host::plugin::client::migrations",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &[
                                                    "message",
                                                    {
                                                        const NAME: ::tracing::__macro_support::FieldName<
                                                            { ::tracing::__macro_support::FieldName::len("plugin") },
                                                        > = ::tracing::__macro_support::FieldName::new("plugin");
                                                        NAME.as_str()
                                                    },
                                                ],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                        if match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &value_set,
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set_all(
                                                &[
                                                    (::tracing::__macro_support::Option::Some(
                                                        &format_args!("Migrations completed successfully")
                                                            as &dyn ::tracing::field::Value,
                                                    )),
                                                    (::tracing::__macro_support::Option::Some(
                                                        &::tracing::field::display(&plugin.id)
                                                            as &dyn ::tracing::field::Value,
                                                    )),
                                                ],
                                            )
                                    });
                                } else {
                                    if match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &{
                                                                #[allow(unused_imports)]
                                                                use ::tracing::field::{debug, display, Value};
                                                                __CALLSITE
                                                                    .metadata()
                                                                    .fields()
                                                                    .value_set_all(
                                                                        &[
                                                                            (::tracing::__macro_support::Option::Some(
                                                                                &format_args!("Migrations completed successfully")
                                                                                    as &dyn ::tracing::field::Value,
                                                                            )),
                                                                            (::tracing::__macro_support::Option::Some(
                                                                                &::tracing::field::display(&plugin.id)
                                                                                    as &dyn ::tracing::field::Value,
                                                                            )),
                                                                        ],
                                                                    )
                                                            },
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                }
                            };
                        }
                    }
                    Ok(())
                }
            }
        }
        /// Trait for plugin interfaces that can be linked by the host
        pub trait PluginClient {
            fn is_interface_supported(interface: &str) -> bool;
            /// Try to add this interface to the linker if component supports it
            /// Returns Ok(()) if added, Err if not supported
            fn add_to_linker(linker: &mut Linker<CoreState>) -> anyhow::Result<()>;
        }
        pub fn add_all_clients_to_linker(
            linker: &mut Linker<CoreState>,
        ) -> anyhow::Result<()> {
            migrations::MigrationsPluginClient::add_to_linker(linker)?;
            Ok(())
        }
    }
    pub mod registry {
        use anyhow::Context;
        use serde::Deserialize;
        use std::collections::HashSet;
        use std::sync::{Arc, Mutex};
        use std::{collections::HashMap, path::PathBuf};
        use wasmtime::{Engine, Store, component::{Component, Linker}};
        use crate::config::HostConfig;
        use crate::plugin::{LoadedPlugin, client};
        use crate::{capabilities, engine::{CoreState, create_core_state}};
        /// Single plugin entry in configuration
        pub struct PluginEntry {
            /// Plugin identifier (e.g., "money", "core", "my-custom-plugin")
            pub id: String,
            /// Path to the .wasm file
            pub path: PathBuf,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PluginEntry {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PluginEntry",
                    "id",
                    &self.id,
                    "path",
                    &&self.path,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PluginEntry {
            #[inline]
            fn clone(&self) -> PluginEntry {
                PluginEntry {
                    id: ::core::clone::Clone::clone(&self.id),
                    path: ::core::clone::Clone::clone(&self.path),
                }
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PluginEntry {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                1u64 => _serde::__private228::Ok(__Field::__field1),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private228::Ok(__Field::__field0),
                                "path" => _serde::__private228::Ok(__Field::__field1),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private228::Ok(__Field::__field0),
                                b"path" => _serde::__private228::Ok(__Field::__field1),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<PluginEntry>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PluginEntry;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct PluginEntry",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct PluginEntry with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                PathBuf,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct PluginEntry with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(PluginEntry {
                                id: __field0,
                                path: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                            let mut __field1: _serde::__private228::Option<PathBuf> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private228::Option::is_some(&__field1) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("path"),
                                            );
                                        }
                                        __field1 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<PathBuf>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private228::Some(__field1) => __field1,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("path")?
                                }
                            };
                            _serde::__private228::Ok(PluginEntry {
                                id: __field0,
                                path: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "path"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "PluginEntry",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<PluginEntry>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        /// Registry of all loaded plugins
        ///
        /// Plugins are loaded once at startup, compiled into Components, and instantiated.
        /// The base linker contains all plugin interfaces (without host capabilities).
        pub struct PluginRegistry {
            /// Map from plugin ID to loaded plugin
            plugins: HashMap<String, LoadedPlugin>,
            /// Wasmtime engine (stored for potential future use)
            engine: Engine,
        }
        impl PluginRegistry {
            /// Load all plugins from configuration
            ///
            /// This reads .wasm files from disk, compiles them into Components,
            /// and stores them in the registry for future instantiation.
            pub async fn load_from_config(
                engine: &Engine,
                config: &HostConfig,
            ) -> anyhow::Result<Self> {
                let mut registry = Self {
                    plugins: HashMap::new(),
                    engine: engine.clone(),
                };
                for entry in config.plugins.iter() {
                    registry.instantiate_plugin(entry).await?;
                }
                Ok(registry)
            }
            /// Load a single plugin component from a PluginEntry
            async fn instantiate_plugin(
                &mut self,
                entry: &PluginEntry,
            ) -> anyhow::Result<()> {
                let component = Component::from_file(&self.engine, &entry.path)
                    .map_err(|e| {
                        ::anyhow::Error::msg(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to compile plugin \'{0}\' from {1:?}: {2}",
                                        entry.id,
                                        entry.path,
                                        e,
                                    ),
                                )
                            }),
                        )
                    })?;
                let imported_interfaces: HashSet<String> = component
                    .component_type()
                    .imports(&self.engine)
                    .map(|(name, _)| name.to_string())
                    .collect();
                let store_state = create_core_state().await?;
                let mut store = Store::new(&self.engine, store_state);
                let linker = create_default_linker(&self.engine)?;
                let instance = linker
                    .instantiate_async(&mut store, &component)
                    .await
                    .context(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Failed to instantiate plugin \'{0}\'",
                                    &entry.id,
                                ),
                            )
                        }),
                    )?;
                let loaded_plugin = LoadedPlugin {
                    id: entry.id.clone(),
                    component: Arc::new(component),
                    imported_interfaces,
                    instance: instance,
                    store: Mutex::new(store),
                };
                self.plugins.insert(entry.id.clone(), loaded_plugin);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event crates/host/src/plugin/registry.rs:98",
                                "wasmledger_host::plugin::registry",
                                ::tracing::Level::DEBUG,
                                ::tracing_core::__macro_support::Option::Some(
                                    "crates/host/src/plugin/registry.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(98u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "wasmledger_host::plugin::registry",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &[
                                        "message",
                                        {
                                            const NAME: ::tracing::__macro_support::FieldName<
                                                { ::tracing::__macro_support::FieldName::len("plugin") },
                                            > = ::tracing::__macro_support::FieldName::new("plugin");
                                            NAME.as_str()
                                        },
                                    ],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::DEBUG
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::DEBUG
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                            if match ::tracing::Level::DEBUG {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::DEBUG {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let meta = __CALLSITE.metadata();
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target(meta.target())
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                ::tracing::__macro_support::__tracing_log(
                                                    meta,
                                                    logger,
                                                    log_meta,
                                                    &value_set,
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set_all(
                                    &[
                                        (::tracing::__macro_support::Option::Some(
                                            &format_args!("Plugin instantiated")
                                                as &dyn ::tracing::field::Value,
                                        )),
                                        (::tracing::__macro_support::Option::Some(
                                            &::tracing::field::display(&entry.id)
                                                as &dyn ::tracing::field::Value,
                                        )),
                                    ],
                                )
                        });
                    } else {
                        if match ::tracing::Level::DEBUG {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::DEBUG {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let meta = __CALLSITE.metadata();
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target(meta.target())
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            ::tracing::__macro_support::__tracing_log(
                                                meta,
                                                logger,
                                                log_meta,
                                                &{
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set_all(
                                                            &[
                                                                (::tracing::__macro_support::Option::Some(
                                                                    &format_args!("Plugin instantiated")
                                                                        as &dyn ::tracing::field::Value,
                                                                )),
                                                                (::tracing::__macro_support::Option::Some(
                                                                    &::tracing::field::display(&entry.id)
                                                                        as &dyn ::tracing::field::Value,
                                                                )),
                                                            ],
                                                        )
                                                },
                                            )
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                    }
                };
                Ok(())
            }
            /// Get a plugin by ID
            pub fn get(&self, id: &str) -> Option<&LoadedPlugin> {
                self.plugins.get(id)
            }
            /// Iterate over all loaded plugins
            pub fn all(&self) -> impl Iterator<Item = &LoadedPlugin> {
                self.plugins.values()
            }
            /// Get the number of loaded plugins
            pub fn count(&self) -> usize {
                self.plugins.len()
            }
        }
        fn create_default_linker(engine: &Engine) -> anyhow::Result<Linker<CoreState>> {
            let mut linker = Linker::new(engine);
            linker.allow_shadowing(true);
            client::add_all_clients_to_linker(&mut linker)?;
            capabilities::add_to_linker(&mut linker)?;
            Ok(linker)
        }
    }
    /// A plugin that has been loaded and compiled, ready for instantiation
    pub struct LoadedPlugin {
        /// Plugin identifier from config (e.g., "money", "core", "my-custom-plugin")
        pub id: String,
        /// Pre-compiled WebAssembly component
        ///
        /// This is wrapped in Arc because it's shared across all instantiations.
        /// Components are cheap to instantiate but expensive to compile, so we
        /// compile once at startup and instantiate per-request for isolation.
        pub component: Arc<Component>,
        pub imported_interfaces: HashSet<String>,
        /// Pre-instantiated plugin instance
        ///
        /// Plugins are instantiated once at startup and reused throughout the application lifetime.
        /// This allows plugin interfaces to be called directly without re-instantiation.
        pub instance: Instance,
        /// Store for the plugin instance
        ///
        /// Each plugin gets its own Store with CapabilitiesState.
        /// Mutex provides thread-safe interior mutability for the Store, which must be mutable when executing WASM.
        pub store: Mutex<Store<CoreState>>,
    }
    impl std::fmt::Debug for LoadedPlugin {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("LoadedPlugin")
                .field("id", &self.id)
                .field("component", &"<Component>")
                .finish()
        }
    }
}
use crate::{app_state::AppState, api::execute_function_handler};
fn main() -> anyhow::Result<()> {
    let body = async {
        let app_state = Arc::new(AppState::initialize().await?);
        let plugin_count = app_state.plugin_registry.count();
        if plugin_count > 0 {
            {
                ::std::io::_print(format_args!("Loaded {0} plugin(s):\n", plugin_count));
            };
            for plugin in app_state.plugin_registry.all() {
                {
                    ::std::io::_print(format_args!("  - {0}\n", plugin.id));
                };
            }
        } else {
            {
                ::std::io::_print(
                    format_args!(
                        "No plugins loaded (create config.yaml to load plugins)\n",
                    ),
                );
            };
        }
        let app = Router::new()
            .route("/execute", post(execute_function_handler))
            .with_state(app_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        {
            ::std::io::_print(
                format_args!("Server listening on http://127.0.0.1:3000\n"),
            );
        };
        {
            ::std::io::_print(
                format_args!(
                    "POST /execute - Execute a ledger function (multipart/form-data)\n",
                ),
            );
        };
        {
            ::std::io::_print(
                format_args!("  - field \'wasm\': WASM function bytes (required)\n"),
            );
        };
        {
            ::std::io::_print(
                format_args!(
                    "  - field \'input\': JSON parameters (optional, future use)\n",
                ),
            );
        };
        axum::serve(listener, app).await?;
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}

#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod sqldb {
    use sqlx::Pool;
    pub(crate) type SqlDatabase = sqlx::Postgres;
    pub struct SqlDB {
        pub(crate) pool: Pool<SqlDatabase>,
    }
    impl SqlDB {
        pub fn new(pool: Pool<SqlDatabase>) -> Self {
            Self { pool: pool }
        }
    }
}
mod core {
    pub mod bindings {
        #[allow(dead_code, clippy::all)]
        pub mod wasmledger {
            pub mod sql {
                /// Types used by components and providers of a sql
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod util_types {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub enum Error {
                        /// no rows returned by a query that expected to return at least one row.
                        RowNotFound,
                        /// error occurred while encoding a value
                        Encode(_rt::String),
                        /// error occurred while decoding
                        Decode(_rt::String),
                        /// pool timed out while waiting for an open connection
                        PoolTimedOut,
                        /// attempted to acquire a connection on a closed pool
                        PoolClosed,
                        /// got unexpected connection status after attempting to begin transaction
                        BeginFailed,
                        /// generic unexpected error
                        Unexpected(_rt::String),
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Error {
                        #[inline]
                        fn clone(&self) -> Error {
                            match self {
                                Error::RowNotFound => Error::RowNotFound,
                                Error::Encode(__self_0) => {
                                    Error::Encode(::core::clone::Clone::clone(__self_0))
                                }
                                Error::Decode(__self_0) => {
                                    Error::Decode(::core::clone::Clone::clone(__self_0))
                                }
                                Error::PoolTimedOut => Error::PoolTimedOut,
                                Error::PoolClosed => Error::PoolClosed,
                                Error::BeginFailed => Error::BeginFailed,
                                Error::Unexpected(__self_0) => {
                                    Error::Unexpected(::core::clone::Clone::clone(__self_0))
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Debug for Error {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            match self {
                                Error::RowNotFound => {
                                    f.debug_tuple("Error::RowNotFound").finish()
                                }
                                Error::Encode(e) => {
                                    f.debug_tuple("Error::Encode").field(e).finish()
                                }
                                Error::Decode(e) => {
                                    f.debug_tuple("Error::Decode").field(e).finish()
                                }
                                Error::PoolTimedOut => {
                                    f.debug_tuple("Error::PoolTimedOut").finish()
                                }
                                Error::PoolClosed => {
                                    f.debug_tuple("Error::PoolClosed").finish()
                                }
                                Error::BeginFailed => {
                                    f.debug_tuple("Error::BeginFailed").finish()
                                }
                                Error::Unexpected(e) => {
                                    f.debug_tuple("Error::Unexpected").field(e).finish()
                                }
                            }
                        }
                    }
                    impl ::core::fmt::Display for Error {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter<'_>,
                        ) -> ::core::fmt::Result {
                            f.write_fmt(format_args!("{0:?}", self))
                        }
                    }
                    impl ::core::error::Error for Error {}
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod exports {
            pub mod wasmledger {
                pub mod sql {
                    #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                    pub mod transaction {
                        #[used]
                        #[doc(hidden)]
                        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                        use super::super::super::super::_rt;
                        pub type Error = super::super::super::super::wasmledger::sql::util_types::Error;
                        #[repr(transparent)]
                        pub struct Transaction {
                            handle: _rt::Resource<Transaction>,
                        }
                        #[automatically_derived]
                        impl ::core::fmt::Debug for Transaction {
                            #[inline]
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field1_finish(
                                    f,
                                    "Transaction",
                                    "handle",
                                    &&self.handle,
                                )
                            }
                        }
                        type _TransactionRep<T> = Option<T>;
                        impl Transaction {
                            /// Creates a new resource from the specified representation.
                            ///
                            /// This function will create a new resource handle by moving `val` onto
                            /// the heap and then passing that heap pointer to the component model to
                            /// create a handle. The owned handle is then returned as `Transaction`.
                            pub fn new<T: GuestTransaction>(val: T) -> Self {
                                Self::type_guard::<T>();
                                let val: _TransactionRep<T> = Some(val);
                                let ptr: *mut _TransactionRep<T> = _rt::Box::into_raw(
                                    _rt::Box::new(val),
                                );
                                unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                            }
                            /// Gets access to the underlying `T` which represents this resource.
                            pub fn get<T: GuestTransaction>(&self) -> &T {
                                let ptr = unsafe { &*self.as_ptr::<T>() };
                                ptr.as_ref().unwrap()
                            }
                            /// Gets mutable access to the underlying `T` which represents this
                            /// resource.
                            pub fn get_mut<T: GuestTransaction>(&mut self) -> &mut T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.as_mut().unwrap()
                            }
                            /// Consumes this resource and returns the underlying `T`.
                            pub fn into_inner<T: GuestTransaction>(self) -> T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.take().unwrap()
                            }
                            #[doc(hidden)]
                            pub unsafe fn from_handle(handle: u32) -> Self {
                                Self {
                                    handle: unsafe { _rt::Resource::from_handle(handle) },
                                }
                            }
                            #[doc(hidden)]
                            pub fn take_handle(&self) -> u32 {
                                _rt::Resource::take_handle(&self.handle)
                            }
                            #[doc(hidden)]
                            pub fn handle(&self) -> u32 {
                                _rt::Resource::handle(&self.handle)
                            }
                            #[doc(hidden)]
                            fn type_guard<T: 'static>() {
                                use core::any::TypeId;
                                static mut LAST_TYPE: Option<TypeId> = None;
                                unsafe {
                                    if !!false {
                                        ::core::panicking::panic(
                                            "assertion failed: !cfg!(target_feature = \"atomics\")",
                                        )
                                    }
                                    let id = TypeId::of::<T>();
                                    match LAST_TYPE {
                                        Some(ty) => {
                                            if !(ty == id) {
                                                {
                                                    ::core::panicking::panic_fmt(
                                                        format_args!("cannot use two types with this resource type"),
                                                    );
                                                }
                                            }
                                        }
                                        None => LAST_TYPE = Some(id),
                                    }
                                }
                            }
                            #[doc(hidden)]
                            pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                                Self::type_guard::<T>();
                                let _ = unsafe {
                                    _rt::Box::from_raw(handle as *mut _TransactionRep<T>)
                                };
                            }
                            fn as_ptr<T: GuestTransaction>(
                                &self,
                            ) -> *mut _TransactionRep<T> {
                                Transaction::type_guard::<T>();
                                T::_resource_rep(self.handle()).cast()
                            }
                        }
                        /// A borrowed version of [`Transaction`] which represents a borrowed value
                        /// with the lifetime `'a`.
                        #[repr(transparent)]
                        pub struct TransactionBorrow<'a> {
                            rep: *mut u8,
                            _marker: core::marker::PhantomData<&'a Transaction>,
                        }
                        #[automatically_derived]
                        impl<'a> ::core::fmt::Debug for TransactionBorrow<'a> {
                            #[inline]
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field2_finish(
                                    f,
                                    "TransactionBorrow",
                                    "rep",
                                    &self.rep,
                                    "_marker",
                                    &&self._marker,
                                )
                            }
                        }
                        impl<'a> TransactionBorrow<'a> {
                            #[doc(hidden)]
                            pub unsafe fn lift(rep: usize) -> Self {
                                Self {
                                    rep: rep as *mut u8,
                                    _marker: core::marker::PhantomData,
                                }
                            }
                            /// Gets access to the underlying `T` in this resource.
                            pub fn get<T: GuestTransaction>(&self) -> &T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.as_ref().unwrap()
                            }
                            fn as_ptr<T: 'static>(&self) -> *mut _TransactionRep<T> {
                                Transaction::type_guard::<T>();
                                self.rep.cast()
                            }
                        }
                        unsafe impl _rt::WasmResource for Transaction {
                            #[inline]
                            unsafe fn drop(_handle: u32) {
                                unsafe extern "C" fn drop(_: i32) {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe {
                                    drop(_handle as i32);
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_async_static_transaction_commit_cabi<
                            T: GuestTransaction,
                        >(arg0: i32) -> i32 {
                            unsafe {
                                wit_bindgen::rt::async_support::start_task(async move {
                                    let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                    let result0 = &{
                                        T::commit(Transaction::from_handle(arg0 as u32)).await
                                    };
                                    let (result6_0, result6_1, result6_2, result6_3) = match result0 {
                                        Ok(_) => (0i32, 0i32, ::core::ptr::null_mut(), 0usize),
                                        Err(e) => {
                                            use super::super::super::super::wasmledger::sql::util_types::Error as V4;
                                            let (result5_0, result5_1, result5_2) = match e {
                                                V4::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                                V4::Encode(e) => {
                                                    let vec1 = e;
                                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                                    let len1 = vec1.len();
                                                    (1i32, ptr1.cast_mut(), len1)
                                                }
                                                V4::Decode(e) => {
                                                    let vec2 = e;
                                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                                    let len2 = vec2.len();
                                                    (2i32, ptr2.cast_mut(), len2)
                                                }
                                                V4::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                                V4::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                                V4::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                                V4::Unexpected(e) => {
                                                    let vec3 = e;
                                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                                    let len3 = vec3.len();
                                                    (6i32, ptr3.cast_mut(), len3)
                                                }
                                            };
                                            (1i32, result5_0, result5_1, result5_2)
                                        }
                                    };
                                    unsafe extern "C" fn wit_import7(
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
                                    wit_import7(result6_0, result6_1, result6_2, result6_3);
                                })
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __callback_async_static_transaction_commit(
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
                        pub unsafe fn _export_async_static_transaction_rollback_cabi<
                            T: GuestTransaction,
                        >(arg0: i32) -> i32 {
                            unsafe {
                                wit_bindgen::rt::async_support::start_task(async move {
                                    let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                    let result0 = &{
                                        T::rollback(Transaction::from_handle(arg0 as u32)).await
                                    };
                                    let (result6_0, result6_1, result6_2, result6_3) = match result0 {
                                        Ok(_) => (0i32, 0i32, ::core::ptr::null_mut(), 0usize),
                                        Err(e) => {
                                            use super::super::super::super::wasmledger::sql::util_types::Error as V4;
                                            let (result5_0, result5_1, result5_2) = match e {
                                                V4::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                                V4::Encode(e) => {
                                                    let vec1 = e;
                                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                                    let len1 = vec1.len();
                                                    (1i32, ptr1.cast_mut(), len1)
                                                }
                                                V4::Decode(e) => {
                                                    let vec2 = e;
                                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                                    let len2 = vec2.len();
                                                    (2i32, ptr2.cast_mut(), len2)
                                                }
                                                V4::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                                V4::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                                V4::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                                V4::Unexpected(e) => {
                                                    let vec3 = e;
                                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                                    let len3 = vec3.len();
                                                    (6i32, ptr3.cast_mut(), len3)
                                                }
                                            };
                                            (1i32, result5_0, result5_1, result5_2)
                                        }
                                    };
                                    unsafe extern "C" fn wit_import7(
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
                                    wit_import7(result6_0, result6_1, result6_2, result6_3);
                                })
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __callback_async_static_transaction_rollback(
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
                            type Transaction: GuestTransaction;
                        }
                        pub trait GuestTransaction: 'static {
                            #[doc(hidden)]
                            unsafe fn _resource_new(val: *mut u8) -> u32
                            where
                                Self: Sized,
                            {
                                unsafe extern "C" fn new(_: *mut u8) -> i32 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe { new(val) as u32 }
                            }
                            #[doc(hidden)]
                            fn _resource_rep(handle: u32) -> *mut u8
                            where
                                Self: Sized,
                            {
                                unsafe extern "C" fn rep(_: i32) -> *mut u8 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe { rep(handle as i32) }
                            }
                            #[allow(async_fn_in_trait)]
                            async fn commit(this: Transaction) -> Result<(), Error>;
                            #[allow(async_fn_in_trait)]
                            async fn rollback(this: Transaction) -> Result<(), Error>;
                        }
                        #[doc(hidden)]
                        pub(crate) use __export_wasmledger_sql_transaction_cabi;
                    }
                    #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                    pub mod pool {
                        #[used]
                        #[doc(hidden)]
                        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                        use super::super::super::super::_rt;
                        pub type Error = super::super::super::super::wasmledger::sql::util_types::Error;
                        pub type Transaction = super::super::super::super::exports::wasmledger::sql::transaction::Transaction;
                        pub type TransactionBorrow<'a> = super::super::super::super::exports::wasmledger::sql::transaction::TransactionBorrow<
                            'a,
                        >;
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_async_begin_transaction_cabi<T: Guest>() -> i32 {
                            unsafe {
                                wit_bindgen::rt::async_support::start_task(async move {
                                    let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                    let result0 = &{ T::begin_transaction().await };
                                    let (result6_0, result6_1, result6_2, result6_3) = match result0 {
                                        Ok(e) => {
                                            (
                                                0i32,
                                                (e).take_handle() as i32,
                                                ::core::ptr::null_mut(),
                                                0usize,
                                            )
                                        }
                                        Err(e) => {
                                            use super::super::super::super::wasmledger::sql::util_types::Error as V4;
                                            let (result5_0, result5_1, result5_2) = match e {
                                                V4::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                                V4::Encode(e) => {
                                                    let vec1 = e;
                                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                                    let len1 = vec1.len();
                                                    (1i32, ptr1.cast_mut(), len1)
                                                }
                                                V4::Decode(e) => {
                                                    let vec2 = e;
                                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                                    let len2 = vec2.len();
                                                    (2i32, ptr2.cast_mut(), len2)
                                                }
                                                V4::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                                V4::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                                V4::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                                V4::Unexpected(e) => {
                                                    let vec3 = e;
                                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                                    let len3 = vec3.len();
                                                    (6i32, ptr3.cast_mut(), len3)
                                                }
                                            };
                                            (1i32, result5_0, result5_1, result5_2)
                                        }
                                    };
                                    unsafe extern "C" fn wit_import7(
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
                                    wit_import7(result6_0, result6_1, result6_2, result6_3);
                                })
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __callback_async_begin_transaction(
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
                            async fn begin_transaction() -> Result<Transaction, Error>;
                        }
                        #[doc(hidden)]
                        pub(crate) use __export_wasmledger_sql_pool_cabi;
                    }
                    #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                    pub mod query_types {
                        #[used]
                        #[doc(hidden)]
                        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                        use super::super::super::super::_rt;
                        pub type SqlString = _rt::String;
                        #[repr(transparent)]
                        pub struct SqlArguments {
                            handle: _rt::Resource<SqlArguments>,
                        }
                        #[automatically_derived]
                        impl ::core::fmt::Debug for SqlArguments {
                            #[inline]
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field1_finish(
                                    f,
                                    "SqlArguments",
                                    "handle",
                                    &&self.handle,
                                )
                            }
                        }
                        type _SqlArgumentsRep<T> = Option<T>;
                        impl SqlArguments {
                            /// Creates a new resource from the specified representation.
                            ///
                            /// This function will create a new resource handle by moving `val` onto
                            /// the heap and then passing that heap pointer to the component model to
                            /// create a handle. The owned handle is then returned as `SqlArguments`.
                            pub fn new<T: GuestSqlArguments>(val: T) -> Self {
                                Self::type_guard::<T>();
                                let val: _SqlArgumentsRep<T> = Some(val);
                                let ptr: *mut _SqlArgumentsRep<T> = _rt::Box::into_raw(
                                    _rt::Box::new(val),
                                );
                                unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                            }
                            /// Gets access to the underlying `T` which represents this resource.
                            pub fn get<T: GuestSqlArguments>(&self) -> &T {
                                let ptr = unsafe { &*self.as_ptr::<T>() };
                                ptr.as_ref().unwrap()
                            }
                            /// Gets mutable access to the underlying `T` which represents this
                            /// resource.
                            pub fn get_mut<T: GuestSqlArguments>(&mut self) -> &mut T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.as_mut().unwrap()
                            }
                            /// Consumes this resource and returns the underlying `T`.
                            pub fn into_inner<T: GuestSqlArguments>(self) -> T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.take().unwrap()
                            }
                            #[doc(hidden)]
                            pub unsafe fn from_handle(handle: u32) -> Self {
                                Self {
                                    handle: unsafe { _rt::Resource::from_handle(handle) },
                                }
                            }
                            #[doc(hidden)]
                            pub fn take_handle(&self) -> u32 {
                                _rt::Resource::take_handle(&self.handle)
                            }
                            #[doc(hidden)]
                            pub fn handle(&self) -> u32 {
                                _rt::Resource::handle(&self.handle)
                            }
                            #[doc(hidden)]
                            fn type_guard<T: 'static>() {
                                use core::any::TypeId;
                                static mut LAST_TYPE: Option<TypeId> = None;
                                unsafe {
                                    if !!false {
                                        ::core::panicking::panic(
                                            "assertion failed: !cfg!(target_feature = \"atomics\")",
                                        )
                                    }
                                    let id = TypeId::of::<T>();
                                    match LAST_TYPE {
                                        Some(ty) => {
                                            if !(ty == id) {
                                                {
                                                    ::core::panicking::panic_fmt(
                                                        format_args!("cannot use two types with this resource type"),
                                                    );
                                                }
                                            }
                                        }
                                        None => LAST_TYPE = Some(id),
                                    }
                                }
                            }
                            #[doc(hidden)]
                            pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                                Self::type_guard::<T>();
                                let _ = unsafe {
                                    _rt::Box::from_raw(handle as *mut _SqlArgumentsRep<T>)
                                };
                            }
                            fn as_ptr<T: GuestSqlArguments>(
                                &self,
                            ) -> *mut _SqlArgumentsRep<T> {
                                SqlArguments::type_guard::<T>();
                                T::_resource_rep(self.handle()).cast()
                            }
                        }
                        /// A borrowed version of [`SqlArguments`] which represents a borrowed value
                        /// with the lifetime `'a`.
                        #[repr(transparent)]
                        pub struct SqlArgumentsBorrow<'a> {
                            rep: *mut u8,
                            _marker: core::marker::PhantomData<&'a SqlArguments>,
                        }
                        #[automatically_derived]
                        impl<'a> ::core::fmt::Debug for SqlArgumentsBorrow<'a> {
                            #[inline]
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field2_finish(
                                    f,
                                    "SqlArgumentsBorrow",
                                    "rep",
                                    &self.rep,
                                    "_marker",
                                    &&self._marker,
                                )
                            }
                        }
                        impl<'a> SqlArgumentsBorrow<'a> {
                            #[doc(hidden)]
                            pub unsafe fn lift(rep: usize) -> Self {
                                Self {
                                    rep: rep as *mut u8,
                                    _marker: core::marker::PhantomData,
                                }
                            }
                            /// Gets access to the underlying `T` in this resource.
                            pub fn get<T: GuestSqlArguments>(&self) -> &T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.as_ref().unwrap()
                            }
                            fn as_ptr<T: 'static>(&self) -> *mut _SqlArgumentsRep<T> {
                                SqlArguments::type_guard::<T>();
                                self.rep.cast()
                            }
                        }
                        unsafe impl _rt::WasmResource for SqlArguments {
                            #[inline]
                            unsafe fn drop(_handle: u32) {
                                unsafe extern "C" fn drop(_: i32) {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe {
                                    drop(_handle as i32);
                                }
                            }
                        }
                        pub struct SqlQuery {
                            pub sql: SqlString,
                            pub args: SqlArguments,
                            pub persistent: Option<bool>,
                        }
                        impl ::core::fmt::Debug for SqlQuery {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter<'_>,
                            ) -> ::core::fmt::Result {
                                f.debug_struct("SqlQuery")
                                    .field("sql", &self.sql)
                                    .field("args", &self.args)
                                    .field("persistent", &self.persistent)
                                    .finish()
                            }
                        }
                        #[repr(transparent)]
                        pub struct QueryResults {
                            handle: _rt::Resource<QueryResults>,
                        }
                        #[automatically_derived]
                        impl ::core::fmt::Debug for QueryResults {
                            #[inline]
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field1_finish(
                                    f,
                                    "QueryResults",
                                    "handle",
                                    &&self.handle,
                                )
                            }
                        }
                        type _QueryResultsRep<T> = Option<T>;
                        impl QueryResults {
                            /// Creates a new resource from the specified representation.
                            ///
                            /// This function will create a new resource handle by moving `val` onto
                            /// the heap and then passing that heap pointer to the component model to
                            /// create a handle. The owned handle is then returned as `QueryResults`.
                            pub fn new<T: GuestQueryResults>(val: T) -> Self {
                                Self::type_guard::<T>();
                                let val: _QueryResultsRep<T> = Some(val);
                                let ptr: *mut _QueryResultsRep<T> = _rt::Box::into_raw(
                                    _rt::Box::new(val),
                                );
                                unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                            }
                            /// Gets access to the underlying `T` which represents this resource.
                            pub fn get<T: GuestQueryResults>(&self) -> &T {
                                let ptr = unsafe { &*self.as_ptr::<T>() };
                                ptr.as_ref().unwrap()
                            }
                            /// Gets mutable access to the underlying `T` which represents this
                            /// resource.
                            pub fn get_mut<T: GuestQueryResults>(&mut self) -> &mut T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.as_mut().unwrap()
                            }
                            /// Consumes this resource and returns the underlying `T`.
                            pub fn into_inner<T: GuestQueryResults>(self) -> T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.take().unwrap()
                            }
                            #[doc(hidden)]
                            pub unsafe fn from_handle(handle: u32) -> Self {
                                Self {
                                    handle: unsafe { _rt::Resource::from_handle(handle) },
                                }
                            }
                            #[doc(hidden)]
                            pub fn take_handle(&self) -> u32 {
                                _rt::Resource::take_handle(&self.handle)
                            }
                            #[doc(hidden)]
                            pub fn handle(&self) -> u32 {
                                _rt::Resource::handle(&self.handle)
                            }
                            #[doc(hidden)]
                            fn type_guard<T: 'static>() {
                                use core::any::TypeId;
                                static mut LAST_TYPE: Option<TypeId> = None;
                                unsafe {
                                    if !!false {
                                        ::core::panicking::panic(
                                            "assertion failed: !cfg!(target_feature = \"atomics\")",
                                        )
                                    }
                                    let id = TypeId::of::<T>();
                                    match LAST_TYPE {
                                        Some(ty) => {
                                            if !(ty == id) {
                                                {
                                                    ::core::panicking::panic_fmt(
                                                        format_args!("cannot use two types with this resource type"),
                                                    );
                                                }
                                            }
                                        }
                                        None => LAST_TYPE = Some(id),
                                    }
                                }
                            }
                            #[doc(hidden)]
                            pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                                Self::type_guard::<T>();
                                let _ = unsafe {
                                    _rt::Box::from_raw(handle as *mut _QueryResultsRep<T>)
                                };
                            }
                            fn as_ptr<T: GuestQueryResults>(
                                &self,
                            ) -> *mut _QueryResultsRep<T> {
                                QueryResults::type_guard::<T>();
                                T::_resource_rep(self.handle()).cast()
                            }
                        }
                        /// A borrowed version of [`QueryResults`] which represents a borrowed value
                        /// with the lifetime `'a`.
                        #[repr(transparent)]
                        pub struct QueryResultsBorrow<'a> {
                            rep: *mut u8,
                            _marker: core::marker::PhantomData<&'a QueryResults>,
                        }
                        #[automatically_derived]
                        impl<'a> ::core::fmt::Debug for QueryResultsBorrow<'a> {
                            #[inline]
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                ::core::fmt::Formatter::debug_struct_field2_finish(
                                    f,
                                    "QueryResultsBorrow",
                                    "rep",
                                    &self.rep,
                                    "_marker",
                                    &&self._marker,
                                )
                            }
                        }
                        impl<'a> QueryResultsBorrow<'a> {
                            #[doc(hidden)]
                            pub unsafe fn lift(rep: usize) -> Self {
                                Self {
                                    rep: rep as *mut u8,
                                    _marker: core::marker::PhantomData,
                                }
                            }
                            /// Gets access to the underlying `T` in this resource.
                            pub fn get<T: GuestQueryResults>(&self) -> &T {
                                let ptr = unsafe { &mut *self.as_ptr::<T>() };
                                ptr.as_ref().unwrap()
                            }
                            fn as_ptr<T: 'static>(&self) -> *mut _QueryResultsRep<T> {
                                QueryResults::type_guard::<T>();
                                self.rep.cast()
                            }
                        }
                        unsafe impl _rt::WasmResource for QueryResults {
                            #[inline]
                            unsafe fn drop(_handle: u32) {
                                unsafe extern "C" fn drop(_: i32) {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe {
                                    drop(_handle as i32);
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_constructor_sql_arguments_cabi<
                            T: GuestSqlArguments,
                        >() -> i32 {
                            unsafe {
                                let result0 = { SqlArguments::new(T::new()) };
                                (result0).take_handle() as i32
                            }
                        }
                        pub trait Guest {
                            type SqlArguments: GuestSqlArguments;
                            type QueryResults: GuestQueryResults;
                        }
                        pub trait GuestSqlArguments: 'static {
                            #[doc(hidden)]
                            unsafe fn _resource_new(val: *mut u8) -> u32
                            where
                                Self: Sized,
                            {
                                unsafe extern "C" fn new(_: *mut u8) -> i32 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe { new(val) as u32 }
                            }
                            #[doc(hidden)]
                            fn _resource_rep(handle: u32) -> *mut u8
                            where
                                Self: Sized,
                            {
                                unsafe extern "C" fn rep(_: i32) -> *mut u8 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe { rep(handle as i32) }
                            }
                            #[allow(async_fn_in_trait)]
                            fn new() -> Self;
                        }
                        pub trait GuestQueryResults: 'static {
                            #[doc(hidden)]
                            unsafe fn _resource_new(val: *mut u8) -> u32
                            where
                                Self: Sized,
                            {
                                unsafe extern "C" fn new(_: *mut u8) -> i32 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe { new(val) as u32 }
                            }
                            #[doc(hidden)]
                            fn _resource_rep(handle: u32) -> *mut u8
                            where
                                Self: Sized,
                            {
                                unsafe extern "C" fn rep(_: i32) -> *mut u8 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                unsafe { rep(handle as i32) }
                            }
                        }
                        #[doc(hidden)]
                        pub(crate) use __export_wasmledger_sql_query_types_cabi;
                    }
                    #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                    pub mod query {
                        #[used]
                        #[doc(hidden)]
                        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                        use super::super::super::super::_rt;
                        pub type SqlQuery = super::super::super::super::exports::wasmledger::sql::query_types::SqlQuery;
                        pub type SqlString = super::super::super::super::exports::wasmledger::sql::query_types::SqlString;
                        pub type QueryResults = super::super::super::super::exports::wasmledger::sql::query_types::QueryResults;
                        pub type QueryResultsBorrow<'a> = super::super::super::super::exports::wasmledger::sql::query_types::QueryResultsBorrow<
                            'a,
                        >;
                        pub type Error = super::super::super::super::wasmledger::sql::util_types::Error;
                        pub type Transaction = super::super::super::super::exports::wasmledger::sql::transaction::Transaction;
                        pub type TransactionBorrow<'a> = super::super::super::super::exports::wasmledger::sql::transaction::TransactionBorrow<
                            'a,
                        >;
                        pub enum QueryExecutor<'a> {
                            Pool,
                            Transaction(TransactionBorrow<'a>),
                        }
                        impl<'a> ::core::fmt::Debug for QueryExecutor<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter<'_>,
                            ) -> ::core::fmt::Result {
                                match self {
                                    QueryExecutor::Pool => {
                                        f.debug_tuple("QueryExecutor::Pool").finish()
                                    }
                                    QueryExecutor::Transaction(e) => {
                                        f.debug_tuple("QueryExecutor::Transaction")
                                            .field(e)
                                            .finish()
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_async_fetch_all_cabi<T: Guest>(
                            arg0: *mut u8,
                            arg1: usize,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                            arg6: i32,
                        ) -> i32 {
                            unsafe {
                                wit_bindgen::rt::async_support::start_task(async move {
                                    let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                    let result2 = &{
                                        let len0 = arg1;
                                        let bytes0 = _rt::Vec::from_raw_parts(
                                            arg0.cast(),
                                            len0,
                                            len0,
                                        );
                                        let v1 = match arg5 {
                                            0 => QueryExecutor::Pool,
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
                                                let e1 = TransactionBorrow::lift(arg6 as u32 as usize);
                                                QueryExecutor::Transaction(e1)
                                            }
                                        };
                                        T::fetch_all(
                                                super::super::super::super::exports::wasmledger::sql::query_types::SqlQuery {
                                                    sql: _rt::string_lift(bytes0),
                                                    args: super::super::super::super::exports::wasmledger::sql::query_types::SqlArguments::from_handle(
                                                        arg2 as u32,
                                                    ),
                                                    persistent: match arg3 {
                                                        0 => None,
                                                        1 => {
                                                            let e = _rt::bool_lift(arg4 as u8);
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                },
                                                v1,
                                            )
                                            .await
                                    };
                                    let (result8_0, result8_1, result8_2, result8_3) = match result2 {
                                        Ok(e) => {
                                            (
                                                0i32,
                                                (e).take_handle() as i32,
                                                ::core::ptr::null_mut(),
                                                0usize,
                                            )
                                        }
                                        Err(e) => {
                                            use super::super::super::super::wasmledger::sql::util_types::Error as V6;
                                            let (result7_0, result7_1, result7_2) = match e {
                                                V6::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                                V6::Encode(e) => {
                                                    let vec3 = e;
                                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                                    let len3 = vec3.len();
                                                    (1i32, ptr3.cast_mut(), len3)
                                                }
                                                V6::Decode(e) => {
                                                    let vec4 = e;
                                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                                    let len4 = vec4.len();
                                                    (2i32, ptr4.cast_mut(), len4)
                                                }
                                                V6::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                                V6::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                                V6::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                                V6::Unexpected(e) => {
                                                    let vec5 = e;
                                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                                    let len5 = vec5.len();
                                                    (6i32, ptr5.cast_mut(), len5)
                                                }
                                            };
                                            (1i32, result7_0, result7_1, result7_2)
                                        }
                                    };
                                    unsafe extern "C" fn wit_import9(
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
                                    wit_import9(result8_0, result8_1, result8_2, result8_3);
                                })
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __callback_async_fetch_all(
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
                        pub unsafe fn _export_async_execute_cabi<T: Guest>(
                            arg0: *mut u8,
                            arg1: usize,
                            arg2: i32,
                            arg3: i32,
                            arg4: i32,
                            arg5: i32,
                            arg6: i32,
                        ) -> i32 {
                            unsafe {
                                wit_bindgen::rt::async_support::start_task(async move {
                                    let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                    let result2 = &{
                                        let len0 = arg1;
                                        let bytes0 = _rt::Vec::from_raw_parts(
                                            arg0.cast(),
                                            len0,
                                            len0,
                                        );
                                        let v1 = match arg5 {
                                            0 => QueryExecutor::Pool,
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
                                                let e1 = TransactionBorrow::lift(arg6 as u32 as usize);
                                                QueryExecutor::Transaction(e1)
                                            }
                                        };
                                        T::execute(
                                                super::super::super::super::exports::wasmledger::sql::query_types::SqlQuery {
                                                    sql: _rt::string_lift(bytes0),
                                                    args: super::super::super::super::exports::wasmledger::sql::query_types::SqlArguments::from_handle(
                                                        arg2 as u32,
                                                    ),
                                                    persistent: match arg3 {
                                                        0 => None,
                                                        1 => {
                                                            let e = _rt::bool_lift(arg4 as u8);
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                },
                                                v1,
                                            )
                                            .await
                                    };
                                    let (result8_0, result8_1, result8_2, result8_3) = match result2 {
                                        Ok(e) => {
                                            (0i32, _rt::as_i64(e), ::core::ptr::null_mut(), 0usize)
                                        }
                                        Err(e) => {
                                            use super::super::super::super::wasmledger::sql::util_types::Error as V6;
                                            let (result7_0, result7_1, result7_2) = match e {
                                                V6::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                                V6::Encode(e) => {
                                                    let vec3 = e;
                                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                                    let len3 = vec3.len();
                                                    (1i32, ptr3.cast_mut(), len3)
                                                }
                                                V6::Decode(e) => {
                                                    let vec4 = e;
                                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                                    let len4 = vec4.len();
                                                    (2i32, ptr4.cast_mut(), len4)
                                                }
                                                V6::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                                V6::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                                V6::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                                V6::Unexpected(e) => {
                                                    let vec5 = e;
                                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                                    let len5 = vec5.len();
                                                    (6i32, ptr5.cast_mut(), len5)
                                                }
                                            };
                                            (1i32, i64::from(result7_0), result7_1, result7_2)
                                        }
                                    };
                                    unsafe extern "C" fn wit_import9(
                                        _: i32,
                                        _: i64,
                                        _: *mut u8,
                                        _: usize,
                                    ) {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                    _task_cancel.forget();
                                    wit_import9(result8_0, result8_1, result8_2, result8_3);
                                })
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __callback_async_execute(
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
                        pub unsafe fn _export_async_execute_raw_cabi<T: Guest>(
                            arg0: *mut u8,
                            arg1: usize,
                            arg2: i32,
                            arg3: i32,
                        ) -> i32 {
                            unsafe {
                                wit_bindgen::rt::async_support::start_task(async move {
                                    let _task_cancel = wit_bindgen::rt::async_support::TaskCancelOnDrop::new();
                                    let result2 = &{
                                        let len0 = arg1;
                                        let bytes0 = _rt::Vec::from_raw_parts(
                                            arg0.cast(),
                                            len0,
                                            len0,
                                        );
                                        let v1 = match arg2 {
                                            0 => QueryExecutor::Pool,
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
                                                let e1 = TransactionBorrow::lift(arg3 as u32 as usize);
                                                QueryExecutor::Transaction(e1)
                                            }
                                        };
                                        T::execute_raw(_rt::string_lift(bytes0), v1).await
                                    };
                                    let (result8_0, result8_1, result8_2, result8_3) = match result2 {
                                        Ok(_) => (0i32, 0i32, ::core::ptr::null_mut(), 0usize),
                                        Err(e) => {
                                            use super::super::super::super::wasmledger::sql::util_types::Error as V6;
                                            let (result7_0, result7_1, result7_2) = match e {
                                                V6::RowNotFound => (0i32, ::core::ptr::null_mut(), 0usize),
                                                V6::Encode(e) => {
                                                    let vec3 = e;
                                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                                    let len3 = vec3.len();
                                                    (1i32, ptr3.cast_mut(), len3)
                                                }
                                                V6::Decode(e) => {
                                                    let vec4 = e;
                                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                                    let len4 = vec4.len();
                                                    (2i32, ptr4.cast_mut(), len4)
                                                }
                                                V6::PoolTimedOut => (3i32, ::core::ptr::null_mut(), 0usize),
                                                V6::PoolClosed => (4i32, ::core::ptr::null_mut(), 0usize),
                                                V6::BeginFailed => (5i32, ::core::ptr::null_mut(), 0usize),
                                                V6::Unexpected(e) => {
                                                    let vec5 = e;
                                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                                    let len5 = vec5.len();
                                                    (6i32, ptr5.cast_mut(), len5)
                                                }
                                            };
                                            (1i32, result7_0, result7_1, result7_2)
                                        }
                                    };
                                    unsafe extern "C" fn wit_import9(
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
                                    wit_import9(result8_0, result8_1, result8_2, result8_3);
                                })
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __callback_async_execute_raw(
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
                            async fn fetch_all(
                                query: SqlQuery,
                                executor: QueryExecutor<'_>,
                            ) -> Result<QueryResults, Error>;
                            #[allow(async_fn_in_trait)]
                            async fn execute(
                                query: SqlQuery,
                                executor: QueryExecutor<'_>,
                            ) -> Result<u64, Error>;
                            #[allow(async_fn_in_trait)]
                            async fn execute_raw(
                                query: SqlString,
                                executor: QueryExecutor<'_>,
                            ) -> Result<(), Error>;
                        }
                        #[doc(hidden)]
                        pub(crate) use __export_wasmledger_sql_query_cabi;
                    }
                }
            }
        }
        mod _rt {
            #![allow(dead_code, clippy::all)]
            pub use alloc_crate::string::String;
            use core::fmt;
            use core::marker;
            use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
            /// A type which represents a component model resource, either imported or
            /// exported into this component.
            ///
            /// This is a low-level wrapper which handles the lifetime of the resource
            /// (namely this has a destructor). The `T` provided defines the component model
            /// intrinsics that this wrapper uses.
            ///
            /// One of the chief purposes of this type is to provide `Deref` implementations
            /// to access the underlying data when it is owned.
            ///
            /// This type is primarily used in generated code for exported and imported
            /// resources.
            #[repr(transparent)]
            pub struct Resource<T: WasmResource> {
                handle: AtomicU32,
                _marker: marker::PhantomData<T>,
            }
            /// A trait which all wasm resources implement, namely providing the ability to
            /// drop a resource.
            ///
            /// This generally is implemented by generated code, not user-facing code.
            #[allow(clippy::missing_safety_doc)]
            pub unsafe trait WasmResource {
                /// Invokes the `[resource-drop]...` intrinsic.
                unsafe fn drop(handle: u32);
            }
            impl<T: WasmResource> Resource<T> {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    if true {
                        if !(handle != 0 && handle != u32::MAX) {
                            ::core::panicking::panic(
                                "assertion failed: handle != 0 && handle != u32::MAX",
                            )
                        }
                    }
                    Self {
                        handle: AtomicU32::new(handle),
                        _marker: marker::PhantomData,
                    }
                }
                /// Takes ownership of the handle owned by `resource`.
                ///
                /// Note that this ideally would be `into_handle` taking `Resource<T>` by
                /// ownership. The code generator does not enable that in all situations,
                /// unfortunately, so this is provided instead.
                ///
                /// Also note that `take_handle` is in theory only ever called on values
                /// owned by a generated function. For example a generated function might
                /// take `Resource<T>` as an argument but then call `take_handle` on a
                /// reference to that argument. In that sense the dynamic nature of
                /// `take_handle` should only be exposed internally to generated code, not
                /// to user code.
                #[doc(hidden)]
                pub fn take_handle(resource: &Resource<T>) -> u32 {
                    resource.handle.swap(u32::MAX, Relaxed)
                }
                #[doc(hidden)]
                pub fn handle(resource: &Resource<T>) -> u32 {
                    resource.handle.load(Relaxed)
                }
            }
            impl<T: WasmResource> fmt::Debug for Resource<T> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_struct("Resource").field("handle", &self.handle).finish()
                }
            }
            impl<T: WasmResource> Drop for Resource<T> {
                fn drop(&mut self) {
                    unsafe {
                        match self.handle.load(Relaxed) {
                            u32::MAX => {}
                            other => T::drop(other),
                        }
                    }
                }
            }
            pub use alloc_crate::boxed::Box;
            pub use alloc_crate::vec::Vec;
            pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
                if true {
                    String::from_utf8(bytes).unwrap()
                } else {
                    unsafe { String::from_utf8_unchecked(bytes) }
                }
            }
            pub unsafe fn bool_lift(val: u8) -> bool {
                if true {
                    match val {
                        0 => false,
                        1 => true,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("invalid bool discriminant"),
                            );
                        }
                    }
                } else {
                    val != 0
                }
            }
            pub unsafe fn invalid_enum_discriminant<T>() -> T {
                if true {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("invalid enum discriminant"),
                        );
                    }
                } else {
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
            pub fn as_i64<T: AsI64>(t: T) -> i64 {
                t.as_i64()
            }
            pub trait AsI64 {
                fn as_i64(self) -> i64;
            }
            impl<'a, T: Copy + AsI64> AsI64 for &'a T {
                fn as_i64(self) -> i64 {
                    (*self).as_i64()
                }
            }
            impl AsI64 for i64 {
                #[inline]
                fn as_i64(self) -> i64 {
                    self as i64
                }
            }
            impl AsI64 for u64 {
                #[inline]
                fn as_i64(self) -> i64 {
                    self as i64
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
        const _: &[u8] = b"package wasmledger:sql;\n\nworld core {\n  export pool;\n  export transaction;\n  export query;\n  export query-types;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// no rows returned by a query that expected to return at least one row.\n    row-not-found,\n\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  begin-transaction: async func() -> result<transaction, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}";
        use crate::core::bindings::{
            pool::PoolImpl, query::QueryImpl, sql_arguments::SqlArgumentsImpl,
            transaction::TransactionImpl,
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn export_async_static_transaction_commit(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_commit_cabi::<
                        <PoolImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_commit(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_commit(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn export_async_static_transaction_rollback(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_rollback_cabi::<
                        <PoolImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_rollback(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_rollback(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/transaction#[dtor]transaction")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::transaction::Transaction::dtor::<
                            <PoolImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn export_async_begin_transaction() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::_export_async_begin_transaction_cabi::<
                        PoolImpl,
                    >()
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn _callback_async_begin_transaction(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::__callback_async_begin_transaction(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        const _: () = {
            #[unsafe(
                export_name = "wasmledger:sql/query-types#[constructor]sql-arguments"
            )]
            unsafe extern "C" fn export_constructor_sql_arguments() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query_types::_export_constructor_sql_arguments_cabi::<
                        <PoolImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                    >()
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]sql-arguments")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::SqlArguments::dtor::<
                            <PoolImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                        >(rep)
                    }
                }
            };
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]query-results")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::QueryResults::dtor::<
                            <PoolImpl as self::exports::wasmledger::sql::query_types::Guest>::QueryResults,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]fetch-all")]
            unsafe extern "C" fn export_async_fetch_all(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_fetch_all_cabi::<
                        PoolImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]fetch-all"
            )]
            unsafe extern "C" fn _callback_async_fetch_all(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_fetch_all(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]execute")]
            unsafe extern "C" fn export_async_execute(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_cabi::<
                        PoolImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute"
            )]
            unsafe extern "C" fn _callback_async_execute(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn export_async_execute_raw(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_raw_cabi::<
                        PoolImpl,
                    >(arg0, arg1, arg2, arg3)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn _callback_async_execute_raw(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute_raw(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn export_async_static_transaction_commit(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_commit_cabi::<
                        <TransactionImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_commit(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_commit(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn export_async_static_transaction_rollback(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_rollback_cabi::<
                        <TransactionImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_rollback(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_rollback(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/transaction#[dtor]transaction")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::transaction::Transaction::dtor::<
                            <TransactionImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn export_async_begin_transaction() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::_export_async_begin_transaction_cabi::<
                        TransactionImpl,
                    >()
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn _callback_async_begin_transaction(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::__callback_async_begin_transaction(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        const _: () = {
            #[unsafe(
                export_name = "wasmledger:sql/query-types#[constructor]sql-arguments"
            )]
            unsafe extern "C" fn export_constructor_sql_arguments() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query_types::_export_constructor_sql_arguments_cabi::<
                        <TransactionImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                    >()
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]sql-arguments")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::SqlArguments::dtor::<
                            <TransactionImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                        >(rep)
                    }
                }
            };
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]query-results")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::QueryResults::dtor::<
                            <TransactionImpl as self::exports::wasmledger::sql::query_types::Guest>::QueryResults,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]fetch-all")]
            unsafe extern "C" fn export_async_fetch_all(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_fetch_all_cabi::<
                        TransactionImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]fetch-all"
            )]
            unsafe extern "C" fn _callback_async_fetch_all(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_fetch_all(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]execute")]
            unsafe extern "C" fn export_async_execute(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_cabi::<
                        TransactionImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute"
            )]
            unsafe extern "C" fn _callback_async_execute(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn export_async_execute_raw(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_raw_cabi::<
                        TransactionImpl,
                    >(arg0, arg1, arg2, arg3)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn _callback_async_execute_raw(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute_raw(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn export_async_static_transaction_commit(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_commit_cabi::<
                        <QueryImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_commit(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_commit(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn export_async_static_transaction_rollback(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_rollback_cabi::<
                        <QueryImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_rollback(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_rollback(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/transaction#[dtor]transaction")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::transaction::Transaction::dtor::<
                            <QueryImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn export_async_begin_transaction() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::_export_async_begin_transaction_cabi::<
                        QueryImpl,
                    >()
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn _callback_async_begin_transaction(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::__callback_async_begin_transaction(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        const _: () = {
            #[unsafe(
                export_name = "wasmledger:sql/query-types#[constructor]sql-arguments"
            )]
            unsafe extern "C" fn export_constructor_sql_arguments() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query_types::_export_constructor_sql_arguments_cabi::<
                        <QueryImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                    >()
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]sql-arguments")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::SqlArguments::dtor::<
                            <QueryImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                        >(rep)
                    }
                }
            };
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]query-results")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::QueryResults::dtor::<
                            <QueryImpl as self::exports::wasmledger::sql::query_types::Guest>::QueryResults,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]fetch-all")]
            unsafe extern "C" fn export_async_fetch_all(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_fetch_all_cabi::<
                        QueryImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]fetch-all"
            )]
            unsafe extern "C" fn _callback_async_fetch_all(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_fetch_all(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]execute")]
            unsafe extern "C" fn export_async_execute(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_cabi::<
                        QueryImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute"
            )]
            unsafe extern "C" fn _callback_async_execute(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn export_async_execute_raw(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_raw_cabi::<
                        QueryImpl,
                    >(arg0, arg1, arg2, arg3)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn _callback_async_execute_raw(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute_raw(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn export_async_static_transaction_commit(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_commit_cabi::<
                        <SqlArgumentsImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.commit"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_commit(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_commit(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn export_async_static_transaction_rollback(
                arg0: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::_export_async_static_transaction_rollback_cabi::<
                        <SqlArgumentsImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                    >(arg0)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/transaction#[async static]transaction.rollback"
            )]
            unsafe extern "C" fn _callback_async_static_transaction_rollback(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::transaction::__callback_async_static_transaction_rollback(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/transaction#[dtor]transaction")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::transaction::Transaction::dtor::<
                            <SqlArgumentsImpl as self::exports::wasmledger::sql::transaction::Guest>::Transaction,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn export_async_begin_transaction() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::_export_async_begin_transaction_cabi::<
                        SqlArgumentsImpl,
                    >()
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/pool#[async]begin-transaction"
            )]
            unsafe extern "C" fn _callback_async_begin_transaction(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::pool::__callback_async_begin_transaction(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        const _: () = {
            #[unsafe(
                export_name = "wasmledger:sql/query-types#[constructor]sql-arguments"
            )]
            unsafe extern "C" fn export_constructor_sql_arguments() -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query_types::_export_constructor_sql_arguments_cabi::<
                        <SqlArgumentsImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                    >()
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]sql-arguments")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::SqlArguments::dtor::<
                            <SqlArgumentsImpl as self::exports::wasmledger::sql::query_types::Guest>::SqlArguments,
                        >(rep)
                    }
                }
            };
            const _: () = {
                #[doc(hidden)]
                #[unsafe(export_name = "wasmledger:sql/query-types#[dtor]query-results")]
                #[allow(non_snake_case)]
                unsafe extern "C" fn dtor(rep: *mut u8) {
                    unsafe {
                        self::exports::wasmledger::sql::query_types::QueryResults::dtor::<
                            <SqlArgumentsImpl as self::exports::wasmledger::sql::query_types::Guest>::QueryResults,
                        >(rep)
                    }
                }
            };
        };
        const _: () = {
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]fetch-all")]
            unsafe extern "C" fn export_async_fetch_all(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_fetch_all_cabi::<
                        SqlArgumentsImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]fetch-all"
            )]
            unsafe extern "C" fn _callback_async_fetch_all(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_fetch_all(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(export_name = "[async-lift]wasmledger:sql/query#[async]execute")]
            unsafe extern "C" fn export_async_execute(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
                arg4: i32,
                arg5: i32,
                arg6: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_cabi::<
                        SqlArgumentsImpl,
                    >(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute"
            )]
            unsafe extern "C" fn _callback_async_execute(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
            #[unsafe(
                export_name = "[async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn export_async_execute_raw(
                arg0: *mut u8,
                arg1: usize,
                arg2: i32,
                arg3: i32,
            ) -> i32 {
                unsafe {
                    self::exports::wasmledger::sql::query::_export_async_execute_raw_cabi::<
                        SqlArgumentsImpl,
                    >(arg0, arg1, arg2, arg3)
                }
            }
            #[unsafe(
                export_name = "[callback][async-lift]wasmledger:sql/query#[async]execute-raw"
            )]
            unsafe extern "C" fn _callback_async_execute_raw(
                event0: u32,
                event1: u32,
                event2: u32,
            ) -> u32 {
                unsafe {
                    self::exports::wasmledger::sql::query::__callback_async_execute_raw(
                        event0,
                        event1,
                        event2,
                    )
                }
            }
        };
        pub(crate) mod context {
            use sqlx::Pool;
            use crate::sqldb::{SqlDB, SqlDatabase};
            use std::sync::OnceLock;
            pub struct BindingsContext;
            static CTX: OnceLock<SqlDB> = OnceLock::new();
            impl BindingsContext {
                pub fn get_pool() -> &'static Pool<SqlDatabase> {
                    &CTX.get().unwrap().pool
                }
                pub fn init(db: SqlDB) {
                    CTX.set(db);
                }
            }
        }
        pub(crate) mod error {
            use crate::core::bindings::wasmledger::sql::util_types::Error;
            impl From<sqlx::Error> for Error {
                fn from(err: sqlx::Error) -> Self {
                    match err {
                        sqlx::Error::PoolTimedOut => Error::PoolTimedOut,
                        sqlx::Error::PoolClosed => Error::PoolClosed,
                        sqlx::Error::Decode(_) => Error::Decode(err.to_string()),
                        sqlx::Error::Encode(_) => Error::Encode(err.to_string()),
                        sqlx::Error::BeginFailed => Error::BeginFailed,
                        sqlx::Error::RowNotFound => Error::RowNotFound,
                        err => Error::Unexpected(err.to_string()),
                    }
                }
            }
        }
        pub(crate) mod pool {
            use tokio::sync::RwLock;
            use crate::core::bindings::{
                context::BindingsContext, exports::wasmledger::sql::pool::Transaction,
                transaction::TransactionImpl, wasmledger::sql::util_types::Error,
            };
            pub struct PoolImpl;
            impl crate::core::bindings::exports::wasmledger::sql::pool::Guest
            for PoolImpl {
                async fn begin_transaction() -> Result<Transaction, Error> {
                    let pool = BindingsContext::get_pool();
                    let tx = pool.begin().await?;
                    Ok(
                        Transaction::new(TransactionImpl {
                            tx: RwLock::new(tx),
                        }),
                    )
                }
            }
        }
        pub(crate) mod query {
            use sqlx::Executor;
            use sqlx::any::AnyQueryResult;
            use crate::core::bindings::context::BindingsContext;
            use crate::core::bindings::exports::wasmledger::sql::query::QueryExecutor;
            use crate::core::bindings::exports::wasmledger::sql::query_types::{
                QueryResults, SqlQuery, SqlString,
            };
            use crate::core::bindings::query_results::QueryResultsImpl;
            use crate::core::bindings::sql_arguments::SqlArgumentsImpl;
            use crate::core::bindings::transaction::TransactionImpl;
            use crate::core::bindings::wasmledger::sql::util_types::Error;
            pub struct QueryImpl;
            impl crate::core::bindings::exports::wasmledger::sql::query::Guest
            for QueryImpl {
                async fn fetch_all(
                    query: SqlQuery,
                    executor: QueryExecutor<'_>,
                ) -> Result<QueryResults, Error> {
                    let args_impl: SqlArgumentsImpl = query.args.into_inner();
                    let args = args_impl.args.into_inner();
                    let q = sqlx::query_with(&query.sql, args);
                    let results = {
                        match executor {
                            QueryExecutor::Pool => {
                                let pool = BindingsContext::get_pool();
                                pool.fetch_all(q).await
                            }
                            QueryExecutor::Transaction(transaction) => {
                                let tx_impl: &TransactionImpl = transaction.get();
                                let mut tx = tx_impl.tx.write().await;
                                tx.fetch_all(q).await
                            }
                        }
                    }?;
                    Ok(QueryResults::new(QueryResultsImpl { results }))
                }
                async fn execute(
                    query: SqlQuery,
                    executor: QueryExecutor<'_>,
                ) -> Result<u64, Error> {
                    let args_impl: SqlArgumentsImpl = query.args.into_inner();
                    let args = args_impl.args.into_inner();
                    let q = sqlx::query_with(&query.sql, args);
                    let results: AnyQueryResult = {
                        match executor {
                            QueryExecutor::Pool => {
                                let pool = BindingsContext::get_pool();
                                pool.execute(q).await
                            }
                            QueryExecutor::Transaction(transaction) => {
                                let tx_impl: &TransactionImpl = transaction.get();
                                let mut tx = tx_impl.tx.write().await;
                                tx.execute(q).await
                            }
                        }
                    }?
                        .into();
                    Ok(results.rows_affected())
                }
                async fn execute_raw(
                    query: SqlString,
                    executor: QueryExecutor<'_>,
                ) -> Result<(), Error> {
                    let q = sqlx::raw_sql(&query);
                    {
                        match executor {
                            QueryExecutor::Pool => {
                                let pool = BindingsContext::get_pool();
                                pool.execute(q).await
                            }
                            QueryExecutor::Transaction(transaction) => {
                                let tx_impl: &TransactionImpl = transaction.get();
                                let mut tx = tx_impl.tx.write().await;
                                tx.execute(q).await
                            }
                        }
                    }?;
                    Ok(())
                }
            }
        }
        pub(crate) mod query_results {
            use crate::sqldb::SqlDatabase;
            pub(crate) struct QueryResultsImpl {
                pub(crate) results: Vec<<SqlDatabase as sqlx::Database>::Row>,
            }
            impl crate::core::bindings::exports::wasmledger::sql::query_types::GuestQueryResults
            for QueryResultsImpl {}
        }
        pub(crate) mod sql_arguments {
            use crate::sqldb::SqlDatabase;
            use tokio::sync::RwLock;
            pub struct SqlArgumentsImpl<'q> {
                pub(crate) args: RwLock<<SqlDatabase as sqlx::Database>::Arguments<'q>>,
            }
            impl crate::core::bindings::exports::wasmledger::sql::query_types::GuestSqlArguments
            for SqlArgumentsImpl<'static> {
                fn new() -> Self {
                    Self {
                        args: RwLock::new(
                            <SqlDatabase as sqlx::Database>::Arguments::default(),
                        ),
                    }
                }
            }
        }
        pub(crate) mod transaction {
            use std::sync::Arc;
            use tokio::sync::{mpsc, oneshot};
            use sqlx::{Any, pool::PoolConnection, query::Query};
            use tokio::sync::RwLock;
            use crate::{
                core::bindings::{
                    exports::wasmledger::sql::transaction::Transaction,
                    wasmledger::sql::util_types::Error,
                },
                sqldb::SqlDatabase,
            };
            pub struct TransactionImpl<'c> {
                pub(crate) tx: RwLock<sqlx::Transaction<'c, SqlDatabase>>,
            }
            impl crate::core::bindings::exports::wasmledger::sql::transaction::GuestTransaction
            for TransactionImpl<'static> {
                async fn commit(this: Transaction) -> Result<(), Error> {
                    let tx_impl: TransactionImpl = this.into_inner();
                    let tx = tx_impl.tx.into_inner();
                    tx.commit().await?;
                    Ok(())
                }
                async fn rollback(this: Transaction) -> Result<(), Error> {
                    let tx_impl: TransactionImpl = this.into_inner();
                    let tx = tx_impl.tx.into_inner();
                    tx.rollback().await?;
                    Ok(())
                }
            }
        }
    }
}
pub mod postgres {
    mod bindings {
        #[allow(unfulfilled_lint_expectations, unused_imports)]
        use crate::core::bindings::exports::wasmledger::sql::query_types as __with_name0;
        #[allow(unfulfilled_lint_expectations, unused_imports)]
        use crate::core::bindings::wasmledger::sql::util_types as __with_name1;
        #[allow(dead_code, clippy::all)]
        pub mod exports {
            pub mod wasmledger {
                pub mod sql_postgres {
                    #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                    pub mod postgres_codecs {
                        #[used]
                        #[doc(hidden)]
                        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                        use super::super::super::super::_rt;
                        pub type SqlArguments = super::super::super::super::__with_name0::SqlArguments;
                        pub type QueryResults = super::super::super::super::__with_name0::QueryResults;
                        pub type Error = super::super::super::super::__with_name1::Error;
                        pub type PushResult = Result<(), Error>;
                        pub type RowIndex = u64;
                        pub enum ColumnIndex {
                            Index(u64),
                            ColumnName(_rt::String),
                        }
                        #[automatically_derived]
                        impl ::core::clone::Clone for ColumnIndex {
                            #[inline]
                            fn clone(&self) -> ColumnIndex {
                                match self {
                                    ColumnIndex::Index(__self_0) => {
                                        ColumnIndex::Index(::core::clone::Clone::clone(__self_0))
                                    }
                                    ColumnIndex::ColumnName(__self_0) => {
                                        ColumnIndex::ColumnName(
                                            ::core::clone::Clone::clone(__self_0),
                                        )
                                    }
                                }
                            }
                        }
                        impl ::core::fmt::Debug for ColumnIndex {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter<'_>,
                            ) -> ::core::fmt::Result {
                                match self {
                                    ColumnIndex::Index(e) => {
                                        f.debug_tuple("ColumnIndex::Index").field(e).finish()
                                    }
                                    ColumnIndex::ColumnName(e) => {
                                        f.debug_tuple("ColumnIndex::ColumnName").field(e).finish()
                                    }
                                }
                            }
                        }
                        pub type ValuePosition = (RowIndex, ColumnIndex);
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_push_int16_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        ) -> *mut u8 {
                            unsafe {
                                let result1 = {
                                    let handle0;
                                    T::push_int16(
                                        match arg0 {
                                            0 => None,
                                            1 => {
                                                let e = arg1 as i16;
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        {
                                            handle0 = super::super::super::super::__with_name0::SqlArguments::from_handle(
                                                arg2 as u32,
                                            );
                                            &handle0
                                        },
                                    )
                                };
                                let ptr2 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result1 {
                                    Ok(_) => {
                                        *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                                    }
                                    Err(e) => {
                                        *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V6;
                                        match e {
                                            V6::RowNotFound => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V6::Encode(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec3 = (e.into_bytes()).into_boxed_slice();
                                                let ptr3 = vec3.as_ptr().cast::<u8>();
                                                let len3 = vec3.len();
                                                ::core::mem::forget(vec3);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len3;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr3.cast_mut();
                                            }
                                            V6::Decode(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec4 = (e.into_bytes()).into_boxed_slice();
                                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                                let len4 = vec4.len();
                                                ::core::mem::forget(vec4);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len4;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr4.cast_mut();
                                            }
                                            V6::PoolTimedOut => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V6::PoolClosed => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V6::BeginFailed => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V6::Unexpected(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr2
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_push_int16<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_get_int16_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                            arg3: ::core::mem::MaybeUninit<u64>,
                            arg4: usize,
                        ) -> *mut u8 {
                            unsafe {
                                let result3 = {
                                    let handle0;
                                    let v2 = match arg2 {
                                        0 => {
                                            let e2 = arg3.assume_init() as i64 as u64;
                                            ColumnIndex::Index(e2)
                                        }
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
                                                let len1 = arg4;
                                                let bytes1 = _rt::Vec::from_raw_parts(
                                                    arg3.as_ptr().cast::<*mut u8>().read().cast(),
                                                    len1,
                                                    len1,
                                                );
                                                _rt::string_lift(bytes1)
                                            };
                                            ColumnIndex::ColumnName(e2)
                                        }
                                    };
                                    T::get_int16(
                                        {
                                            handle0 = super::super::super::super::__with_name0::QueryResults::from_handle(
                                                arg0 as u32,
                                            );
                                            &handle0
                                        },
                                        (arg1 as u64, v2),
                                    )
                                };
                                let ptr4 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result3 {
                                    Ok(e) => {
                                        *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                                        match e {
                                            Some(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                *ptr4
                                                    .add(2 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<u16>() = (_rt::as_i32(e)) as u16;
                                            }
                                            None => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                        };
                                    }
                                    Err(e) => {
                                        *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V8;
                                        match e {
                                            V8::RowNotFound => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V8::Encode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            V8::Decode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec6 = (e.into_bytes()).into_boxed_slice();
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                ::core::mem::forget(vec6);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len6;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            V8::PoolTimedOut => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V8::PoolClosed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V8::BeginFailed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V8::Unexpected(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec7 = (e.into_bytes()).into_boxed_slice();
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                ::core::mem::forget(vec7);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len7;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr4
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_get_int16<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_push_int32_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i32,
                            arg2: i32,
                        ) -> *mut u8 {
                            unsafe {
                                let result1 = {
                                    let handle0;
                                    T::push_int32(
                                        match arg0 {
                                            0 => None,
                                            1 => {
                                                let e = arg1;
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        {
                                            handle0 = super::super::super::super::__with_name0::SqlArguments::from_handle(
                                                arg2 as u32,
                                            );
                                            &handle0
                                        },
                                    )
                                };
                                let ptr2 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result1 {
                                    Ok(_) => {
                                        *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                                    }
                                    Err(e) => {
                                        *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V6;
                                        match e {
                                            V6::RowNotFound => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V6::Encode(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec3 = (e.into_bytes()).into_boxed_slice();
                                                let ptr3 = vec3.as_ptr().cast::<u8>();
                                                let len3 = vec3.len();
                                                ::core::mem::forget(vec3);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len3;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr3.cast_mut();
                                            }
                                            V6::Decode(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec4 = (e.into_bytes()).into_boxed_slice();
                                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                                let len4 = vec4.len();
                                                ::core::mem::forget(vec4);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len4;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr4.cast_mut();
                                            }
                                            V6::PoolTimedOut => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V6::PoolClosed => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V6::BeginFailed => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V6::Unexpected(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr2
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_push_int32<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_get_int32_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                            arg3: ::core::mem::MaybeUninit<u64>,
                            arg4: usize,
                        ) -> *mut u8 {
                            unsafe {
                                let result3 = {
                                    let handle0;
                                    let v2 = match arg2 {
                                        0 => {
                                            let e2 = arg3.assume_init() as i64 as u64;
                                            ColumnIndex::Index(e2)
                                        }
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
                                                let len1 = arg4;
                                                let bytes1 = _rt::Vec::from_raw_parts(
                                                    arg3.as_ptr().cast::<*mut u8>().read().cast(),
                                                    len1,
                                                    len1,
                                                );
                                                _rt::string_lift(bytes1)
                                            };
                                            ColumnIndex::ColumnName(e2)
                                        }
                                    };
                                    T::get_int32(
                                        {
                                            handle0 = super::super::super::super::__with_name0::QueryResults::from_handle(
                                                arg0 as u32,
                                            );
                                            &handle0
                                        },
                                        (arg1 as u64, v2),
                                    )
                                };
                                let ptr4 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result3 {
                                    Ok(e) => {
                                        *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                                        match e {
                                            Some(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                *ptr4
                                                    .add(4 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<i32>() = _rt::as_i32(e);
                                            }
                                            None => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                        };
                                    }
                                    Err(e) => {
                                        *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V8;
                                        match e {
                                            V8::RowNotFound => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V8::Encode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            V8::Decode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec6 = (e.into_bytes()).into_boxed_slice();
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                ::core::mem::forget(vec6);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len6;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            V8::PoolTimedOut => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V8::PoolClosed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V8::BeginFailed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V8::Unexpected(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec7 = (e.into_bytes()).into_boxed_slice();
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                ::core::mem::forget(vec7);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len7;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr4
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_get_int32<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_push_int64_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                        ) -> *mut u8 {
                            unsafe {
                                let result1 = {
                                    let handle0;
                                    T::push_int64(
                                        match arg0 {
                                            0 => None,
                                            1 => {
                                                let e = arg1;
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        {
                                            handle0 = super::super::super::super::__with_name0::SqlArguments::from_handle(
                                                arg2 as u32,
                                            );
                                            &handle0
                                        },
                                    )
                                };
                                let ptr2 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result1 {
                                    Ok(_) => {
                                        *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                                    }
                                    Err(e) => {
                                        *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V6;
                                        match e {
                                            V6::RowNotFound => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V6::Encode(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec3 = (e.into_bytes()).into_boxed_slice();
                                                let ptr3 = vec3.as_ptr().cast::<u8>();
                                                let len3 = vec3.len();
                                                ::core::mem::forget(vec3);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len3;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr3.cast_mut();
                                            }
                                            V6::Decode(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec4 = (e.into_bytes()).into_boxed_slice();
                                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                                let len4 = vec4.len();
                                                ::core::mem::forget(vec4);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len4;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr4.cast_mut();
                                            }
                                            V6::PoolTimedOut => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V6::PoolClosed => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V6::BeginFailed => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V6::Unexpected(e) => {
                                                *ptr2
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr2
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr2
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr2
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_push_int64<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_get_int64_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                            arg3: ::core::mem::MaybeUninit<u64>,
                            arg4: usize,
                        ) -> *mut u8 {
                            unsafe {
                                let result3 = {
                                    let handle0;
                                    let v2 = match arg2 {
                                        0 => {
                                            let e2 = arg3.assume_init() as i64 as u64;
                                            ColumnIndex::Index(e2)
                                        }
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
                                                let len1 = arg4;
                                                let bytes1 = _rt::Vec::from_raw_parts(
                                                    arg3.as_ptr().cast::<*mut u8>().read().cast(),
                                                    len1,
                                                    len1,
                                                );
                                                _rt::string_lift(bytes1)
                                            };
                                            ColumnIndex::ColumnName(e2)
                                        }
                                    };
                                    T::get_int64(
                                        {
                                            handle0 = super::super::super::super::__with_name0::QueryResults::from_handle(
                                                arg0 as u32,
                                            );
                                            &handle0
                                        },
                                        (arg1 as u64, v2),
                                    )
                                };
                                let ptr4 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result3 {
                                    Ok(e) => {
                                        *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                                        match e {
                                            Some(e) => {
                                                *ptr4.add(8).cast::<u8>() = (1i32) as u8;
                                                *ptr4.add(16).cast::<i64>() = _rt::as_i64(e);
                                            }
                                            None => {
                                                *ptr4.add(8).cast::<u8>() = (0i32) as u8;
                                            }
                                        };
                                    }
                                    Err(e) => {
                                        *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V8;
                                        match e {
                                            V8::RowNotFound => {
                                                *ptr4.add(8).cast::<u8>() = (0i32) as u8;
                                            }
                                            V8::Encode(e) => {
                                                *ptr4.add(8).cast::<u8>() = (1i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr4
                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr4
                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            V8::Decode(e) => {
                                                *ptr4.add(8).cast::<u8>() = (2i32) as u8;
                                                let vec6 = (e.into_bytes()).into_boxed_slice();
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                ::core::mem::forget(vec6);
                                                *ptr4
                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len6;
                                                *ptr4
                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            V8::PoolTimedOut => {
                                                *ptr4.add(8).cast::<u8>() = (3i32) as u8;
                                            }
                                            V8::PoolClosed => {
                                                *ptr4.add(8).cast::<u8>() = (4i32) as u8;
                                            }
                                            V8::BeginFailed => {
                                                *ptr4.add(8).cast::<u8>() = (5i32) as u8;
                                            }
                                            V8::Unexpected(e) => {
                                                *ptr4.add(8).cast::<u8>() = (6i32) as u8;
                                                let vec7 = (e.into_bytes()).into_boxed_slice();
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                ::core::mem::forget(vec7);
                                                *ptr4
                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len7;
                                                *ptr4
                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr4
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_get_int64<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(*arg0.add(8).cast::<u8>());
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_push_string_cabi<T: Guest>(
                            arg0: i32,
                            arg1: *mut u8,
                            arg2: usize,
                            arg3: i32,
                        ) -> *mut u8 {
                            unsafe {
                                let result2 = {
                                    let handle1;
                                    T::push_string(
                                        match arg0 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let len0 = arg2;
                                                    let bytes0 = _rt::Vec::from_raw_parts(
                                                        arg1.cast(),
                                                        len0,
                                                        len0,
                                                    );
                                                    _rt::string_lift(bytes0)
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        {
                                            handle1 = super::super::super::super::__with_name0::SqlArguments::from_handle(
                                                arg3 as u32,
                                            );
                                            &handle1
                                        },
                                    )
                                };
                                let ptr3 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result2 {
                                    Ok(_) => {
                                        *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                                    }
                                    Err(e) => {
                                        *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V7;
                                        match e {
                                            V7::RowNotFound => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V7::Encode(e) => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec4 = (e.into_bytes()).into_boxed_slice();
                                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                                let len4 = vec4.len();
                                                ::core::mem::forget(vec4);
                                                *ptr3
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len4;
                                                *ptr3
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr4.cast_mut();
                                            }
                                            V7::Decode(e) => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr3
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr3
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            V7::PoolTimedOut => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V7::PoolClosed => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V7::BeginFailed => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V7::Unexpected(e) => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec6 = (e.into_bytes()).into_boxed_slice();
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                ::core::mem::forget(vec6);
                                                *ptr3
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len6;
                                                *ptr3
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr3
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_push_string<T: Guest>(
                            arg0: *mut u8,
                        ) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_get_string_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                            arg3: ::core::mem::MaybeUninit<u64>,
                            arg4: usize,
                        ) -> *mut u8 {
                            unsafe {
                                let result3 = {
                                    let handle0;
                                    let v2 = match arg2 {
                                        0 => {
                                            let e2 = arg3.assume_init() as i64 as u64;
                                            ColumnIndex::Index(e2)
                                        }
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
                                                let len1 = arg4;
                                                let bytes1 = _rt::Vec::from_raw_parts(
                                                    arg3.as_ptr().cast::<*mut u8>().read().cast(),
                                                    len1,
                                                    len1,
                                                );
                                                _rt::string_lift(bytes1)
                                            };
                                            ColumnIndex::ColumnName(e2)
                                        }
                                    };
                                    T::get_string(
                                        {
                                            handle0 = super::super::super::super::__with_name0::QueryResults::from_handle(
                                                arg0 as u32,
                                            );
                                            &handle0
                                        },
                                        (arg1 as u64, v2),
                                    )
                                };
                                let ptr4 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result3 {
                                    Ok(e) => {
                                        *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                                        match e {
                                            Some(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            None => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                        };
                                    }
                                    Err(e) => {
                                        *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V9;
                                        match e {
                                            V9::RowNotFound => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V9::Encode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec6 = (e.into_bytes()).into_boxed_slice();
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                ::core::mem::forget(vec6);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len6;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            V9::Decode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec7 = (e.into_bytes()).into_boxed_slice();
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                ::core::mem::forget(vec7);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len7;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                            V9::PoolTimedOut => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V9::PoolClosed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V9::BeginFailed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V9::Unexpected(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec8 = (e.into_bytes()).into_boxed_slice();
                                                let ptr8 = vec8.as_ptr().cast::<u8>();
                                                let len8 = vec8.len();
                                                ::core::mem::forget(vec8);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len8;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr8.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr4
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_get_string<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            _ => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                        }
                                    }
                                    _ => {
                                        let l4 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l4 {
                                            0 => {}
                                            1 => {
                                                let l5 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l6 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l5, l6, 1);
                                            }
                                            2 => {
                                                let l7 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l8 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l7, l8, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l9 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l10 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l9, l10, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_push_json_cabi<T: Guest>(
                            arg0: i32,
                            arg1: *mut u8,
                            arg2: usize,
                            arg3: i32,
                        ) -> *mut u8 {
                            unsafe {
                                let result2 = {
                                    let handle1;
                                    T::push_json(
                                        match arg0 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let len0 = arg2;
                                                    let bytes0 = _rt::Vec::from_raw_parts(
                                                        arg1.cast(),
                                                        len0,
                                                        len0,
                                                    );
                                                    _rt::string_lift(bytes0)
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        {
                                            handle1 = super::super::super::super::__with_name0::SqlArguments::from_handle(
                                                arg3 as u32,
                                            );
                                            &handle1
                                        },
                                    )
                                };
                                let ptr3 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result2 {
                                    Ok(_) => {
                                        *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                                    }
                                    Err(e) => {
                                        *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V7;
                                        match e {
                                            V7::RowNotFound => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V7::Encode(e) => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec4 = (e.into_bytes()).into_boxed_slice();
                                                let ptr4 = vec4.as_ptr().cast::<u8>();
                                                let len4 = vec4.len();
                                                ::core::mem::forget(vec4);
                                                *ptr3
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len4;
                                                *ptr3
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr4.cast_mut();
                                            }
                                            V7::Decode(e) => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr3
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr3
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            V7::PoolTimedOut => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V7::PoolClosed => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V7::BeginFailed => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V7::Unexpected(e) => {
                                                *ptr3
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec6 = (e.into_bytes()).into_boxed_slice();
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                ::core::mem::forget(vec6);
                                                *ptr3
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len6;
                                                *ptr3
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr3
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_push_json<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {}
                                    _ => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            1 => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                            2 => {
                                                let l4 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l5 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l4, l5, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l6 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l7 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l6, l7, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case, unused_unsafe)]
                        pub unsafe fn _export_get_json_cabi<T: Guest>(
                            arg0: i32,
                            arg1: i64,
                            arg2: i32,
                            arg3: ::core::mem::MaybeUninit<u64>,
                            arg4: usize,
                        ) -> *mut u8 {
                            unsafe {
                                let result3 = {
                                    let handle0;
                                    let v2 = match arg2 {
                                        0 => {
                                            let e2 = arg3.assume_init() as i64 as u64;
                                            ColumnIndex::Index(e2)
                                        }
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
                                                let len1 = arg4;
                                                let bytes1 = _rt::Vec::from_raw_parts(
                                                    arg3.as_ptr().cast::<*mut u8>().read().cast(),
                                                    len1,
                                                    len1,
                                                );
                                                _rt::string_lift(bytes1)
                                            };
                                            ColumnIndex::ColumnName(e2)
                                        }
                                    };
                                    T::get_json(
                                        {
                                            handle0 = super::super::super::super::__with_name0::QueryResults::from_handle(
                                                arg0 as u32,
                                            );
                                            &handle0
                                        },
                                        (arg1 as u64, v2),
                                    )
                                };
                                let ptr4 = (&raw mut _RET_AREA.0).cast::<u8>();
                                match result3 {
                                    Ok(e) => {
                                        *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                                        match e {
                                            Some(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec5 = (e.into_bytes()).into_boxed_slice();
                                                let ptr5 = vec5.as_ptr().cast::<u8>();
                                                let len5 = vec5.len();
                                                ::core::mem::forget(vec5);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len5;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr5.cast_mut();
                                            }
                                            None => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                        };
                                    }
                                    Err(e) => {
                                        *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::super::__with_name1::Error as V9;
                                        match e {
                                            V9::RowNotFound => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                            V9::Encode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                let vec6 = (e.into_bytes()).into_boxed_slice();
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                ::core::mem::forget(vec6);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len6;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr6.cast_mut();
                                            }
                                            V9::Decode(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (2i32) as u8;
                                                let vec7 = (e.into_bytes()).into_boxed_slice();
                                                let ptr7 = vec7.as_ptr().cast::<u8>();
                                                let len7 = vec7.len();
                                                ::core::mem::forget(vec7);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len7;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr7.cast_mut();
                                            }
                                            V9::PoolTimedOut => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (3i32) as u8;
                                            }
                                            V9::PoolClosed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (4i32) as u8;
                                            }
                                            V9::BeginFailed => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (5i32) as u8;
                                            }
                                            V9::Unexpected(e) => {
                                                *ptr4
                                                    .add(::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (6i32) as u8;
                                                let vec8 = (e.into_bytes()).into_boxed_slice();
                                                let ptr8 = vec8.as_ptr().cast::<u8>();
                                                let len8 = vec8.len();
                                                ::core::mem::forget(vec8);
                                                *ptr4
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>() = len8;
                                                *ptr4
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>() = ptr8.cast_mut();
                                            }
                                        }
                                    }
                                };
                                ptr4
                            }
                        }
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        pub unsafe fn __post_return_get_json<T: Guest>(arg0: *mut u8) {
                            unsafe {
                                let l0 = i32::from(*arg0.add(0).cast::<u8>());
                                match l0 {
                                    0 => {
                                        let l1 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l1 {
                                            0 => {}
                                            _ => {
                                                let l2 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l3 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l2, l3, 1);
                                            }
                                        }
                                    }
                                    _ => {
                                        let l4 = i32::from(
                                            *arg0.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l4 {
                                            0 => {}
                                            1 => {
                                                let l5 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l6 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l5, l6, 1);
                                            }
                                            2 => {
                                                let l7 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l8 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l7, l8, 1);
                                            }
                                            3 => {}
                                            4 => {}
                                            5 => {}
                                            _ => {
                                                let l9 = *arg0
                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<*mut u8>();
                                                let l10 = *arg0
                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<usize>();
                                                _rt::cabi_dealloc(l9, l10, 1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        pub trait Guest {
                            #[allow(async_fn_in_trait)]
                            fn push_int16(
                                value: Option<i16>,
                                to: &SqlArguments,
                            ) -> PushResult;
                            #[allow(async_fn_in_trait)]
                            fn get_int16(
                                result: &QueryResults,
                                position: ValuePosition,
                            ) -> Result<Option<i16>, Error>;
                            #[allow(async_fn_in_trait)]
                            fn push_int32(
                                value: Option<i32>,
                                to: &SqlArguments,
                            ) -> PushResult;
                            #[allow(async_fn_in_trait)]
                            fn get_int32(
                                result: &QueryResults,
                                position: ValuePosition,
                            ) -> Result<Option<i32>, Error>;
                            #[allow(async_fn_in_trait)]
                            fn push_int64(
                                value: Option<i64>,
                                to: &SqlArguments,
                            ) -> PushResult;
                            #[allow(async_fn_in_trait)]
                            fn get_int64(
                                result: &QueryResults,
                                position: ValuePosition,
                            ) -> Result<Option<i64>, Error>;
                            #[allow(async_fn_in_trait)]
                            fn push_string(
                                value: Option<_rt::String>,
                                to: &SqlArguments,
                            ) -> PushResult;
                            #[allow(async_fn_in_trait)]
                            fn get_string(
                                result: &QueryResults,
                                position: ValuePosition,
                            ) -> Result<Option<_rt::String>, Error>;
                            #[allow(async_fn_in_trait)]
                            fn push_json(
                                value: Option<_rt::String>,
                                to: &SqlArguments,
                            ) -> PushResult;
                            #[allow(async_fn_in_trait)]
                            fn get_json(
                                result: &QueryResults,
                                position: ValuePosition,
                            ) -> Result<Option<_rt::String>, Error>;
                        }
                        #[doc(hidden)]
                        pub(crate) use __export_wasmledger_sql_postgres_postgres_codecs_cabi;
                        #[repr(align(8))]
                        struct _RetArea(
                            [::core::mem::MaybeUninit<
                                u8,
                            >; 16 + 2 * ::core::mem::size_of::<*const u8>()],
                        );
                        static mut _RET_AREA: _RetArea = _RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16
                                + 2 * ::core::mem::size_of::<*const u8>()],
                        );
                    }
                }
            }
        }
        mod _rt {
            #![allow(dead_code, clippy::all)]
            pub use alloc_crate::string::String;
            pub unsafe fn invalid_enum_discriminant<T>() -> T {
                if true {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("invalid enum discriminant"),
                        );
                    }
                } else {
                    unsafe { core::hint::unreachable_unchecked() }
                }
            }
            pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
                if size == 0 {
                    return;
                }
                unsafe {
                    let layout = alloc::Layout::from_size_align_unchecked(size, align);
                    alloc::dealloc(ptr, layout);
                }
            }
            pub use alloc_crate::vec::Vec;
            pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
                if true {
                    String::from_utf8(bytes).unwrap()
                } else {
                    unsafe { String::from_utf8_unchecked(bytes) }
                }
            }
            pub fn as_i32<T: AsI32>(t: T) -> i32 {
                t.as_i32()
            }
            pub trait AsI32 {
                fn as_i32(self) -> i32;
            }
            impl<'a, T: Copy + AsI32> AsI32 for &'a T {
                fn as_i32(self) -> i32 {
                    (*self).as_i32()
                }
            }
            impl AsI32 for i32 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for u32 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for i16 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for u16 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for i8 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for u8 {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for char {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            impl AsI32 for usize {
                #[inline]
                fn as_i32(self) -> i32 {
                    self as i32
                }
            }
            pub fn as_i64<T: AsI64>(t: T) -> i64 {
                t.as_i64()
            }
            pub trait AsI64 {
                fn as_i64(self) -> i64;
            }
            impl<'a, T: Copy + AsI64> AsI64 for &'a T {
                fn as_i64(self) -> i64 {
                    (*self).as_i64()
                }
            }
            impl AsI64 for i64 {
                #[inline]
                fn as_i64(self) -> i64 {
                    self as i64
                }
            }
            impl AsI64 for u64 {
                #[inline]
                fn as_i64(self) -> i64 {
                    self as i64
                }
            }
            extern crate alloc as alloc_crate;
            pub use alloc_crate::alloc;
        }
        #[doc(inline)]
        pub(crate) use __export_postgres_impl as export;
        #[inline(never)]
        #[doc(hidden)]
        pub fn __link_custom_section_describing_imports() {
            wit_bindgen::rt::maybe_link_cabi_realloc();
        }
        const _: &[u8] = b"package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// no rows returned by a query that expected to return at least one row.\n    row-not-found,\n\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\nworld core {\n  export pool;\n  export transaction;\n  export query;\n  export query-types;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  begin-transaction: async func() -> result<transaction, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
        const _: &[u8] = b"// package wasmledger:sql-postgres;\n\n// interface postgres-codecs-ext {\n//   type hstore = list<tuple<string, option<string>>>;\n\n//   hstore-new: func(value: option<hstore>) -> sql-param;\n//   hstore-get: func(row: borrow<row-result>) -> result<option<hstore>, error>;\n// }";
        const _: &[u8] = b"package wasmledger:sql-postgres;\n\nworld postgres {\n  export postgres-codecs;\n  // export postgres-codecs-ext;\n}";
        const _: &[u8] = b"package wasmledger:sql-postgres;\n\ninterface postgres-codecs {\n  use wasmledger:sql/query-types.{sql-arguments, query-results};\n  use wasmledger:sql/util-types.{error};\n\n  type push-result = result<_, error>;\n\n  type row-index = u64;\n\n  variant column-index {\n    index(u64),\n    column-name(string)\n  }\n\n  type value-position = tuple<row-index, column-index>;\n\n  push-int16: func(value: option<s16>, to: borrow<sql-arguments>) -> push-result;\n  get-int16: func(%result: borrow<query-results>, position: value-position) -> result<option<s16>, error>;\n\n  push-int32: func(value: option<s32>, to: borrow<sql-arguments>) -> push-result;\n  get-int32: func(%result: borrow<query-results>, position: value-position) -> result<option<s32>, error>;\n\n  push-int64: func(value: option<s64>, to: borrow<sql-arguments>) -> push-result;\n  get-int64: func(%result: borrow<query-results>, position: value-position) -> result<option<s64>, error>;\n\n  push-string: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-string: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n  push-json: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-json: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n\n  // variant pg-value {\n  //   /// SQL: NULL\n  //   null,\n\n  //   /// SQL: BIGINT, INT8\n  //   int64(s64),\n  //   /// SQL: BIGINT[], INT8[]\n  //   int64-array(list<s64>),\n\n  //   /// SQL: INTEGER, INT, INT4\n  //   int32(s32),\n  //   /// SQL: INTEGER[], INT4[]\n  //   int32-array(list<s32>),\n\n  //   /// SQL: SMALLINT, INT2\n  //   int2(s16),\n  //   /// SQL: SMALLINT[], INT2[]\n  //   int2-array(list<s16>),\n\n  //   /// SQL: DOUBLE PRECISION, FLOAT8\n  //   float8(hashable-f64),\n  //   /// SQL: DOUBLE PRECISION[], FLOAT8[]\n  //   float8-array(list<hashable-f64>),\n\n  //   /// SQL: REAL, FLOAT4\n  //   float4(hashable-f32),\n  //   /// SQL: REAL[], FLOAT4[]\n  //   float4-array(list<hashable-f32>),\n\n  //   /// SQL: BOOLEAN, BOOL\n  //   %bool(bool),\n  //   /// SQL: BOOLEAN[], BOOL[]\n  //   %bool-array(list<bool>),\n\n  //   /// SQL: NUMERIC, DECIMAL\n  //   numeric(numeric),\n  //   /// SQL: NUMERIC[], DECIMAL[]\n  //   numeric-array(list<numeric>),\n\n  //   /// SQL: BIT(n)\n  //   bit(list<u8>),\n  //   /// SQL: BIT(n)[]\n  //   bit-array(list<list<u8>>),\n\n  //   /// SQL: VARBIT(n)\n  //   varbit(list<u8>),\n  //   /// SQL: BIT VARYING[], VARBIT[]\n  //   varbit-array(list<list<u8>>),\n\n  //   /// SQL: BYTEA\n  //   bytea(list<u8>),\n  //   /// SQL: BYTEA[]\n  //   bytea-array(list<list<u8>>),\n\n  //   /// SQL: CHAR(n), CHARACTER(n)\n  //   %char(string),\n  //   /// SQL: CHAR(n)[]\n  //   %char-array(list<string>),\n\n  //   /// SQL: VARCHAR(n), CHARACTER VARYING(n)\n  //   varchar(string),\n  //   /// SQL: VARCHAR(n)[]\n  //   varchar-array(list<string>),\n\n  //   // Networking\n  //   /// SQL: CIDR\n  //   cidr(string),\n  //   /// SQL: CIDR[]\n  //   cidr-array(list<string>),\n\n  //   /// SQL: INET\n  //   inet(string),\n  //   /// SQL: INET[]\n  //   inet-array(list<string>),\n\n  //   /// SQL: MACADDR (EUI-48)\n  //   macaddr(mac-address-eui48),\n  //   /// SQL: MACADDR[]\n  //   macaddr-array(list<mac-address-eui48>),\n\n  //   /// SQL: MACADDR8 (EUI-64, deprecated)\n  //   macaddr8(mac-address-eui64),\n  //   /// SQL: MACADDR8[]\n  //   macaddr8-array(list<mac-address-eui64>),\n\n  //   // Date-time\n  //   /// SQL: DATE\n  //   date(date),\n  //   /// SQL: DATE[]\n  //   date-array(list<date>),\n\n  //   /// SQL: INTERVAL\n  //   interval(interval),\n  //   /// SQL: INTERVAL[]\n  //   interval-array(list<interval>),\n\n  //   /// SQL: TIME WITHOUT TIME ZONE\n  //   time(time),\n  //   /// SQL: TIME[]\n  //   time-array(list<time>),\n\n  //   /// SQL: TIME WITH TIME ZONE\n  //   time-tz(time-tz),\n  //   /// SQL: TIMETZ[]\n  //   time-tz-array(list<time-tz>),\n\n  //   /// SQL: TIMESTAMP WITHOUT TIME ZONE\n  //   timestamp(timestamp),\n  //   /// SQL: TIMESTAMP[]\n  //   timestamp-array(list<timestamp>),\n\n  //   /// SQL: TIMESTAMP WITH TIME ZONE, TIMESTAMPTZ\n  //   timestamp-tz(timestamp-tz),\n  //   /// SQL: TIMESTAMPTZ[]\n  //   timestamp-tz-array(list<timestamp-tz>),\n\n  //   // JSON\n  //   /// SQL: JSON\n  //   json(string),\n  //   /// SQL: JSON[]\n  //   json-array(list<string>),\n\n  //   /// SQL: JSONB\n  //   jsonb(string),\n  //   /// SQL: JSONB[]\n  //   jsonb-array(list<string>),\n\n  //   /// SQL: MONEY (internal fixed-point type)\n  //   money(numeric),\n  //   /// SQL: MONEY[]\n  //   money-array(list<numeric>),\n\n  //   // Text\n  //   /// SQL: NAME (system identifier type)\n  //   name(string),\n  //   /// SQL: NAME[]\n  //   name-array(list<string>),\n\n  //   /// SQL: TEXT\n  //   text(string),\n  //   /// SQL: TEXT[]\n  //   text-array(list<string>),\n\n  //   /// SQL: XML\n  //   xml(string),\n  //   /// SQL: XML[]\n  //   xml-array(list<string>),\n\n  //   // UUIDs\n  //   /// SQL: UUID\n  //   uuid(string),\n  //   /// SQL: UUID[]\n  //   uuid-array(list<string>),\n\n  //   // Containers\n  //   /// SQL: HSTORE (extension)\n  //   hstore(list<tuple<string, option<string>>>),\n  // }\n}";
        mod codecs {
            use std::num::TryFromIntError;
            use crate::{
                core::bindings::{
                    exports::wasmledger::sql::{
                        query::QueryResults, query_types::SqlArguments,
                    },
                    query_results::QueryResultsImpl, sql_arguments::SqlArgumentsImpl,
                    wasmledger::sql::util_types::Error,
                },
                postgres::bindings::exports::wasmledger::sql_postgres::postgres_codecs::{
                    ColumnIndex, PushResult, ValuePosition,
                },
            };
            use sqlx::types::Json;
            use sqlx::{Arguments, Encode, Postgres, Type, types::JsonRawValue};
            use sqlx::{Decode, Row};
            pub struct CodecsImpl;
            impl CodecsImpl {
                fn _encode<T>(value: T, to: &SqlArguments) -> PushResult
                where
                    T: 'static + Encode<'static, Postgres> + Type<Postgres>,
                {
                    let _impl: &SqlArgumentsImpl = to.get();
                    let mut args = _impl.args.blocking_write();
                    args.add(value).map_err(|e| Error::Encode(e.to_string()))
                }
                fn _decode<'a, R>(
                    result: &'a QueryResults,
                    position: ValuePosition,
                ) -> Result<R, Error>
                where
                    R: Decode<'a, Postgres> + Type<Postgres>,
                {
                    let _impl: &QueryResultsImpl = result.get();
                    let row_index = Self::_index_to_usize(position.0)?;
                    let row = _impl.results.get(row_index);
                    match row {
                        None => Err(Error::Decode("row not found".to_string())),
                        Some(row) => {
                            match position.1 {
                                ColumnIndex::Index(i) => {
                                    let i = Self::_index_to_usize(i)?;
                                    let r: R = row.try_get(i)?;
                                    Ok(r)
                                }
                                ColumnIndex::ColumnName(name) => {
                                    let r: R = row.try_get(name.as_str())?;
                                    Ok(r)
                                }
                            }
                        }
                    }
                }
                fn _index_to_usize(index: u64) -> Result<usize, Error> {
                    index
                        .try_into()
                        .map_err(|e: TryFromIntError| Error::Decode(e.to_string()))
                }
            }
            impl crate::postgres::bindings::exports::wasmledger::sql_postgres::postgres_codecs::Guest
            for CodecsImpl {
                fn push_int16(value: Option<i16>, to: &SqlArguments) -> PushResult {
                    Self::_encode(value, to)
                }
                fn push_int32(value: Option<i32>, to: &SqlArguments) -> PushResult {
                    Self::_encode(value, to)
                }
                fn push_int64(value: Option<i64>, to: &SqlArguments) -> PushResult {
                    Self::_encode(value, to)
                }
                fn push_string(value: Option<String>, to: &SqlArguments) -> PushResult {
                    Self::_encode(value, to)
                }
                fn push_json(value: Option<String>, to: &SqlArguments) -> PushResult {
                    match value {
                        Some(value) => {
                            let raw_value = JsonRawValue::from_string(value)
                                .map_err(|e| Error::Encode(e.to_string()))?;
                            Self::_encode(Json(raw_value), to)
                        }
                        None => Self::_encode(value, to),
                    }
                }
                fn get_int16(
                    result: &QueryResults,
                    position: ValuePosition,
                ) -> Result<Option<i16>, Error> {
                    Self::_decode(result, position)
                }
                fn get_int32(
                    result: &QueryResults,
                    position: ValuePosition,
                ) -> Result<Option<i32>, Error> {
                    Self::_decode(result, position)
                }
                fn get_int64(
                    result: &QueryResults,
                    position: ValuePosition,
                ) -> Result<Option<i64>, Error> {
                    Self::_decode(result, position)
                }
                fn get_string(
                    result: &QueryResults,
                    position: ValuePosition,
                ) -> Result<Option<String>, Error> {
                    Self::_decode(result, position)
                }
                fn get_json(
                    result: &QueryResults,
                    position: ValuePosition,
                ) -> Result<Option<String>, Error> {
                    let a = Self::_decode::<Option<&JsonRawValue>>(result, position)?;
                    Ok(a.map(|x| x.get().to_string()))
                }
            }
        }
    }
}

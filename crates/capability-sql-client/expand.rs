#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod base {
    use crate::base::bindings::wasmledger::sql::{
        codecs::{ColumnIndex, ValuePosition},
        query_types::QueryResults,
    };
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
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod transaction {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
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
                    impl Transaction {
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
                    impl Transaction {
                        #[allow(unused_unsafe, clippy::all)]
                        #[allow(async_fn_in_trait)]
                        pub async fn commit(this: Transaction) -> Result<(), Error> {
                            unsafe {
                                use wit_bindgen::rt::async_support::Subtask as _Subtask;
                                struct _MySubtask<'a> {
                                    _unused: core::marker::PhantomData<&'a ()>,
                                }
                                #[allow(unused_parens)]
                                unsafe impl<'a> _Subtask for _MySubtask<'a> {
                                    type Params = (Transaction,);
                                    type Results = Result<(), Error>;
                                    type ParamsLower = (i32,);
                                    fn abi_layout(&self) -> ::core::alloc::Layout {
                                        unsafe {
                                            ::core::alloc::Layout::from_size_align_unchecked(
                                                (4 * ::core::mem::size_of::<*const u8>()),
                                                ::core::mem::size_of::<*const u8>(),
                                            )
                                        }
                                    }
                                    fn results_offset(&self) -> usize {
                                        0
                                    }
                                    unsafe fn call_import(
                                        &self,
                                        _params: Self::ParamsLower,
                                        _results: *mut u8,
                                    ) -> u32 {
                                        unsafe extern "C" fn call(_: i32, _: *mut u8) -> i32 {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                        unsafe { call(_params.0, _results) as u32 }
                                    }
                                    unsafe fn params_dealloc_lists(
                                        &self,
                                        _params: Self::ParamsLower,
                                    ) {
                                        unsafe {}
                                    }
                                    unsafe fn params_dealloc_lists_and_own(
                                        &self,
                                        _params: Self::ParamsLower,
                                    ) {
                                        unsafe {
                                            let _ = Transaction::from_handle(_params.0 as u32);
                                        }
                                    }
                                    unsafe fn params_lower(
                                        &self,
                                        (_lower0,): Self::Params,
                                        _ptr: *mut u8,
                                    ) -> Self::ParamsLower {
                                        unsafe { ((_lower0).take_handle() as i32,) }
                                    }
                                    unsafe fn results_lift(
                                        &self,
                                        _ptr: *mut u8,
                                    ) -> Self::Results {
                                        unsafe {
                                            let l0 = i32::from(*_ptr.add(0).cast::<u8>());
                                            match l0 {
                                                0 => {
                                                    let e = ();
                                                    Ok(e)
                                                }
                                                1 => {
                                                    let e = {
                                                        let l1 = i32::from(
                                                            *_ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                                        );
                                                        use super::super::super::wasmledger::sql::util_types::Error as V11;
                                                        let v11 = match l1 {
                                                            0 => V11::RowNotFound,
                                                            1 => {
                                                                let e11 = {
                                                                    let l2 = *_ptr
                                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l3 = *_ptr
                                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len4 = l3;
                                                                    let bytes4 = _rt::Vec::from_raw_parts(
                                                                        l2.cast(),
                                                                        len4,
                                                                        len4,
                                                                    );
                                                                    _rt::string_lift(bytes4)
                                                                };
                                                                V11::Encode(e11)
                                                            }
                                                            2 => {
                                                                let e11 = {
                                                                    let l5 = *_ptr
                                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l6 = *_ptr
                                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len7 = l6;
                                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                                        l5.cast(),
                                                                        len7,
                                                                        len7,
                                                                    );
                                                                    _rt::string_lift(bytes7)
                                                                };
                                                                V11::Decode(e11)
                                                            }
                                                            3 => V11::PoolTimedOut,
                                                            4 => V11::PoolClosed,
                                                            5 => V11::BeginFailed,
                                                            n => {
                                                                if true {
                                                                    match (&n, &6) {
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
                                                                let e11 = {
                                                                    let l8 = *_ptr
                                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l9 = *_ptr
                                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len10 = l9;
                                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                                        l8.cast(),
                                                                        len10,
                                                                        len10,
                                                                    );
                                                                    _rt::string_lift(bytes10)
                                                                };
                                                                V11::Unexpected(e11)
                                                            }
                                                        };
                                                        v11
                                                    };
                                                    Err(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        }
                                    }
                                }
                                _MySubtask {
                                    _unused: core::marker::PhantomData,
                                }
                                    .call((this,))
                                    .await
                            }
                        }
                    }
                    impl Transaction {
                        #[allow(unused_unsafe, clippy::all)]
                        #[allow(async_fn_in_trait)]
                        pub async fn rollback(this: Transaction) -> Result<(), Error> {
                            unsafe {
                                use wit_bindgen::rt::async_support::Subtask as _Subtask;
                                struct _MySubtask<'a> {
                                    _unused: core::marker::PhantomData<&'a ()>,
                                }
                                #[allow(unused_parens)]
                                unsafe impl<'a> _Subtask for _MySubtask<'a> {
                                    type Params = (Transaction,);
                                    type Results = Result<(), Error>;
                                    type ParamsLower = (i32,);
                                    fn abi_layout(&self) -> ::core::alloc::Layout {
                                        unsafe {
                                            ::core::alloc::Layout::from_size_align_unchecked(
                                                (4 * ::core::mem::size_of::<*const u8>()),
                                                ::core::mem::size_of::<*const u8>(),
                                            )
                                        }
                                    }
                                    fn results_offset(&self) -> usize {
                                        0
                                    }
                                    unsafe fn call_import(
                                        &self,
                                        _params: Self::ParamsLower,
                                        _results: *mut u8,
                                    ) -> u32 {
                                        unsafe extern "C" fn call(_: i32, _: *mut u8) -> i32 {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        }
                                        unsafe { call(_params.0, _results) as u32 }
                                    }
                                    unsafe fn params_dealloc_lists(
                                        &self,
                                        _params: Self::ParamsLower,
                                    ) {
                                        unsafe {}
                                    }
                                    unsafe fn params_dealloc_lists_and_own(
                                        &self,
                                        _params: Self::ParamsLower,
                                    ) {
                                        unsafe {
                                            let _ = Transaction::from_handle(_params.0 as u32);
                                        }
                                    }
                                    unsafe fn params_lower(
                                        &self,
                                        (_lower0,): Self::Params,
                                        _ptr: *mut u8,
                                    ) -> Self::ParamsLower {
                                        unsafe { ((_lower0).take_handle() as i32,) }
                                    }
                                    unsafe fn results_lift(
                                        &self,
                                        _ptr: *mut u8,
                                    ) -> Self::Results {
                                        unsafe {
                                            let l0 = i32::from(*_ptr.add(0).cast::<u8>());
                                            match l0 {
                                                0 => {
                                                    let e = ();
                                                    Ok(e)
                                                }
                                                1 => {
                                                    let e = {
                                                        let l1 = i32::from(
                                                            *_ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                                        );
                                                        use super::super::super::wasmledger::sql::util_types::Error as V11;
                                                        let v11 = match l1 {
                                                            0 => V11::RowNotFound,
                                                            1 => {
                                                                let e11 = {
                                                                    let l2 = *_ptr
                                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l3 = *_ptr
                                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len4 = l3;
                                                                    let bytes4 = _rt::Vec::from_raw_parts(
                                                                        l2.cast(),
                                                                        len4,
                                                                        len4,
                                                                    );
                                                                    _rt::string_lift(bytes4)
                                                                };
                                                                V11::Encode(e11)
                                                            }
                                                            2 => {
                                                                let e11 = {
                                                                    let l5 = *_ptr
                                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l6 = *_ptr
                                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len7 = l6;
                                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                                        l5.cast(),
                                                                        len7,
                                                                        len7,
                                                                    );
                                                                    _rt::string_lift(bytes7)
                                                                };
                                                                V11::Decode(e11)
                                                            }
                                                            3 => V11::PoolTimedOut,
                                                            4 => V11::PoolClosed,
                                                            5 => V11::BeginFailed,
                                                            n => {
                                                                if true {
                                                                    match (&n, &6) {
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
                                                                let e11 = {
                                                                    let l8 = *_ptr
                                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<*mut u8>();
                                                                    let l9 = *_ptr
                                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                                        .cast::<usize>();
                                                                    let len10 = l9;
                                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                                        l8.cast(),
                                                                        len10,
                                                                        len10,
                                                                    );
                                                                    _rt::string_lift(bytes10)
                                                                };
                                                                V11::Unexpected(e11)
                                                            }
                                                        };
                                                        v11
                                                    };
                                                    Err(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        }
                                    }
                                }
                                _MySubtask {
                                    _unused: core::marker::PhantomData,
                                }
                                    .call((this,))
                                    .await
                            }
                        }
                    }
                }
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod pool {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
                    pub type Transaction = super::super::super::wasmledger::sql::transaction::Transaction;
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub async fn begin_transaction() -> Result<Transaction, Error> {
                        unsafe {
                            use wit_bindgen::rt::async_support::Subtask as _Subtask;
                            struct _MySubtask<'a> {
                                _unused: core::marker::PhantomData<&'a ()>,
                            }
                            #[allow(unused_parens)]
                            unsafe impl<'a> _Subtask for _MySubtask<'a> {
                                type Params = ();
                                type Results = Result<Transaction, Error>;
                                type ParamsLower = ();
                                fn abi_layout(&self) -> ::core::alloc::Layout {
                                    unsafe {
                                        ::core::alloc::Layout::from_size_align_unchecked(
                                            (4 * ::core::mem::size_of::<*const u8>()),
                                            ::core::mem::size_of::<*const u8>(),
                                        )
                                    }
                                }
                                fn results_offset(&self) -> usize {
                                    0
                                }
                                unsafe fn call_import(
                                    &self,
                                    _params: Self::ParamsLower,
                                    _results: *mut u8,
                                ) -> u32 {
                                    unsafe extern "C" fn call(_: *mut u8) -> i32 {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                    unsafe { call(_results) as u32 }
                                }
                                unsafe fn params_dealloc_lists(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {}
                                }
                                unsafe fn params_dealloc_lists_and_own(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {}
                                }
                                unsafe fn params_lower(
                                    &self,
                                    (): Self::Params,
                                    _ptr: *mut u8,
                                ) -> Self::ParamsLower {
                                    unsafe { () }
                                }
                                unsafe fn results_lift(
                                    &self,
                                    _ptr: *mut u8,
                                ) -> Self::Results {
                                    unsafe {
                                        let l0 = i32::from(*_ptr.add(0).cast::<u8>());
                                        match l0 {
                                            0 => {
                                                let e = {
                                                    let l1 = *_ptr
                                                        .add(::core::mem::size_of::<*const u8>())
                                                        .cast::<i32>();
                                                    super::super::super::wasmledger::sql::transaction::Transaction::from_handle(
                                                        l1 as u32,
                                                    )
                                                };
                                                Ok(e)
                                            }
                                            1 => {
                                                let e = {
                                                    let l2 = i32::from(
                                                        *_ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                                    );
                                                    use super::super::super::wasmledger::sql::util_types::Error as V12;
                                                    let v12 = match l2 {
                                                        0 => V12::RowNotFound,
                                                        1 => {
                                                            let e12 = {
                                                                let l3 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l4 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len5 = l4;
                                                                let bytes5 = _rt::Vec::from_raw_parts(
                                                                    l3.cast(),
                                                                    len5,
                                                                    len5,
                                                                );
                                                                _rt::string_lift(bytes5)
                                                            };
                                                            V12::Encode(e12)
                                                        }
                                                        2 => {
                                                            let e12 = {
                                                                let l6 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l7 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len8 = l7;
                                                                let bytes8 = _rt::Vec::from_raw_parts(
                                                                    l6.cast(),
                                                                    len8,
                                                                    len8,
                                                                );
                                                                _rt::string_lift(bytes8)
                                                            };
                                                            V12::Decode(e12)
                                                        }
                                                        3 => V12::PoolTimedOut,
                                                        4 => V12::PoolClosed,
                                                        5 => V12::BeginFailed,
                                                        n => {
                                                            if true {
                                                                match (&n, &6) {
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
                                                            let e12 = {
                                                                let l9 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l10 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len11 = l10;
                                                                let bytes11 = _rt::Vec::from_raw_parts(
                                                                    l9.cast(),
                                                                    len11,
                                                                    len11,
                                                                );
                                                                _rt::string_lift(bytes11)
                                                            };
                                                            V12::Unexpected(e12)
                                                        }
                                                    };
                                                    v12
                                                };
                                                Err(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    }
                                }
                            }
                            _MySubtask {
                                _unused: core::marker::PhantomData,
                            }
                                .call(())
                                .await
                        }
                    }
                }
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod query_types {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
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
                    impl SqlArguments {
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
                    impl QueryResults {
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
                    impl SqlArguments {
                        #[allow(unused_unsafe, clippy::all)]
                        #[allow(async_fn_in_trait)]
                        pub fn new() -> Self {
                            unsafe {
                                unsafe extern "C" fn wit_import0() -> i32 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                let ret = wit_import0();
                                SqlArguments::from_handle(ret as u32)
                            }
                        }
                    }
                    impl QueryResults {
                        #[allow(unused_unsafe, clippy::all)]
                        #[allow(async_fn_in_trait)]
                        pub fn row_count(&self) -> u64 {
                            unsafe {
                                unsafe extern "C" fn wit_import0(_: i32) -> i64 {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                let ret = wit_import0((self).handle() as i32);
                                ret as u64
                            }
                        }
                    }
                }
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod query {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type SqlQuery = super::super::super::wasmledger::sql::query_types::SqlQuery;
                    pub type SqlString = super::super::super::wasmledger::sql::query_types::SqlString;
                    pub type QueryResults = super::super::super::wasmledger::sql::query_types::QueryResults;
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
                    pub type Transaction = super::super::super::wasmledger::sql::transaction::Transaction;
                    pub enum QueryExecutor<'a> {
                        Pool,
                        Transaction(&'a Transaction),
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
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub async fn fetch_all(
                        query: SqlQuery,
                        executor: QueryExecutor<'_>,
                    ) -> Result<QueryResults, Error> {
                        unsafe {
                            use wit_bindgen::rt::async_support::Subtask as _Subtask;
                            struct _MySubtask<'a> {
                                _unused: core::marker::PhantomData<&'a ()>,
                            }
                            #[allow(unused_parens)]
                            unsafe impl<'a> _Subtask for _MySubtask<'a> {
                                type Params = (SqlQuery, QueryExecutor<'a>);
                                type Results = Result<QueryResults, Error>;
                                type ParamsLower = (*mut u8,);
                                fn abi_layout(&self) -> ::core::alloc::Layout {
                                    unsafe {
                                        ::core::alloc::Layout::from_size_align_unchecked(
                                            (16 + 6 * ::core::mem::size_of::<*const u8>()),
                                            ::core::mem::size_of::<*const u8>(),
                                        )
                                    }
                                }
                                fn results_offset(&self) -> usize {
                                    (16 + 2 * ::core::mem::size_of::<*const u8>())
                                }
                                unsafe fn call_import(
                                    &self,
                                    _params: Self::ParamsLower,
                                    _results: *mut u8,
                                ) -> u32 {
                                    unsafe extern "C" fn call(_: *mut u8, _: *mut u8) -> i32 {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                    unsafe { call(_params.0, _results) as u32 }
                                }
                                unsafe fn params_dealloc_lists(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {
                                        let l0 = *_params.0.add(0).cast::<*mut u8>();
                                        let l1 = *_params
                                            .0
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>();
                                        _rt::cabi_dealloc(l0, l1, 1);
                                    }
                                }
                                unsafe fn params_dealloc_lists_and_own(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {
                                        let l0 = *_params.0.add(0).cast::<*mut u8>();
                                        let l1 = *_params
                                            .0
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>();
                                        _rt::cabi_dealloc(l0, l1, 1);
                                        let l2 = *_params
                                            .0
                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                            .cast::<i32>();
                                        let _ = super::super::super::wasmledger::sql::query_types::SqlArguments::from_handle(
                                            l2 as u32,
                                        );
                                    }
                                }
                                unsafe fn params_lower(
                                    &self,
                                    (_lower0, _lower1): Self::Params,
                                    _ptr: *mut u8,
                                ) -> Self::ParamsLower {
                                    let _param_ptr = unsafe { _ptr.add(0) };
                                    unsafe {
                                        let super::super::super::wasmledger::sql::query_types::SqlQuery {
                                            sql: sql0,
                                            args: args0,
                                            persistent: persistent0,
                                        } = _lower0;
                                        let vec1 = (sql0.into_bytes()).into_boxed_slice();
                                        let ptr1 = vec1.as_ptr().cast::<u8>();
                                        let len1 = vec1.len();
                                        ::core::mem::forget(vec1);
                                        *_param_ptr
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>() = len1;
                                        *_param_ptr.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                        *_param_ptr
                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                            .cast::<i32>() = (args0).take_handle() as i32;
                                        match persistent0 {
                                            Some(e) => {
                                                *_param_ptr
                                                    .add(4 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                *_param_ptr
                                                    .add(5 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (match e {
                                                    true => 1,
                                                    false => 0,
                                                }) as u8;
                                            }
                                            None => {
                                                *_param_ptr
                                                    .add(4 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                        };
                                    }
                                    let _param_ptr = unsafe {
                                        _ptr.add((8 + 2 * ::core::mem::size_of::<*const u8>()))
                                    };
                                    unsafe {
                                        match _lower1 {
                                            QueryExecutor::Pool => {
                                                *_param_ptr.add(0).cast::<u8>() = (0i32) as u8;
                                            }
                                            QueryExecutor::Transaction(e) => {
                                                *_param_ptr.add(0).cast::<u8>() = (1i32) as u8;
                                                *_param_ptr.add(4).cast::<i32>() = (e).handle() as i32;
                                            }
                                        }
                                    }
                                    (_ptr,)
                                }
                                unsafe fn results_lift(
                                    &self,
                                    _ptr: *mut u8,
                                ) -> Self::Results {
                                    unsafe {
                                        let l0 = i32::from(*_ptr.add(0).cast::<u8>());
                                        match l0 {
                                            0 => {
                                                let e = {
                                                    let l1 = *_ptr
                                                        .add(::core::mem::size_of::<*const u8>())
                                                        .cast::<i32>();
                                                    super::super::super::wasmledger::sql::query_types::QueryResults::from_handle(
                                                        l1 as u32,
                                                    )
                                                };
                                                Ok(e)
                                            }
                                            1 => {
                                                let e = {
                                                    let l2 = i32::from(
                                                        *_ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                                    );
                                                    use super::super::super::wasmledger::sql::util_types::Error as V12;
                                                    let v12 = match l2 {
                                                        0 => V12::RowNotFound,
                                                        1 => {
                                                            let e12 = {
                                                                let l3 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l4 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len5 = l4;
                                                                let bytes5 = _rt::Vec::from_raw_parts(
                                                                    l3.cast(),
                                                                    len5,
                                                                    len5,
                                                                );
                                                                _rt::string_lift(bytes5)
                                                            };
                                                            V12::Encode(e12)
                                                        }
                                                        2 => {
                                                            let e12 = {
                                                                let l6 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l7 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len8 = l7;
                                                                let bytes8 = _rt::Vec::from_raw_parts(
                                                                    l6.cast(),
                                                                    len8,
                                                                    len8,
                                                                );
                                                                _rt::string_lift(bytes8)
                                                            };
                                                            V12::Decode(e12)
                                                        }
                                                        3 => V12::PoolTimedOut,
                                                        4 => V12::PoolClosed,
                                                        5 => V12::BeginFailed,
                                                        n => {
                                                            if true {
                                                                match (&n, &6) {
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
                                                            let e12 = {
                                                                let l9 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l10 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len11 = l10;
                                                                let bytes11 = _rt::Vec::from_raw_parts(
                                                                    l9.cast(),
                                                                    len11,
                                                                    len11,
                                                                );
                                                                _rt::string_lift(bytes11)
                                                            };
                                                            V12::Unexpected(e12)
                                                        }
                                                    };
                                                    v12
                                                };
                                                Err(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    }
                                }
                            }
                            _MySubtask {
                                _unused: core::marker::PhantomData,
                            }
                                .call((query, executor))
                                .await
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub async fn execute(
                        query: SqlQuery,
                        executor: QueryExecutor<'_>,
                    ) -> Result<u64, Error> {
                        unsafe {
                            use wit_bindgen::rt::async_support::Subtask as _Subtask;
                            struct _MySubtask<'a> {
                                _unused: core::marker::PhantomData<&'a ()>,
                            }
                            #[allow(unused_parens)]
                            unsafe impl<'a> _Subtask for _MySubtask<'a> {
                                type Params = (SqlQuery, QueryExecutor<'a>);
                                type Results = Result<u64, Error>;
                                type ParamsLower = (*mut u8,);
                                fn abi_layout(&self) -> ::core::alloc::Layout {
                                    unsafe {
                                        ::core::alloc::Layout::from_size_align_unchecked(
                                            (32 + 4 * ::core::mem::size_of::<*const u8>()),
                                            8,
                                        )
                                    }
                                }
                                fn results_offset(&self) -> usize {
                                    (16 + 2 * ::core::mem::size_of::<*const u8>())
                                }
                                unsafe fn call_import(
                                    &self,
                                    _params: Self::ParamsLower,
                                    _results: *mut u8,
                                ) -> u32 {
                                    unsafe extern "C" fn call(_: *mut u8, _: *mut u8) -> i32 {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                    unsafe { call(_params.0, _results) as u32 }
                                }
                                unsafe fn params_dealloc_lists(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {
                                        let l0 = *_params.0.add(0).cast::<*mut u8>();
                                        let l1 = *_params
                                            .0
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>();
                                        _rt::cabi_dealloc(l0, l1, 1);
                                    }
                                }
                                unsafe fn params_dealloc_lists_and_own(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {
                                        let l0 = *_params.0.add(0).cast::<*mut u8>();
                                        let l1 = *_params
                                            .0
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>();
                                        _rt::cabi_dealloc(l0, l1, 1);
                                        let l2 = *_params
                                            .0
                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                            .cast::<i32>();
                                        let _ = super::super::super::wasmledger::sql::query_types::SqlArguments::from_handle(
                                            l2 as u32,
                                        );
                                    }
                                }
                                unsafe fn params_lower(
                                    &self,
                                    (_lower0, _lower1): Self::Params,
                                    _ptr: *mut u8,
                                ) -> Self::ParamsLower {
                                    let _param_ptr = unsafe { _ptr.add(0) };
                                    unsafe {
                                        let super::super::super::wasmledger::sql::query_types::SqlQuery {
                                            sql: sql0,
                                            args: args0,
                                            persistent: persistent0,
                                        } = _lower0;
                                        let vec1 = (sql0.into_bytes()).into_boxed_slice();
                                        let ptr1 = vec1.as_ptr().cast::<u8>();
                                        let len1 = vec1.len();
                                        ::core::mem::forget(vec1);
                                        *_param_ptr
                                            .add(::core::mem::size_of::<*const u8>())
                                            .cast::<usize>() = len1;
                                        *_param_ptr.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                        *_param_ptr
                                            .add(2 * ::core::mem::size_of::<*const u8>())
                                            .cast::<i32>() = (args0).take_handle() as i32;
                                        match persistent0 {
                                            Some(e) => {
                                                *_param_ptr
                                                    .add(4 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (1i32) as u8;
                                                *_param_ptr
                                                    .add(5 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (match e {
                                                    true => 1,
                                                    false => 0,
                                                }) as u8;
                                            }
                                            None => {
                                                *_param_ptr
                                                    .add(4 + 2 * ::core::mem::size_of::<*const u8>())
                                                    .cast::<u8>() = (0i32) as u8;
                                            }
                                        };
                                    }
                                    let _param_ptr = unsafe {
                                        _ptr.add((8 + 2 * ::core::mem::size_of::<*const u8>()))
                                    };
                                    unsafe {
                                        match _lower1 {
                                            QueryExecutor::Pool => {
                                                *_param_ptr.add(0).cast::<u8>() = (0i32) as u8;
                                            }
                                            QueryExecutor::Transaction(e) => {
                                                *_param_ptr.add(0).cast::<u8>() = (1i32) as u8;
                                                *_param_ptr.add(4).cast::<i32>() = (e).handle() as i32;
                                            }
                                        }
                                    }
                                    (_ptr,)
                                }
                                unsafe fn results_lift(
                                    &self,
                                    _ptr: *mut u8,
                                ) -> Self::Results {
                                    unsafe {
                                        let l0 = i32::from(*_ptr.add(0).cast::<u8>());
                                        match l0 {
                                            0 => {
                                                let e = {
                                                    let l1 = *_ptr.add(8).cast::<i64>();
                                                    l1 as u64
                                                };
                                                Ok(e)
                                            }
                                            1 => {
                                                let e = {
                                                    let l2 = i32::from(*_ptr.add(8).cast::<u8>());
                                                    use super::super::super::wasmledger::sql::util_types::Error as V12;
                                                    let v12 = match l2 {
                                                        0 => V12::RowNotFound,
                                                        1 => {
                                                            let e12 = {
                                                                let l3 = *_ptr
                                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l4 = *_ptr
                                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len5 = l4;
                                                                let bytes5 = _rt::Vec::from_raw_parts(
                                                                    l3.cast(),
                                                                    len5,
                                                                    len5,
                                                                );
                                                                _rt::string_lift(bytes5)
                                                            };
                                                            V12::Encode(e12)
                                                        }
                                                        2 => {
                                                            let e12 = {
                                                                let l6 = *_ptr
                                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l7 = *_ptr
                                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len8 = l7;
                                                                let bytes8 = _rt::Vec::from_raw_parts(
                                                                    l6.cast(),
                                                                    len8,
                                                                    len8,
                                                                );
                                                                _rt::string_lift(bytes8)
                                                            };
                                                            V12::Decode(e12)
                                                        }
                                                        3 => V12::PoolTimedOut,
                                                        4 => V12::PoolClosed,
                                                        5 => V12::BeginFailed,
                                                        n => {
                                                            if true {
                                                                match (&n, &6) {
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
                                                            let e12 = {
                                                                let l9 = *_ptr
                                                                    .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l10 = *_ptr
                                                                    .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len11 = l10;
                                                                let bytes11 = _rt::Vec::from_raw_parts(
                                                                    l9.cast(),
                                                                    len11,
                                                                    len11,
                                                                );
                                                                _rt::string_lift(bytes11)
                                                            };
                                                            V12::Unexpected(e12)
                                                        }
                                                    };
                                                    v12
                                                };
                                                Err(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    }
                                }
                            }
                            _MySubtask {
                                _unused: core::marker::PhantomData,
                            }
                                .call((query, executor))
                                .await
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub async fn execute_raw(
                        query: SqlString,
                        executor: QueryExecutor<'_>,
                    ) -> Result<(), Error> {
                        unsafe {
                            use wit_bindgen::rt::async_support::Subtask as _Subtask;
                            struct _MySubtask<'a> {
                                _unused: core::marker::PhantomData<&'a ()>,
                            }
                            #[allow(unused_parens)]
                            unsafe impl<'a> _Subtask for _MySubtask<'a> {
                                type Params = (SqlString, QueryExecutor<'a>);
                                type Results = Result<(), Error>;
                                type ParamsLower = (*mut u8, usize, i32, i32);
                                fn abi_layout(&self) -> ::core::alloc::Layout {
                                    unsafe {
                                        ::core::alloc::Layout::from_size_align_unchecked(
                                            (4 * ::core::mem::size_of::<*const u8>()),
                                            ::core::mem::size_of::<*const u8>(),
                                        )
                                    }
                                }
                                fn results_offset(&self) -> usize {
                                    0
                                }
                                unsafe fn call_import(
                                    &self,
                                    _params: Self::ParamsLower,
                                    _results: *mut u8,
                                ) -> u32 {
                                    unsafe extern "C" fn call(
                                        _: *mut u8,
                                        _: usize,
                                        _: i32,
                                        _: i32,
                                        _: *mut u8,
                                    ) -> i32 {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                    unsafe {
                                        call(_params.0, _params.1, _params.2, _params.3, _results)
                                            as u32
                                    }
                                }
                                unsafe fn params_dealloc_lists(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {
                                        _rt::cabi_dealloc(_params.0, _params.1, 1);
                                        match _params.2 {
                                            0 => {}
                                            _ => {}
                                        }
                                    }
                                }
                                unsafe fn params_dealloc_lists_and_own(
                                    &self,
                                    _params: Self::ParamsLower,
                                ) {
                                    unsafe {
                                        _rt::cabi_dealloc(_params.0, _params.1, 1);
                                        match _params.2 {
                                            0 => {}
                                            _ => {}
                                        }
                                    }
                                }
                                unsafe fn params_lower(
                                    &self,
                                    (_lower0, _lower1): Self::Params,
                                    _ptr: *mut u8,
                                ) -> Self::ParamsLower {
                                    unsafe {
                                        let vec0 = (_lower0.into_bytes()).into_boxed_slice();
                                        let ptr0 = vec0.as_ptr().cast::<u8>();
                                        let len0 = vec0.len();
                                        ::core::mem::forget(vec0);
                                        let (result1_0, result1_1) = match _lower1 {
                                            QueryExecutor::Pool => (0i32, 0i32),
                                            QueryExecutor::Transaction(e) => (1i32, (e).handle() as i32),
                                        };
                                        (ptr0.cast_mut(), len0, result1_0, result1_1)
                                    }
                                }
                                unsafe fn results_lift(
                                    &self,
                                    _ptr: *mut u8,
                                ) -> Self::Results {
                                    unsafe {
                                        let l0 = i32::from(*_ptr.add(0).cast::<u8>());
                                        match l0 {
                                            0 => {
                                                let e = ();
                                                Ok(e)
                                            }
                                            1 => {
                                                let e = {
                                                    let l1 = i32::from(
                                                        *_ptr.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                                    );
                                                    use super::super::super::wasmledger::sql::util_types::Error as V11;
                                                    let v11 = match l1 {
                                                        0 => V11::RowNotFound,
                                                        1 => {
                                                            let e11 = {
                                                                let l2 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l3 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len4 = l3;
                                                                let bytes4 = _rt::Vec::from_raw_parts(
                                                                    l2.cast(),
                                                                    len4,
                                                                    len4,
                                                                );
                                                                _rt::string_lift(bytes4)
                                                            };
                                                            V11::Encode(e11)
                                                        }
                                                        2 => {
                                                            let e11 = {
                                                                let l5 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l6 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len7 = l6;
                                                                let bytes7 = _rt::Vec::from_raw_parts(
                                                                    l5.cast(),
                                                                    len7,
                                                                    len7,
                                                                );
                                                                _rt::string_lift(bytes7)
                                                            };
                                                            V11::Decode(e11)
                                                        }
                                                        3 => V11::PoolTimedOut,
                                                        4 => V11::PoolClosed,
                                                        5 => V11::BeginFailed,
                                                        n => {
                                                            if true {
                                                                match (&n, &6) {
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
                                                            let e11 = {
                                                                let l8 = *_ptr
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<*mut u8>();
                                                                let l9 = *_ptr
                                                                    .add(3 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<usize>();
                                                                let len10 = l9;
                                                                let bytes10 = _rt::Vec::from_raw_parts(
                                                                    l8.cast(),
                                                                    len10,
                                                                    len10,
                                                                );
                                                                _rt::string_lift(bytes10)
                                                            };
                                                            V11::Unexpected(e11)
                                                        }
                                                    };
                                                    v11
                                                };
                                                Err(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    }
                                }
                            }
                            _MySubtask {
                                _unused: core::marker::PhantomData,
                            }
                                .call((query, executor))
                                .await
                        }
                    }
                }
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod codecs {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
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
            pub use alloc_crate::vec::Vec;
            pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
                if true {
                    String::from_utf8(bytes).unwrap()
                } else {
                    unsafe { String::from_utf8_unchecked(bytes) }
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
            pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
                if size == 0 {
                    return;
                }
                unsafe {
                    let layout = alloc::Layout::from_size_align_unchecked(size, align);
                    alloc::dealloc(ptr, layout);
                }
            }
            extern crate alloc as alloc_crate;
            pub use alloc_crate::alloc;
        }
        #[inline(never)]
        #[doc(hidden)]
        pub fn __link_custom_section_describing_imports() {
            wit_bindgen::rt::maybe_link_cabi_realloc();
        }
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface codecs {\n  use util-types.{error};\n\n  type push-result = result<_, error>;\n\n  type row-index = u64;\n\n  variant column-index {\n    index(u64),\n    column-name(string)\n  }\n\n  type value-position = tuple<row-index, column-index>;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// no rows returned by a query that expected to return at least one row.\n    row-not-found,\n\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results {\n    row-count: func() -> u64;\n  }\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  begin-transaction: async func() -> result<transaction, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
        const _: &[u8] = b"package wasmledger:sql-client;\n\nworld client-base {\n  import wasmledger:sql/pool;\n  import wasmledger:sql/transaction;\n  import wasmledger:sql/query;\n  import wasmledger:sql/query-types;\n  import wasmledger:sql/codecs;\n}";
    }
    pub struct RowPointer {
        row_index: u64,
    }
    pub struct QueryResultsIter {
        row_count: u64,
        current: u64,
    }
    impl RowPointer {
        pub fn as_position_index(&self, index: u64) -> ValuePosition {
            (self.row_index, ColumnIndex::Index(index))
        }
        pub fn as_position_name(&self, name: String) -> ValuePosition {
            (self.row_index, ColumnIndex::ColumnName(name))
        }
    }
    impl Iterator for QueryResultsIter {
        type Item = RowPointer;
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.row_count {
                let pos = RowPointer {
                    row_index: self.current,
                };
                self.current += 1;
                Some(pos)
            } else {
                None
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = (self.row_count - self.current) as usize;
            (remaining, Some(remaining))
        }
    }
    impl<'a> IntoIterator for &'a QueryResults {
        type Item = RowPointer;
        type IntoIter = QueryResultsIter;
        fn into_iter(self) -> Self::IntoIter {
            QueryResultsIter {
                row_count: self.row_count(),
                current: 0,
            }
        }
    }
}
pub mod postgres {
    use std::{slice, vec::IntoIter};
    pub mod bindings {
        #[allow(unfulfilled_lint_expectations, unused_imports)]
        use crate::base::bindings::wasmledger::sql::query_types as __with_name0;
        #[allow(unfulfilled_lint_expectations, unused_imports)]
        use crate::base::bindings::wasmledger::sql::util_types as __with_name1;
        #[allow(unfulfilled_lint_expectations, unused_imports)]
        use crate::base::bindings::wasmledger::sql::codecs as __with_name2;
        #[allow(dead_code, clippy::all)]
        pub mod wasmledger {
            pub mod sql_postgres {
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod postgres_codecs {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type SqlArguments = super::super::super::__with_name0::SqlArguments;
                    pub type QueryResults = super::super::super::__with_name0::QueryResults;
                    pub type Error = super::super::super::__with_name1::Error;
                    pub type PushResult = super::super::super::__with_name2::PushResult;
                    pub type ValuePosition = super::super::super::__with_name2::ValuePosition;
                    pub type Uuid = _rt::String;
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_int16(
                        value: Option<i16>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result0_0, result0_1) = match value {
                                Some(e) => (1i32, _rt::as_i32(e)),
                                None => (0i32, 0i32),
                            };
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import2(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import2(
                                result0_0,
                                result0_1,
                                (to).handle() as i32,
                                ptr1,
                            );
                            let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                            let result15 = match l3 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l4 = i32::from(
                                            *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V14;
                                        let v14 = match l4 {
                                            0 => V14::RowNotFound,
                                            1 => {
                                                let e14 = {
                                                    let l5 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l6 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len7 = l6;
                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                        l5.cast(),
                                                        len7,
                                                        len7,
                                                    );
                                                    _rt::string_lift(bytes7)
                                                };
                                                V14::Encode(e14)
                                            }
                                            2 => {
                                                let e14 = {
                                                    let l8 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                V14::Decode(e14)
                                            }
                                            3 => V14::PoolTimedOut,
                                            4 => V14::PoolClosed,
                                            5 => V14::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e14 = {
                                                    let l11 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l12 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len13 = l12;
                                                    let bytes13 = _rt::Vec::from_raw_parts(
                                                        l11.cast(),
                                                        len13,
                                                        len13,
                                                    );
                                                    _rt::string_lift(bytes13)
                                                };
                                                V14::Unexpected(e14)
                                            }
                                        };
                                        v14
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result15
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_int16(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<i16>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result20 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = i32::from(
                                                        *ptr4
                                                            .add(2 + 1 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<i16>(),
                                                    );
                                                    l8 as i16
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l9 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V19;
                                        let v19 = match l9 {
                                            0 => V19::RowNotFound,
                                            1 => {
                                                let e19 = {
                                                    let l10 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l11 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len12 = l11;
                                                    let bytes12 = _rt::Vec::from_raw_parts(
                                                        l10.cast(),
                                                        len12,
                                                        len12,
                                                    );
                                                    _rt::string_lift(bytes12)
                                                };
                                                V19::Encode(e19)
                                            }
                                            2 => {
                                                let e19 = {
                                                    let l13 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l14 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len15 = l14;
                                                    let bytes15 = _rt::Vec::from_raw_parts(
                                                        l13.cast(),
                                                        len15,
                                                        len15,
                                                    );
                                                    _rt::string_lift(bytes15)
                                                };
                                                V19::Decode(e19)
                                            }
                                            3 => V19::PoolTimedOut,
                                            4 => V19::PoolClosed,
                                            5 => V19::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e19 = {
                                                    let l16 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l17 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len18 = l17;
                                                    let bytes18 = _rt::Vec::from_raw_parts(
                                                        l16.cast(),
                                                        len18,
                                                        len18,
                                                    );
                                                    _rt::string_lift(bytes18)
                                                };
                                                V19::Unexpected(e19)
                                            }
                                        };
                                        v19
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result20
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_int32(
                        value: Option<i32>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result0_0, result0_1) = match value {
                                Some(e) => (1i32, _rt::as_i32(e)),
                                None => (0i32, 0i32),
                            };
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import2(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import2(
                                result0_0,
                                result0_1,
                                (to).handle() as i32,
                                ptr1,
                            );
                            let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                            let result15 = match l3 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l4 = i32::from(
                                            *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V14;
                                        let v14 = match l4 {
                                            0 => V14::RowNotFound,
                                            1 => {
                                                let e14 = {
                                                    let l5 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l6 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len7 = l6;
                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                        l5.cast(),
                                                        len7,
                                                        len7,
                                                    );
                                                    _rt::string_lift(bytes7)
                                                };
                                                V14::Encode(e14)
                                            }
                                            2 => {
                                                let e14 = {
                                                    let l8 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                V14::Decode(e14)
                                            }
                                            3 => V14::PoolTimedOut,
                                            4 => V14::PoolClosed,
                                            5 => V14::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e14 = {
                                                    let l11 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l12 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len13 = l12;
                                                    let bytes13 = _rt::Vec::from_raw_parts(
                                                        l11.cast(),
                                                        len13,
                                                        len13,
                                                    );
                                                    _rt::string_lift(bytes13)
                                                };
                                                V14::Unexpected(e14)
                                            }
                                        };
                                        v14
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result15
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_int32(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<i32>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result20 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4
                                                        .add(4 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<i32>();
                                                    l8
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l9 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V19;
                                        let v19 = match l9 {
                                            0 => V19::RowNotFound,
                                            1 => {
                                                let e19 = {
                                                    let l10 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l11 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len12 = l11;
                                                    let bytes12 = _rt::Vec::from_raw_parts(
                                                        l10.cast(),
                                                        len12,
                                                        len12,
                                                    );
                                                    _rt::string_lift(bytes12)
                                                };
                                                V19::Encode(e19)
                                            }
                                            2 => {
                                                let e19 = {
                                                    let l13 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l14 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len15 = l14;
                                                    let bytes15 = _rt::Vec::from_raw_parts(
                                                        l13.cast(),
                                                        len15,
                                                        len15,
                                                    );
                                                    _rt::string_lift(bytes15)
                                                };
                                                V19::Decode(e19)
                                            }
                                            3 => V19::PoolTimedOut,
                                            4 => V19::PoolClosed,
                                            5 => V19::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e19 = {
                                                    let l16 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l17 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len18 = l17;
                                                    let bytes18 = _rt::Vec::from_raw_parts(
                                                        l16.cast(),
                                                        len18,
                                                        len18,
                                                    );
                                                    _rt::string_lift(bytes18)
                                                };
                                                V19::Unexpected(e19)
                                            }
                                        };
                                        v19
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result20
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_int64(
                        value: Option<i64>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result0_0, result0_1) = match value {
                                Some(e) => (1i32, _rt::as_i64(e)),
                                None => (0i32, 0i64),
                            };
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import2(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import2(
                                result0_0,
                                result0_1,
                                (to).handle() as i32,
                                ptr1,
                            );
                            let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                            let result15 = match l3 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l4 = i32::from(
                                            *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V14;
                                        let v14 = match l4 {
                                            0 => V14::RowNotFound,
                                            1 => {
                                                let e14 = {
                                                    let l5 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l6 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len7 = l6;
                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                        l5.cast(),
                                                        len7,
                                                        len7,
                                                    );
                                                    _rt::string_lift(bytes7)
                                                };
                                                V14::Encode(e14)
                                            }
                                            2 => {
                                                let e14 = {
                                                    let l8 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                V14::Decode(e14)
                                            }
                                            3 => V14::PoolTimedOut,
                                            4 => V14::PoolClosed,
                                            5 => V14::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e14 = {
                                                    let l11 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l12 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len13 = l12;
                                                    let bytes13 = _rt::Vec::from_raw_parts(
                                                        l11.cast(),
                                                        len13,
                                                        len13,
                                                    );
                                                    _rt::string_lift(bytes13)
                                                };
                                                V14::Unexpected(e14)
                                            }
                                        };
                                        v14
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result15
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_int64(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<i64>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 16 + 2 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 16
                                    + 2 * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result20 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(*ptr4.add(8).cast::<u8>());
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4.add(16).cast::<i64>();
                                                    l8
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l9 = i32::from(*ptr4.add(8).cast::<u8>());
                                        use super::super::super::__with_name1::Error as V19;
                                        let v19 = match l9 {
                                            0 => V19::RowNotFound,
                                            1 => {
                                                let e19 = {
                                                    let l10 = *ptr4
                                                        .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l11 = *ptr4
                                                        .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len12 = l11;
                                                    let bytes12 = _rt::Vec::from_raw_parts(
                                                        l10.cast(),
                                                        len12,
                                                        len12,
                                                    );
                                                    _rt::string_lift(bytes12)
                                                };
                                                V19::Encode(e19)
                                            }
                                            2 => {
                                                let e19 = {
                                                    let l13 = *ptr4
                                                        .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l14 = *ptr4
                                                        .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len15 = l14;
                                                    let bytes15 = _rt::Vec::from_raw_parts(
                                                        l13.cast(),
                                                        len15,
                                                        len15,
                                                    );
                                                    _rt::string_lift(bytes15)
                                                };
                                                V19::Decode(e19)
                                            }
                                            3 => V19::PoolTimedOut,
                                            4 => V19::PoolClosed,
                                            5 => V19::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e19 = {
                                                    let l16 = *ptr4
                                                        .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l17 = *ptr4
                                                        .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len18 = l17;
                                                    let bytes18 = _rt::Vec::from_raw_parts(
                                                        l16.cast(),
                                                        len18,
                                                        len18,
                                                    );
                                                    _rt::string_lift(bytes18)
                                                };
                                                V19::Unexpected(e19)
                                            }
                                        };
                                        v19
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result20
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_float32(
                        value: Option<f32>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result0_0, result0_1) = match value {
                                Some(e) => (1i32, _rt::as_f32(e)),
                                None => (0i32, 0.0f32),
                            };
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import2(
                                _: i32,
                                _: f32,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import2(
                                result0_0,
                                result0_1,
                                (to).handle() as i32,
                                ptr1,
                            );
                            let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                            let result15 = match l3 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l4 = i32::from(
                                            *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V14;
                                        let v14 = match l4 {
                                            0 => V14::RowNotFound,
                                            1 => {
                                                let e14 = {
                                                    let l5 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l6 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len7 = l6;
                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                        l5.cast(),
                                                        len7,
                                                        len7,
                                                    );
                                                    _rt::string_lift(bytes7)
                                                };
                                                V14::Encode(e14)
                                            }
                                            2 => {
                                                let e14 = {
                                                    let l8 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                V14::Decode(e14)
                                            }
                                            3 => V14::PoolTimedOut,
                                            4 => V14::PoolClosed,
                                            5 => V14::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e14 = {
                                                    let l11 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l12 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len13 = l12;
                                                    let bytes13 = _rt::Vec::from_raw_parts(
                                                        l11.cast(),
                                                        len13,
                                                        len13,
                                                    );
                                                    _rt::string_lift(bytes13)
                                                };
                                                V14::Unexpected(e14)
                                            }
                                        };
                                        v14
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result15
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_float32(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<f32>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result20 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4
                                                        .add(4 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<f32>();
                                                    l8
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l9 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V19;
                                        let v19 = match l9 {
                                            0 => V19::RowNotFound,
                                            1 => {
                                                let e19 = {
                                                    let l10 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l11 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len12 = l11;
                                                    let bytes12 = _rt::Vec::from_raw_parts(
                                                        l10.cast(),
                                                        len12,
                                                        len12,
                                                    );
                                                    _rt::string_lift(bytes12)
                                                };
                                                V19::Encode(e19)
                                            }
                                            2 => {
                                                let e19 = {
                                                    let l13 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l14 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len15 = l14;
                                                    let bytes15 = _rt::Vec::from_raw_parts(
                                                        l13.cast(),
                                                        len15,
                                                        len15,
                                                    );
                                                    _rt::string_lift(bytes15)
                                                };
                                                V19::Decode(e19)
                                            }
                                            3 => V19::PoolTimedOut,
                                            4 => V19::PoolClosed,
                                            5 => V19::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e19 = {
                                                    let l16 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l17 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len18 = l17;
                                                    let bytes18 = _rt::Vec::from_raw_parts(
                                                        l16.cast(),
                                                        len18,
                                                        len18,
                                                    );
                                                    _rt::string_lift(bytes18)
                                                };
                                                V19::Unexpected(e19)
                                            }
                                        };
                                        v19
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result20
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_float64(
                        value: Option<f64>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result0_0, result0_1) = match value {
                                Some(e) => (1i32, _rt::as_f64(e)),
                                None => (0i32, 0.0f64),
                            };
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import2(
                                _: i32,
                                _: f64,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import2(
                                result0_0,
                                result0_1,
                                (to).handle() as i32,
                                ptr1,
                            );
                            let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                            let result15 = match l3 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l4 = i32::from(
                                            *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V14;
                                        let v14 = match l4 {
                                            0 => V14::RowNotFound,
                                            1 => {
                                                let e14 = {
                                                    let l5 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l6 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len7 = l6;
                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                        l5.cast(),
                                                        len7,
                                                        len7,
                                                    );
                                                    _rt::string_lift(bytes7)
                                                };
                                                V14::Encode(e14)
                                            }
                                            2 => {
                                                let e14 = {
                                                    let l8 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                V14::Decode(e14)
                                            }
                                            3 => V14::PoolTimedOut,
                                            4 => V14::PoolClosed,
                                            5 => V14::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e14 = {
                                                    let l11 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l12 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len13 = l12;
                                                    let bytes13 = _rt::Vec::from_raw_parts(
                                                        l11.cast(),
                                                        len13,
                                                        len13,
                                                    );
                                                    _rt::string_lift(bytes13)
                                                };
                                                V14::Unexpected(e14)
                                            }
                                        };
                                        v14
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result15
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_float64(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<f64>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 16 + 2 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 16
                                    + 2 * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result20 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(*ptr4.add(8).cast::<u8>());
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4.add(16).cast::<f64>();
                                                    l8
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l9 = i32::from(*ptr4.add(8).cast::<u8>());
                                        use super::super::super::__with_name1::Error as V19;
                                        let v19 = match l9 {
                                            0 => V19::RowNotFound,
                                            1 => {
                                                let e19 = {
                                                    let l10 = *ptr4
                                                        .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l11 = *ptr4
                                                        .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len12 = l11;
                                                    let bytes12 = _rt::Vec::from_raw_parts(
                                                        l10.cast(),
                                                        len12,
                                                        len12,
                                                    );
                                                    _rt::string_lift(bytes12)
                                                };
                                                V19::Encode(e19)
                                            }
                                            2 => {
                                                let e19 = {
                                                    let l13 = *ptr4
                                                        .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l14 = *ptr4
                                                        .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len15 = l14;
                                                    let bytes15 = _rt::Vec::from_raw_parts(
                                                        l13.cast(),
                                                        len15,
                                                        len15,
                                                    );
                                                    _rt::string_lift(bytes15)
                                                };
                                                V19::Decode(e19)
                                            }
                                            3 => V19::PoolTimedOut,
                                            4 => V19::PoolClosed,
                                            5 => V19::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e19 = {
                                                    let l16 = *ptr4
                                                        .add(8 + 1 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l17 = *ptr4
                                                        .add(8 + 2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len18 = l17;
                                                    let bytes18 = _rt::Vec::from_raw_parts(
                                                        l16.cast(),
                                                        len18,
                                                        len18,
                                                    );
                                                    _rt::string_lift(bytes18)
                                                };
                                                V19::Unexpected(e19)
                                            }
                                        };
                                        v19
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result20
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_string(
                        value: Option<&str>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result1_0, result1_1, result1_2) = match value {
                                Some(e) => {
                                    let vec0 = e;
                                    let ptr0 = vec0.as_ptr().cast::<u8>();
                                    let len0 = vec0.len();
                                    (1i32, ptr0.cast_mut(), len0)
                                }
                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                            };
                            let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import3(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import3(
                                result1_0,
                                result1_1,
                                result1_2,
                                (to).handle() as i32,
                                ptr2,
                            );
                            let l4 = i32::from(*ptr2.add(0).cast::<u8>());
                            let result16 = match l4 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l5 = i32::from(
                                            *ptr2.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V15;
                                        let v15 = match l5 {
                                            0 => V15::RowNotFound,
                                            1 => {
                                                let e15 = {
                                                    let l6 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l7 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len8 = l7;
                                                    let bytes8 = _rt::Vec::from_raw_parts(
                                                        l6.cast(),
                                                        len8,
                                                        len8,
                                                    );
                                                    _rt::string_lift(bytes8)
                                                };
                                                V15::Encode(e15)
                                            }
                                            2 => {
                                                let e15 = {
                                                    let l9 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l10 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len11 = l10;
                                                    let bytes11 = _rt::Vec::from_raw_parts(
                                                        l9.cast(),
                                                        len11,
                                                        len11,
                                                    );
                                                    _rt::string_lift(bytes11)
                                                };
                                                V15::Decode(e15)
                                            }
                                            3 => V15::PoolTimedOut,
                                            4 => V15::PoolClosed,
                                            5 => V15::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e15 = {
                                                    let l12 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l13 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len14 = l13;
                                                    let bytes14 = _rt::Vec::from_raw_parts(
                                                        l12.cast(),
                                                        len14,
                                                        len14,
                                                    );
                                                    _rt::string_lift(bytes14)
                                                };
                                                V15::Unexpected(e15)
                                            }
                                        };
                                        v15
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result16
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_string(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<_rt::String>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result22 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l11 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V21;
                                        let v21 = match l11 {
                                            0 => V21::RowNotFound,
                                            1 => {
                                                let e21 = {
                                                    let l12 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l13 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len14 = l13;
                                                    let bytes14 = _rt::Vec::from_raw_parts(
                                                        l12.cast(),
                                                        len14,
                                                        len14,
                                                    );
                                                    _rt::string_lift(bytes14)
                                                };
                                                V21::Encode(e21)
                                            }
                                            2 => {
                                                let e21 = {
                                                    let l15 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l16 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len17 = l16;
                                                    let bytes17 = _rt::Vec::from_raw_parts(
                                                        l15.cast(),
                                                        len17,
                                                        len17,
                                                    );
                                                    _rt::string_lift(bytes17)
                                                };
                                                V21::Decode(e21)
                                            }
                                            3 => V21::PoolTimedOut,
                                            4 => V21::PoolClosed,
                                            5 => V21::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e21 = {
                                                    let l18 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l19 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len20 = l19;
                                                    let bytes20 = _rt::Vec::from_raw_parts(
                                                        l18.cast(),
                                                        len20,
                                                        len20,
                                                    );
                                                    _rt::string_lift(bytes20)
                                                };
                                                V21::Unexpected(e21)
                                            }
                                        };
                                        v21
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result22
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_bool(
                        value: Option<bool>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result0_0, result0_1) = match value {
                                Some(e) => {
                                    (
                                        1i32,
                                        match e {
                                            true => 1,
                                            false => 0,
                                        },
                                    )
                                }
                                None => (0i32, 0i32),
                            };
                            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import2(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import2(
                                result0_0,
                                result0_1,
                                (to).handle() as i32,
                                ptr1,
                            );
                            let l3 = i32::from(*ptr1.add(0).cast::<u8>());
                            let result15 = match l3 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l4 = i32::from(
                                            *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V14;
                                        let v14 = match l4 {
                                            0 => V14::RowNotFound,
                                            1 => {
                                                let e14 = {
                                                    let l5 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l6 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len7 = l6;
                                                    let bytes7 = _rt::Vec::from_raw_parts(
                                                        l5.cast(),
                                                        len7,
                                                        len7,
                                                    );
                                                    _rt::string_lift(bytes7)
                                                };
                                                V14::Encode(e14)
                                            }
                                            2 => {
                                                let e14 = {
                                                    let l8 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                V14::Decode(e14)
                                            }
                                            3 => V14::PoolTimedOut,
                                            4 => V14::PoolClosed,
                                            5 => V14::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e14 = {
                                                    let l11 = *ptr1
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l12 = *ptr1
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len13 = l12;
                                                    let bytes13 = _rt::Vec::from_raw_parts(
                                                        l11.cast(),
                                                        len13,
                                                        len13,
                                                    );
                                                    _rt::string_lift(bytes13)
                                                };
                                                V14::Unexpected(e14)
                                            }
                                        };
                                        v14
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result15
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_bool(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<bool>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result20 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = i32::from(
                                                        *ptr4
                                                            .add(1 + 1 * ::core::mem::size_of::<*const u8>())
                                                            .cast::<u8>(),
                                                    );
                                                    _rt::bool_lift(l8 as u8)
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l9 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V19;
                                        let v19 = match l9 {
                                            0 => V19::RowNotFound,
                                            1 => {
                                                let e19 = {
                                                    let l10 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l11 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len12 = l11;
                                                    let bytes12 = _rt::Vec::from_raw_parts(
                                                        l10.cast(),
                                                        len12,
                                                        len12,
                                                    );
                                                    _rt::string_lift(bytes12)
                                                };
                                                V19::Encode(e19)
                                            }
                                            2 => {
                                                let e19 = {
                                                    let l13 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l14 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len15 = l14;
                                                    let bytes15 = _rt::Vec::from_raw_parts(
                                                        l13.cast(),
                                                        len15,
                                                        len15,
                                                    );
                                                    _rt::string_lift(bytes15)
                                                };
                                                V19::Decode(e19)
                                            }
                                            3 => V19::PoolTimedOut,
                                            4 => V19::PoolClosed,
                                            5 => V19::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e19 = {
                                                    let l16 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l17 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len18 = l17;
                                                    let bytes18 = _rt::Vec::from_raw_parts(
                                                        l16.cast(),
                                                        len18,
                                                        len18,
                                                    );
                                                    _rt::string_lift(bytes18)
                                                };
                                                V19::Unexpected(e19)
                                            }
                                        };
                                        v19
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result20
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_json(
                        value: Option<&str>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result1_0, result1_1, result1_2) = match value {
                                Some(e) => {
                                    let vec0 = e;
                                    let ptr0 = vec0.as_ptr().cast::<u8>();
                                    let len0 = vec0.len();
                                    (1i32, ptr0.cast_mut(), len0)
                                }
                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                            };
                            let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import3(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import3(
                                result1_0,
                                result1_1,
                                result1_2,
                                (to).handle() as i32,
                                ptr2,
                            );
                            let l4 = i32::from(*ptr2.add(0).cast::<u8>());
                            let result16 = match l4 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l5 = i32::from(
                                            *ptr2.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V15;
                                        let v15 = match l5 {
                                            0 => V15::RowNotFound,
                                            1 => {
                                                let e15 = {
                                                    let l6 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l7 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len8 = l7;
                                                    let bytes8 = _rt::Vec::from_raw_parts(
                                                        l6.cast(),
                                                        len8,
                                                        len8,
                                                    );
                                                    _rt::string_lift(bytes8)
                                                };
                                                V15::Encode(e15)
                                            }
                                            2 => {
                                                let e15 = {
                                                    let l9 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l10 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len11 = l10;
                                                    let bytes11 = _rt::Vec::from_raw_parts(
                                                        l9.cast(),
                                                        len11,
                                                        len11,
                                                    );
                                                    _rt::string_lift(bytes11)
                                                };
                                                V15::Decode(e15)
                                            }
                                            3 => V15::PoolTimedOut,
                                            4 => V15::PoolClosed,
                                            5 => V15::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e15 = {
                                                    let l12 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l13 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len14 = l13;
                                                    let bytes14 = _rt::Vec::from_raw_parts(
                                                        l12.cast(),
                                                        len14,
                                                        len14,
                                                    );
                                                    _rt::string_lift(bytes14)
                                                };
                                                V15::Unexpected(e15)
                                            }
                                        };
                                        v15
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result16
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_json(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<_rt::String>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result22 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l11 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V21;
                                        let v21 = match l11 {
                                            0 => V21::RowNotFound,
                                            1 => {
                                                let e21 = {
                                                    let l12 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l13 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len14 = l13;
                                                    let bytes14 = _rt::Vec::from_raw_parts(
                                                        l12.cast(),
                                                        len14,
                                                        len14,
                                                    );
                                                    _rt::string_lift(bytes14)
                                                };
                                                V21::Encode(e21)
                                            }
                                            2 => {
                                                let e21 = {
                                                    let l15 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l16 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len17 = l16;
                                                    let bytes17 = _rt::Vec::from_raw_parts(
                                                        l15.cast(),
                                                        len17,
                                                        len17,
                                                    );
                                                    _rt::string_lift(bytes17)
                                                };
                                                V21::Decode(e21)
                                            }
                                            3 => V21::PoolTimedOut,
                                            4 => V21::PoolClosed,
                                            5 => V21::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e21 = {
                                                    let l18 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l19 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len20 = l19;
                                                    let bytes20 = _rt::Vec::from_raw_parts(
                                                        l18.cast(),
                                                        len20,
                                                        len20,
                                                    );
                                                    _rt::string_lift(bytes20)
                                                };
                                                V21::Unexpected(e21)
                                            }
                                        };
                                        v21
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result22
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_uuid(
                        value: Option<&str>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result1_0, result1_1, result1_2) = match value {
                                Some(e) => {
                                    let vec0 = e;
                                    let ptr0 = vec0.as_ptr().cast::<u8>();
                                    let len0 = vec0.len();
                                    (1i32, ptr0.cast_mut(), len0)
                                }
                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                            };
                            let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import3(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import3(
                                result1_0,
                                result1_1,
                                result1_2,
                                (to).handle() as i32,
                                ptr2,
                            );
                            let l4 = i32::from(*ptr2.add(0).cast::<u8>());
                            let result16 = match l4 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l5 = i32::from(
                                            *ptr2.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V15;
                                        let v15 = match l5 {
                                            0 => V15::RowNotFound,
                                            1 => {
                                                let e15 = {
                                                    let l6 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l7 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len8 = l7;
                                                    let bytes8 = _rt::Vec::from_raw_parts(
                                                        l6.cast(),
                                                        len8,
                                                        len8,
                                                    );
                                                    _rt::string_lift(bytes8)
                                                };
                                                V15::Encode(e15)
                                            }
                                            2 => {
                                                let e15 = {
                                                    let l9 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l10 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len11 = l10;
                                                    let bytes11 = _rt::Vec::from_raw_parts(
                                                        l9.cast(),
                                                        len11,
                                                        len11,
                                                    );
                                                    _rt::string_lift(bytes11)
                                                };
                                                V15::Decode(e15)
                                            }
                                            3 => V15::PoolTimedOut,
                                            4 => V15::PoolClosed,
                                            5 => V15::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e15 = {
                                                    let l12 = *ptr2
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l13 = *ptr2
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len14 = l13;
                                                    let bytes14 = _rt::Vec::from_raw_parts(
                                                        l12.cast(),
                                                        len14,
                                                        len14,
                                                    );
                                                    _rt::string_lift(bytes14)
                                                };
                                                V15::Unexpected(e15)
                                            }
                                        };
                                        v15
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result16
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_uuid(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<Uuid>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result22 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len10 = l9;
                                                    let bytes10 = _rt::Vec::from_raw_parts(
                                                        l8.cast(),
                                                        len10,
                                                        len10,
                                                    );
                                                    _rt::string_lift(bytes10)
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l11 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V21;
                                        let v21 = match l11 {
                                            0 => V21::RowNotFound,
                                            1 => {
                                                let e21 = {
                                                    let l12 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l13 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len14 = l13;
                                                    let bytes14 = _rt::Vec::from_raw_parts(
                                                        l12.cast(),
                                                        len14,
                                                        len14,
                                                    );
                                                    _rt::string_lift(bytes14)
                                                };
                                                V21::Encode(e21)
                                            }
                                            2 => {
                                                let e21 = {
                                                    let l15 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l16 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len17 = l16;
                                                    let bytes17 = _rt::Vec::from_raw_parts(
                                                        l15.cast(),
                                                        len17,
                                                        len17,
                                                    );
                                                    _rt::string_lift(bytes17)
                                                };
                                                V21::Decode(e21)
                                            }
                                            3 => V21::PoolTimedOut,
                                            4 => V21::PoolClosed,
                                            5 => V21::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e21 = {
                                                    let l18 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l19 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len20 = l19;
                                                    let bytes20 = _rt::Vec::from_raw_parts(
                                                        l18.cast(),
                                                        len20,
                                                        len20,
                                                    );
                                                    _rt::string_lift(bytes20)
                                                };
                                                V21::Unexpected(e21)
                                            }
                                        };
                                        v21
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result22
                        }
                    }
                }
                #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
                pub mod postgres_codecs_ext {
                    #[used]
                    #[doc(hidden)]
                    static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
                    use super::super::super::_rt;
                    pub type SqlArguments = super::super::super::__with_name0::SqlArguments;
                    pub type QueryResults = super::super::super::__with_name0::QueryResults;
                    pub type Error = super::super::super::__with_name1::Error;
                    pub type PushResult = super::super::super::__with_name2::PushResult;
                    pub type ValuePosition = super::super::super::__with_name2::ValuePosition;
                    pub type Hstore = _rt::Vec<(_rt::String, Option<_rt::String>)>;
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn push_hstore(
                        value: Option<&[(_rt::String, Option<_rt::String>)]>,
                        to: &SqlArguments,
                    ) -> PushResult {
                        unsafe {
                            let mut cleanup_list = _rt::Vec::new();
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (result4_0, result4_1, result4_2) = match value {
                                Some(e) => {
                                    let vec3 = e;
                                    let len3 = vec3.len();
                                    let layout3 = _rt::alloc::Layout::from_size_align(
                                            vec3.len() * (5 * ::core::mem::size_of::<*const u8>()),
                                            ::core::mem::size_of::<*const u8>(),
                                        )
                                        .unwrap();
                                    let (result3, _cleanup3) = wit_bindgen::rt::Cleanup::new(
                                        layout3,
                                    );
                                    cleanup_list.extend(_cleanup3);
                                    for (i, e) in vec3.into_iter().enumerate() {
                                        let base = result3
                                            .add(i * (5 * ::core::mem::size_of::<*const u8>()));
                                        {
                                            let (t0_0, t0_1) = e;
                                            let vec1 = t0_0;
                                            let ptr1 = vec1.as_ptr().cast::<u8>();
                                            let len1 = vec1.len();
                                            *base
                                                .add(::core::mem::size_of::<*const u8>())
                                                .cast::<usize>() = len1;
                                            *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                            match t0_1 {
                                                Some(e) => {
                                                    *base
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<u8>() = (1i32) as u8;
                                                    let vec2 = e;
                                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                                    let len2 = vec2.len();
                                                    *base
                                                        .add(4 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>() = len2;
                                                    *base
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>() = ptr2.cast_mut();
                                                }
                                                None => {
                                                    *base
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<u8>() = (0i32) as u8;
                                                }
                                            };
                                        }
                                    }
                                    (1i32, result3, len3)
                                }
                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                            };
                            let ptr5 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import6(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import6(
                                result4_0,
                                result4_1,
                                result4_2,
                                (to).handle() as i32,
                                ptr5,
                            );
                            let l7 = i32::from(*ptr5.add(0).cast::<u8>());
                            let result19 = match l7 {
                                0 => {
                                    let e = ();
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l8 = i32::from(
                                            *ptr5.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V18;
                                        let v18 = match l8 {
                                            0 => V18::RowNotFound,
                                            1 => {
                                                let e18 = {
                                                    let l9 = *ptr5
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l10 = *ptr5
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len11 = l10;
                                                    let bytes11 = _rt::Vec::from_raw_parts(
                                                        l9.cast(),
                                                        len11,
                                                        len11,
                                                    );
                                                    _rt::string_lift(bytes11)
                                                };
                                                V18::Encode(e18)
                                            }
                                            2 => {
                                                let e18 = {
                                                    let l12 = *ptr5
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l13 = *ptr5
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len14 = l13;
                                                    let bytes14 = _rt::Vec::from_raw_parts(
                                                        l12.cast(),
                                                        len14,
                                                        len14,
                                                    );
                                                    _rt::string_lift(bytes14)
                                                };
                                                V18::Decode(e18)
                                            }
                                            3 => V18::PoolTimedOut,
                                            4 => V18::PoolClosed,
                                            5 => V18::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e18 = {
                                                    let l15 = *ptr5
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l16 = *ptr5
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len17 = l16;
                                                    let bytes17 = _rt::Vec::from_raw_parts(
                                                        l15.cast(),
                                                        len17,
                                                        len17,
                                                    );
                                                    _rt::string_lift(bytes17)
                                                };
                                                V18::Unexpected(e18)
                                            }
                                        };
                                        v18
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result19
                        }
                    }
                    #[allow(unused_unsafe, clippy::all)]
                    #[allow(async_fn_in_trait)]
                    pub fn get_hstore(
                        result: &QueryResults,
                        position: &ValuePosition,
                    ) -> Result<Option<Hstore>, Error> {
                        unsafe {
                            #[repr(align(8))]
                            struct RetArea(
                                [::core::mem::MaybeUninit<
                                    u8,
                                >; 4 * ::core::mem::size_of::<*const u8>()],
                            );
                            let mut ret_area = RetArea(
                                [::core::mem::MaybeUninit::uninit(); 4
                                    * ::core::mem::size_of::<*const u8>()],
                            );
                            let (t0_0, t0_1) = position;
                            use super::super::super::__with_name2::ColumnIndex as V2;
                            let (result3_0, result3_1, result3_2) = match t0_1 {
                                V2::Index(e) => {
                                    (
                                        0i32,
                                        ::core::mem::MaybeUninit::new(_rt::as_i64(e) as u64),
                                        0usize,
                                    )
                                }
                                V2::ColumnName(e) => {
                                    let vec1 = e;
                                    let ptr1 = vec1.as_ptr().cast::<u8>();
                                    let len1 = vec1.len();
                                    (
                                        1i32,
                                        {
                                            let mut t = ::core::mem::MaybeUninit::<u64>::uninit();
                                            t.as_mut_ptr().cast::<*mut u8>().write(ptr1.cast_mut());
                                            t
                                        },
                                        len1,
                                    )
                                }
                            };
                            let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                            unsafe extern "C" fn wit_import5(
                                _: i32,
                                _: i64,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: usize,
                                _: *mut u8,
                            ) {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                            wit_import5(
                                (result).handle() as i32,
                                _rt::as_i64(t0_0),
                                result3_0,
                                result3_1,
                                result3_2,
                                ptr4,
                            );
                            let l6 = i32::from(*ptr4.add(0).cast::<u8>());
                            let result29 = match l6 {
                                0 => {
                                    let e = {
                                        let l7 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l9 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let base17 = l8;
                                                    let len17 = l9;
                                                    let mut result17 = _rt::Vec::with_capacity(len17);
                                                    for i in 0..len17 {
                                                        let base = base17
                                                            .add(i * (5 * ::core::mem::size_of::<*const u8>()));
                                                        let e17 = {
                                                            let l10 = *base.add(0).cast::<*mut u8>();
                                                            let l11 = *base
                                                                .add(::core::mem::size_of::<*const u8>())
                                                                .cast::<usize>();
                                                            let len12 = l11;
                                                            let bytes12 = _rt::Vec::from_raw_parts(
                                                                l10.cast(),
                                                                len12,
                                                                len12,
                                                            );
                                                            let l13 = i32::from(
                                                                *base
                                                                    .add(2 * ::core::mem::size_of::<*const u8>())
                                                                    .cast::<u8>(),
                                                            );
                                                            (
                                                                _rt::string_lift(bytes12),
                                                                match l13 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l14 = *base
                                                                                .add(3 * ::core::mem::size_of::<*const u8>())
                                                                                .cast::<*mut u8>();
                                                                            let l15 = *base
                                                                                .add(4 * ::core::mem::size_of::<*const u8>())
                                                                                .cast::<usize>();
                                                                            let len16 = l15;
                                                                            let bytes16 = _rt::Vec::from_raw_parts(
                                                                                l14.cast(),
                                                                                len16,
                                                                                len16,
                                                                            );
                                                                            _rt::string_lift(bytes16)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                            )
                                                        };
                                                        result17.push(e17);
                                                    }
                                                    _rt::cabi_dealloc(
                                                        base17,
                                                        len17 * (5 * ::core::mem::size_of::<*const u8>()),
                                                        ::core::mem::size_of::<*const u8>(),
                                                    );
                                                    result17
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        }
                                    };
                                    Ok(e)
                                }
                                1 => {
                                    let e = {
                                        let l18 = i32::from(
                                            *ptr4.add(::core::mem::size_of::<*const u8>()).cast::<u8>(),
                                        );
                                        use super::super::super::__with_name1::Error as V28;
                                        let v28 = match l18 {
                                            0 => V28::RowNotFound,
                                            1 => {
                                                let e28 = {
                                                    let l19 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l20 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len21 = l20;
                                                    let bytes21 = _rt::Vec::from_raw_parts(
                                                        l19.cast(),
                                                        len21,
                                                        len21,
                                                    );
                                                    _rt::string_lift(bytes21)
                                                };
                                                V28::Encode(e28)
                                            }
                                            2 => {
                                                let e28 = {
                                                    let l22 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l23 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len24 = l23;
                                                    let bytes24 = _rt::Vec::from_raw_parts(
                                                        l22.cast(),
                                                        len24,
                                                        len24,
                                                    );
                                                    _rt::string_lift(bytes24)
                                                };
                                                V28::Decode(e28)
                                            }
                                            3 => V28::PoolTimedOut,
                                            4 => V28::PoolClosed,
                                            5 => V28::BeginFailed,
                                            n => {
                                                if true {
                                                    match (&n, &6) {
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
                                                let e28 = {
                                                    let l25 = *ptr4
                                                        .add(2 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<*mut u8>();
                                                    let l26 = *ptr4
                                                        .add(3 * ::core::mem::size_of::<*const u8>())
                                                        .cast::<usize>();
                                                    let len27 = l26;
                                                    let bytes27 = _rt::Vec::from_raw_parts(
                                                        l25.cast(),
                                                        len27,
                                                        len27,
                                                    );
                                                    _rt::string_lift(bytes27)
                                                };
                                                V28::Unexpected(e28)
                                            }
                                        };
                                        v28
                                    };
                                    Err(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            };
                            result29
                        }
                    }
                }
            }
        }
        mod _rt {
            #![allow(dead_code, clippy::all)]
            pub use alloc_crate::string::String;
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
            pub use alloc_crate::vec::Vec;
            pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
                if true {
                    String::from_utf8(bytes).unwrap()
                } else {
                    unsafe { String::from_utf8_unchecked(bytes) }
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
            pub fn as_f32<T: AsF32>(t: T) -> f32 {
                t.as_f32()
            }
            pub trait AsF32 {
                fn as_f32(self) -> f32;
            }
            impl<'a, T: Copy + AsF32> AsF32 for &'a T {
                fn as_f32(self) -> f32 {
                    (*self).as_f32()
                }
            }
            impl AsF32 for f32 {
                #[inline]
                fn as_f32(self) -> f32 {
                    self as f32
                }
            }
            pub fn as_f64<T: AsF64>(t: T) -> f64 {
                t.as_f64()
            }
            pub trait AsF64 {
                fn as_f64(self) -> f64;
            }
            impl<'a, T: Copy + AsF64> AsF64 for &'a T {
                fn as_f64(self) -> f64 {
                    (*self).as_f64()
                }
            }
            impl AsF64 for f64 {
                #[inline]
                fn as_f64(self) -> f64 {
                    self as f64
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
            pub use alloc_crate::alloc;
            pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
                if size == 0 {
                    return;
                }
                unsafe {
                    let layout = alloc::Layout::from_size_align_unchecked(size, align);
                    alloc::dealloc(ptr, layout);
                }
            }
            extern crate alloc as alloc_crate;
        }
        #[inline(never)]
        #[doc(hidden)]
        pub fn __link_custom_section_describing_imports() {
            wit_bindgen::rt::maybe_link_cabi_realloc();
        }
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface codecs {\n  use util-types.{error};\n\n  type push-result = result<_, error>;\n\n  type row-index = u64;\n\n  variant column-index {\n    index(u64),\n    column-name(string)\n  }\n\n  type value-position = tuple<row-index, column-index>;\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  begin-transaction: async func() -> result<transaction, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}";
        const _: &[u8] = b"package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// no rows returned by a query that expected to return at least one row.\n    row-not-found,\n\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results {\n    row-count: func() -> u64;\n  }\n}";
        const _: &[u8] = b"package wasmledger:sql-postgres;\n\ninterface postgres-codecs {\n  use wasmledger:sql/query-types.{sql-arguments, query-results};\n  use wasmledger:sql/util-types.{error};\n  use wasmledger:sql/codecs.{push-result, value-position};\n\n  push-int16: func(value: option<s16>, to: borrow<sql-arguments>) -> push-result;\n  get-int16: func(%result: borrow<query-results>, position: value-position) -> result<option<s16>, error>;\n\n  push-int32: func(value: option<s32>, to: borrow<sql-arguments>) -> push-result;\n  get-int32: func(%result: borrow<query-results>, position: value-position) -> result<option<s32>, error>;\n\n  push-int64: func(value: option<s64>, to: borrow<sql-arguments>) -> push-result;\n  get-int64: func(%result: borrow<query-results>, position: value-position) -> result<option<s64>, error>;\n\n  push-float32: func(value: option<f32>, to: borrow<sql-arguments>) -> push-result;\n  get-float32: func(%result: borrow<query-results>, position: value-position) -> result<option<f32>, error>;\n\n  push-float64: func(value: option<f64>, to: borrow<sql-arguments>) -> push-result;\n  get-float64: func(%result: borrow<query-results>, position: value-position) -> result<option<f64>, error>;\n\n  push-string: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-string: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n  push-bool: func(value: option<bool>, to: borrow<sql-arguments>) -> push-result;\n  get-bool: func(%result: borrow<query-results>, position: value-position) -> result<option<bool>, error>;\n\n  push-json: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-json: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n  type uuid = string;\n\n  push-uuid: func(value: option<uuid>, to: borrow<sql-arguments>) -> push-result;\n  get-uuid: func(%result: borrow<query-results>, position: value-position) -> result<option<uuid>, error>;\n\n\n  // variant pg-value {\n  //   /// SQL: NULL\n  //   null,\n\n  //   /// SQL: BIGINT, INT8\n  //   int64(s64),\n  //   /// SQL: BIGINT[], INT8[]\n  //   int64-array(list<s64>),\n\n  //   /// SQL: INTEGER, INT, INT4\n  //   int32(s32),\n  //   /// SQL: INTEGER[], INT4[]\n  //   int32-array(list<s32>),\n\n  //   /// SQL: SMALLINT, INT2\n  //   int2(s16),\n  //   /// SQL: SMALLINT[], INT2[]\n  //   int2-array(list<s16>),\n\n  //   /// SQL: DOUBLE PRECISION, FLOAT8\n  //   float8(hashable-f64),\n  //   /// SQL: DOUBLE PRECISION[], FLOAT8[]\n  //   float8-array(list<hashable-f64>),\n\n  //   /// SQL: REAL, FLOAT4\n  //   float4(hashable-f32),\n  //   /// SQL: REAL[], FLOAT4[]\n  //   float4-array(list<hashable-f32>),\n\n  //   /// SQL: BOOLEAN, BOOL\n  //   %bool(bool),\n  //   /// SQL: BOOLEAN[], BOOL[]\n  //   %bool-array(list<bool>),\n\n  //   /// SQL: NUMERIC, DECIMAL\n  //   numeric(numeric),\n  //   /// SQL: NUMERIC[], DECIMAL[]\n  //   numeric-array(list<numeric>),\n\n  //   /// SQL: BIT(n)\n  //   bit(list<u8>),\n  //   /// SQL: BIT(n)[]\n  //   bit-array(list<list<u8>>),\n\n  //   /// SQL: VARBIT(n)\n  //   varbit(list<u8>),\n  //   /// SQL: BIT VARYING[], VARBIT[]\n  //   varbit-array(list<list<u8>>),\n\n  //   /// SQL: BYTEA\n  //   bytea(list<u8>),\n  //   /// SQL: BYTEA[]\n  //   bytea-array(list<list<u8>>),\n\n  //   /// SQL: CHAR(n), CHARACTER(n)\n  //   %char(string),\n  //   /// SQL: CHAR(n)[]\n  //   %char-array(list<string>),\n\n  //   /// SQL: VARCHAR(n), CHARACTER VARYING(n)\n  //   varchar(string),\n  //   /// SQL: VARCHAR(n)[]\n  //   varchar-array(list<string>),\n\n  //   // Networking\n  //   /// SQL: CIDR\n  //   cidr(string),\n  //   /// SQL: CIDR[]\n  //   cidr-array(list<string>),\n\n  //   /// SQL: INET\n  //   inet(string),\n  //   /// SQL: INET[]\n  //   inet-array(list<string>),\n\n  //   /// SQL: MACADDR (EUI-48)\n  //   macaddr(mac-address-eui48),\n  //   /// SQL: MACADDR[]\n  //   macaddr-array(list<mac-address-eui48>),\n\n  //   /// SQL: MACADDR8 (EUI-64, deprecated)\n  //   macaddr8(mac-address-eui64),\n  //   /// SQL: MACADDR8[]\n  //   macaddr8-array(list<mac-address-eui64>),\n\n  //   // Date-time\n  //   /// SQL: DATE\n  //   date(date),\n  //   /// SQL: DATE[]\n  //   date-array(list<date>),\n\n  //   /// SQL: INTERVAL\n  //   interval(interval),\n  //   /// SQL: INTERVAL[]\n  //   interval-array(list<interval>),\n\n  //   /// SQL: TIME WITHOUT TIME ZONE\n  //   time(time),\n  //   /// SQL: TIME[]\n  //   time-array(list<time>),\n\n  //   /// SQL: TIME WITH TIME ZONE\n  //   time-tz(time-tz),\n  //   /// SQL: TIMETZ[]\n  //   time-tz-array(list<time-tz>),\n\n  //   /// SQL: TIMESTAMP WITHOUT TIME ZONE\n  //   timestamp(timestamp),\n  //   /// SQL: TIMESTAMP[]\n  //   timestamp-array(list<timestamp>),\n\n  //   /// SQL: TIMESTAMP WITH TIME ZONE, TIMESTAMPTZ\n  //   timestamp-tz(timestamp-tz),\n  //   /// SQL: TIMESTAMPTZ[]\n  //   timestamp-tz-array(list<timestamp-tz>),\n\n  //   // JSON\n  //   /// SQL: JSON\n  //   json(string),\n  //   /// SQL: JSON[]\n  //   json-array(list<string>),\n\n  //   /// SQL: JSONB\n  //   jsonb(string),\n  //   /// SQL: JSONB[]\n  //   jsonb-array(list<string>),\n\n  //   /// SQL: MONEY (internal fixed-point type)\n  //   money(numeric),\n  //   /// SQL: MONEY[]\n  //   money-array(list<numeric>),\n\n  //   // Text\n  //   /// SQL: NAME (system identifier type)\n  //   name(string),\n  //   /// SQL: NAME[]\n  //   name-array(list<string>),\n\n  //   /// SQL: TEXT\n  //   text(string),\n  //   /// SQL: TEXT[]\n  //   text-array(list<string>),\n\n  //   /// SQL: XML\n  //   xml(string),\n  //   /// SQL: XML[]\n  //   xml-array(list<string>),\n\n  //   // UUIDs\n  //   /// SQL: UUID\n  //   uuid(string),\n  //   /// SQL: UUID[]\n  //   uuid-array(list<string>),\n\n  //   // Containers\n  //   /// SQL: HSTORE (extension)\n  //   hstore(list<tuple<string, option<string>>>),\n  // }\n}";
        const _: &[u8] = b"package wasmledger:sql-postgres;\n\ninterface postgres-codecs-ext {\n  use wasmledger:sql/query-types.{sql-arguments, query-results};\n  use wasmledger:sql/util-types.{error};\n  use wasmledger:sql/codecs.{push-result, value-position};\n\n  type hstore = list<tuple<string, option<string>>>;\n\n  push-hstore: func(value: option<hstore>, to: borrow<sql-arguments>) -> push-result;\n  get-hstore: func(%result: borrow<query-results>, position: value-position) -> result<option<hstore>, error>;\n}";
        const _: &[u8] = b"package wasmledger:sql-client;\n\nworld client-postgres {\n  import wasmledger:sql-postgres/postgres-codecs;\n  import wasmledger:sql-postgres/postgres-codecs-ext;\n}";
    }
}

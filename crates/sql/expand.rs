#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod core {
    pub mod bindings {
        use wasmtime::component::{HasData, ResourceTable};
        #[doc(hidden)]
        pub use crate::core::bindings::connection::ConnectionImpl as __with_name0;
        #[doc(hidden)]
        pub use crate::core::bindings::query_results::QueryResultsImpl as __with_name1;
        #[doc(hidden)]
        pub use crate::core::bindings::sql_arguments::SqlArgumentsImpl as __with_name2;
        #[doc(hidden)]
        pub use crate::core::bindings::transaction::TransactionImpl as __with_name3;
        /// Auto-generated bindings for a pre-instantiated version of a
        /// component which implements the world `host`.
        ///
        /// This structure is created through [`Host_Pre::new`] which
        /// takes a [`InstancePre`](wasmtime::component::InstancePre) that
        /// has been created through a [`Linker`](wasmtime::component::Linker).
        ///
        /// For more information see [`Host_`] as well.
        pub struct Host_Pre<T: 'static> {
            instance_pre: wasmtime::component::InstancePre<T>,
            indices: Host_Indices,
        }
        impl<T: 'static> Clone for Host_Pre<T> {
            fn clone(&self) -> Self {
                Self {
                    instance_pre: self.instance_pre.clone(),
                    indices: self.indices.clone(),
                }
            }
        }
        impl<_T: 'static> Host_Pre<_T> {
            /// Creates a new copy of `Host_Pre` bindings which can then
            /// be used to instantiate into a particular store.
            ///
            /// This method may fail if the component behind `instance_pre`
            /// does not have the required exports.
            pub fn new(
                instance_pre: wasmtime::component::InstancePre<_T>,
            ) -> wasmtime::Result<Self> {
                let indices = Host_Indices::new(&instance_pre)?;
                Ok(Self { instance_pre, indices })
            }
            pub fn engine(&self) -> &wasmtime::Engine {
                self.instance_pre.engine()
            }
            pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
                &self.instance_pre
            }
            /// Instantiates a new instance of [`Host_`] within the
            /// `store` provided.
            ///
            /// This function will use `self` as the pre-instantiated
            /// instance to perform instantiation. Afterwards the preloaded
            /// indices in `self` are used to lookup all exports on the
            /// resulting instance.
            pub fn instantiate(
                &self,
                mut store: impl wasmtime::AsContextMut<Data = _T>,
            ) -> wasmtime::Result<Host_> {
                let mut store = store.as_context_mut();
                let instance = self.instance_pre.instantiate(&mut store)?;
                self.indices.load(&mut store, &instance)
            }
        }
        impl<_T: Send + 'static> Host_Pre<_T> {
            /// Same as [`Self::instantiate`], except with `async`.
            pub async fn instantiate_async(
                &self,
                mut store: impl wasmtime::AsContextMut<Data = _T>,
            ) -> wasmtime::Result<Host_> {
                let mut store = store.as_context_mut();
                let instance = self.instance_pre.instantiate_async(&mut store).await?;
                self.indices.load(&mut store, &instance)
            }
        }
        /// Auto-generated bindings for index of the exports of
        /// `host`.
        ///
        /// This is an implementation detail of [`Host_Pre`] and can
        /// be constructed if needed as well.
        ///
        /// For more information see [`Host_`] as well.
        pub struct Host_Indices {}
        #[automatically_derived]
        impl ::core::clone::Clone for Host_Indices {
            #[inline]
            fn clone(&self) -> Host_Indices {
                Host_Indices {}
            }
        }
        /// Auto-generated bindings for an instance a component which
        /// implements the world `host`.
        ///
        /// This structure can be created through a number of means
        /// depending on your requirements and what you have on hand:
        ///
        /// * The most convenient way is to use
        ///   [`Host_::instantiate`] which only needs a
        ///   [`Store`], [`Component`], and [`Linker`].
        ///
        /// * Alternatively you can create a [`Host_Pre`] ahead of
        ///   time with a [`Component`] to front-load string lookups
        ///   of exports once instead of per-instantiation. This
        ///   method then uses [`Host_Pre::instantiate`] to
        ///   create a [`Host_`].
        ///
        /// * If you've instantiated the instance yourself already
        ///   then you can use [`Host_::new`].
        ///
        /// These methods are all equivalent to one another and move
        /// around the tradeoff of what work is performed when.
        ///
        /// [`Store`]: wasmtime::Store
        /// [`Component`]: wasmtime::component::Component
        /// [`Linker`]: wasmtime::component::Linker
        pub struct Host_ {}
        const _: () = {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            impl Host_Indices {
                /// Creates a new copy of `Host_Indices` bindings which can then
                /// be used to instantiate into a particular store.
                ///
                /// This method may fail if the component does not have the
                /// required exports.
                pub fn new<_T>(
                    _instance_pre: &wasmtime::component::InstancePre<_T>,
                ) -> wasmtime::Result<Self> {
                    let _component = _instance_pre.component();
                    let _instance_type = _instance_pre.instance_type();
                    Ok(Host_Indices {})
                }
                /// Uses the indices stored in `self` to load an instance
                /// of [`Host_`] from the instance provided.
                ///
                /// Note that at this time this method will additionally
                /// perform type-checks of all exports.
                pub fn load(
                    &self,
                    mut store: impl wasmtime::AsContextMut,
                    instance: &wasmtime::component::Instance,
                ) -> wasmtime::Result<Host_> {
                    let _ = &mut store;
                    let _instance = instance;
                    Ok(Host_ {})
                }
            }
            impl Host_ {
                /// Convenience wrapper around [`Host_Pre::new`] and
                /// [`Host_Pre::instantiate`].
                pub fn instantiate<_T>(
                    store: impl wasmtime::AsContextMut<Data = _T>,
                    component: &wasmtime::component::Component,
                    linker: &wasmtime::component::Linker<_T>,
                ) -> wasmtime::Result<Host_> {
                    let pre = linker.instantiate_pre(component)?;
                    Host_Pre::new(pre)?.instantiate(store)
                }
                /// Convenience wrapper around [`Host_Indices::new`] and
                /// [`Host_Indices::load`].
                pub fn new(
                    mut store: impl wasmtime::AsContextMut,
                    instance: &wasmtime::component::Instance,
                ) -> wasmtime::Result<Host_> {
                    let indices = Host_Indices::new(&instance.instance_pre(&store))?;
                    indices.load(&mut store, instance)
                }
                /// Convenience wrapper around [`Host_Pre::new`] and
                /// [`Host_Pre::instantiate_async`].
                pub async fn instantiate_async<_T>(
                    store: impl wasmtime::AsContextMut<Data = _T>,
                    component: &wasmtime::component::Component,
                    linker: &wasmtime::component::Linker<_T>,
                ) -> wasmtime::Result<Host_>
                where
                    _T: Send,
                {
                    let pre = linker.instantiate_pre(component)?;
                    Host_Pre::new(pre)?.instantiate_async(store).await
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: wasmledger::sql::util_types::HostWithStore
                        + wasmledger::sql::transaction::HostWithStore
                        + wasmledger::sql::connection::HostWithStore
                        + wasmledger::sql::pool::HostWithStore
                        + wasmledger::sql::query_types::HostWithStore
                        + wasmledger::sql::query::HostWithStore + Send,
                    for<'a> D::Data<
                        'a,
                    >: wasmledger::sql::util_types::Host
                        + wasmledger::sql::transaction::Host
                        + wasmledger::sql::connection::Host + wasmledger::sql::pool::Host
                        + wasmledger::sql::query_types::Host
                        + wasmledger::sql::query::Host + Send,
                    T: 'static + Send,
                {
                    wasmledger::sql::util_types::add_to_linker::<
                        T,
                        D,
                    >(linker, host_getter)?;
                    wasmledger::sql::transaction::add_to_linker::<
                        T,
                        D,
                    >(linker, host_getter)?;
                    wasmledger::sql::connection::add_to_linker::<
                        T,
                        D,
                    >(linker, host_getter)?;
                    wasmledger::sql::pool::add_to_linker::<T, D>(linker, host_getter)?;
                    wasmledger::sql::query_types::add_to_linker::<
                        T,
                        D,
                    >(linker, host_getter)?;
                    wasmledger::sql::query::add_to_linker::<T, D>(linker, host_getter)?;
                    Ok(())
                }
            }
        };
        pub mod wasmledger {
            pub mod sql {
                #[allow(clippy::all)]
                pub mod util_types {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    #[component(variant)]
                    pub enum Error {
                        /// error occurred while encoding a value
                        #[component(name = "encode")]
                        Encode(wasmtime::component::__internal::String),
                        /// error occurred while decoding
                        #[component(name = "decode")]
                        Decode(wasmtime::component::__internal::String),
                        /// pool timed out while waiting for an open connection
                        #[component(name = "pool-timed-out")]
                        PoolTimedOut,
                        /// attempted to acquire a connection on a closed pool
                        #[component(name = "pool-closed")]
                        PoolClosed,
                        /// got unexpected connection status after attempting to begin transaction
                        #[component(name = "begin-failed")]
                        BeginFailed,
                        /// attempted to perform operation on already closed transaction
                        #[component(name = "transaction-closed")]
                        TransactionClosed,
                        /// generic unexpected error
                        #[component(name = "unexpected")]
                        Unexpected(wasmtime::component::__internal::String),
                    }
                    #[automatically_derived]
                    impl ::core::clone::Clone for Error {
                        #[inline]
                        fn clone(&self) -> Error {
                            match self {
                                Error::Encode(__self_0) => {
                                    Error::Encode(::core::clone::Clone::clone(__self_0))
                                }
                                Error::Decode(__self_0) => {
                                    Error::Decode(::core::clone::Clone::clone(__self_0))
                                }
                                Error::PoolTimedOut => Error::PoolTimedOut,
                                Error::PoolClosed => Error::PoolClosed,
                                Error::BeginFailed => Error::BeginFailed,
                                Error::TransactionClosed => Error::TransactionClosed,
                                Error::Unexpected(__self_0) => {
                                    Error::Unexpected(::core::clone::Clone::clone(__self_0))
                                }
                            }
                        }
                    }
                    unsafe impl wasmtime::component::Lower for Error {
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
                                Self::Encode(value) => {
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
                                                        m.map(|p| &raw mut (*p).Encode)
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
                                Self::Decode(value) => {
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
                                                        m.map(|p| &raw mut (*p).Decode)
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
                                Self::PoolTimedOut => {
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
                                        .write(wasmtime::ValRaw::u32(2u32));
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
                                                        m.map(|p| &raw mut (*p).PoolTimedOut)
                                                    }
                                                }
                                            },
                                            |dst| Ok(()),
                                        )
                                    }
                                }
                                Self::PoolClosed => {
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
                                        .write(wasmtime::ValRaw::u32(3u32));
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
                                                        m.map(|p| &raw mut (*p).PoolClosed)
                                                    }
                                                }
                                            },
                                            |dst| Ok(()),
                                        )
                                    }
                                }
                                Self::BeginFailed => {
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
                                        .write(wasmtime::ValRaw::u32(4u32));
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
                                                        m.map(|p| &raw mut (*p).BeginFailed)
                                                    }
                                                }
                                            },
                                            |dst| Ok(()),
                                        )
                                    }
                                }
                                Self::TransactionClosed => {
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
                                        .write(wasmtime::ValRaw::u32(5u32));
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
                                                        m.map(|p| &raw mut (*p).TransactionClosed)
                                                    }
                                                }
                                            },
                                            |dst| Ok(()),
                                        )
                                    }
                                }
                                Self::Unexpected(value) => {
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
                                        .write(wasmtime::ValRaw::u32(6u32));
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
                                                        m.map(|p| &raw mut (*p).Unexpected)
                                                    }
                                                }
                                            },
                                            |dst| {
                                                value
                                                    .linear_lower_to_flat(
                                                        cx,
                                                        ty
                                                            .cases[6usize]
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
                                Self::Encode(value) => {
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
                                Self::Decode(value) => {
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
                                Self::PoolTimedOut => {
                                    *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                    Ok(())
                                }
                                Self::PoolClosed => {
                                    *cx.get::<1usize>(offset) = 3u8.to_le_bytes();
                                    Ok(())
                                }
                                Self::BeginFailed => {
                                    *cx.get::<1usize>(offset) = 4u8.to_le_bytes();
                                    Ok(())
                                }
                                Self::TransactionClosed => {
                                    *cx.get::<1usize>(offset) = 5u8.to_le_bytes();
                                    Ok(())
                                }
                                Self::Unexpected(value) => {
                                    *cx.get::<1usize>(offset) = 6u8.to_le_bytes();
                                    value
                                        .linear_lower_to_memory(
                                            cx,
                                            ty
                                                .cases[6usize]
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
                    unsafe impl wasmtime::component::Lift for Error {
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
                                        Self::Encode(
                                            <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_flat(
                                                cx,
                                                ty
                                                    .cases[0usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                unsafe { &src.payload.Encode },
                                            )?,
                                        )
                                    }
                                    1u32 => {
                                        Self::Decode(
                                            <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_flat(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                unsafe { &src.payload.Decode },
                                            )?,
                                        )
                                    }
                                    2u32 => Self::PoolTimedOut,
                                    3u32 => Self::PoolClosed,
                                    4u32 => Self::BeginFailed,
                                    5u32 => Self::TransactionClosed,
                                    6u32 => {
                                        Self::Unexpected(
                                            <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_flat(
                                                cx,
                                                ty
                                                    .cases[6usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                unsafe { &src.payload.Unexpected },
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
                                        Self::Encode(
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
                                        Self::Decode(
                                            <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_memory(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                &payload[..<wasmtime::component::__internal::String as wasmtime::component::ComponentType>::SIZE32],
                                            )?,
                                        )
                                    }
                                    2u8 => Self::PoolTimedOut,
                                    3u8 => Self::PoolClosed,
                                    4u8 => Self::BeginFailed,
                                    5u8 => Self::TransactionClosed,
                                    6u8 => {
                                        Self::Unexpected(
                                            <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_memory(
                                                cx,
                                                ty
                                                    .cases[6usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                &payload[..<wasmtime::component::__internal::String as wasmtime::component::ComponentType>::SIZE32],
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
                        pub struct LowerError<T0: Copy, T1: Copy, T6: Copy> {
                            tag: wasmtime::ValRaw,
                            payload: LowerPayloadError<T0, T1, T6>,
                        }
                        #[automatically_derived]
                        impl<
                            T0: ::core::clone::Clone + Copy,
                            T1: ::core::clone::Clone + Copy,
                            T6: ::core::clone::Clone + Copy,
                        > ::core::clone::Clone for LowerError<T0, T1, T6> {
                            #[inline]
                            fn clone(&self) -> LowerError<T0, T1, T6> {
                                LowerError {
                                    tag: ::core::clone::Clone::clone(&self.tag),
                                    payload: ::core::clone::Clone::clone(&self.payload),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<
                            T0: ::core::marker::Copy + Copy,
                            T1: ::core::marker::Copy + Copy,
                            T6: ::core::marker::Copy + Copy,
                        > ::core::marker::Copy for LowerError<T0, T1, T6> {}
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        #[repr(C)]
                        union LowerPayloadError<T0: Copy, T1: Copy, T6: Copy> {
                            Encode: T0,
                            Decode: T1,
                            PoolTimedOut: [wasmtime::ValRaw; 0],
                            PoolClosed: [wasmtime::ValRaw; 0],
                            BeginFailed: [wasmtime::ValRaw; 0],
                            TransactionClosed: [wasmtime::ValRaw; 0],
                            Unexpected: T6,
                        }
                        #[automatically_derived]
                        #[allow(non_snake_case)]
                        impl<
                            T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
                            T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                            T6: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        > ::core::clone::Clone for LowerPayloadError<T0, T1, T6> {
                            #[inline]
                            fn clone(&self) -> LowerPayloadError<T0, T1, T6> {
                                let _: ::core::clone::AssertParamIsCopy<Self>;
                                *self
                            }
                        }
                        #[automatically_derived]
                        #[allow(non_snake_case)]
                        impl<
                            T0: ::core::marker::Copy + Copy,
                            T1: ::core::marker::Copy + Copy,
                            T6: ::core::marker::Copy + Copy,
                        > ::core::marker::Copy for LowerPayloadError<T0, T1, T6> {}
                        unsafe impl wasmtime::component::ComponentType for Error {
                            type Lower = LowerError<
                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::Lower,
                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::Lower,
                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::Lower,
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
                                            "encode",
                                            Some(
                                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::typecheck,
                                            ),
                                        ),
                                        (
                                            "decode",
                                            Some(
                                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::typecheck,
                                            ),
                                        ),
                                        ("pool-timed-out", None),
                                        ("pool-closed", None),
                                        ("begin-failed", None),
                                        ("transaction-closed", None),
                                        (
                                            "unexpected",
                                            Some(
                                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::typecheck,
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
                                    Some(
                                        <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                    ),
                                    None,
                                    None,
                                    None,
                                    None,
                                    Some(
                                        <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                    ),
                                ],
                            );
                        }
                        unsafe impl wasmtime::component::__internal::ComponentVariant
                        for Error {
                            const CASES: &'static [Option<
                                wasmtime::component::__internal::CanonicalAbiInfo,
                            >] = &[
                                Some(
                                    <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                ),
                                Some(
                                    <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                ),
                                None,
                                None,
                                None,
                                None,
                                Some(
                                    <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                ),
                            ];
                        }
                    };
                    impl core::fmt::Debug for Error {
                        fn fmt(
                            &self,
                            f: &mut core::fmt::Formatter<'_>,
                        ) -> core::fmt::Result {
                            match self {
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
                                Error::TransactionClosed => {
                                    f.debug_tuple("Error::TransactionClosed").finish()
                                }
                                Error::Unexpected(e) => {
                                    f.debug_tuple("Error::Unexpected").field(e).finish()
                                }
                            }
                        }
                    }
                    impl core::fmt::Display for Error {
                        fn fmt(
                            &self,
                            f: &mut core::fmt::Formatter<'_>,
                        ) -> core::fmt::Result {
                            f.write_fmt(format_args!("{0:?}", self))
                        }
                    }
                    impl core::error::Error for Error {}
                    const _: () = {
                        if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub trait HostWithStore: wasmtime::component::HasData {}
                    impl<_T: ?Sized> HostWithStore for _T
                    where
                        _T: wasmtime::component::HasData,
                    {}
                    pub trait Host {}
                    impl<_T: Host + ?Sized> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static,
                    {
                        let mut inst = linker.instance("wasmledger:sql/util-types")?;
                        Ok(())
                    }
                }
                #[allow(clippy::all)]
                pub mod transaction {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
                    const _: () = {
                        if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub use super::super::super::__with_name3 as Transaction;
                    pub trait HostTransactionWithStore: wasmtime::component::HasData + Send {
                        fn commit<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                            this: wasmtime::component::Resource<Transaction>,
                        ) -> impl ::core::future::Future<
                            Output = Result<(), Error>,
                        > + Send;
                        fn rollback<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                            this: wasmtime::component::Resource<Transaction>,
                        ) -> impl ::core::future::Future<
                            Output = Result<(), Error>,
                        > + Send;
                    }
                    pub trait HostTransaction: Send {
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<Transaction>,
                        ) -> impl ::core::future::Future<
                            Output = wasmtime::Result<()>,
                        > + Send;
                    }
                    impl<_T: HostTransaction + ?Sized + Send> HostTransaction
                    for &mut _T {
                        async fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<Transaction>,
                        ) -> wasmtime::Result<()> {
                            HostTransaction::drop(*self, rep).await
                        }
                    }
                    pub trait HostWithStore: wasmtime::component::HasData + HostTransactionWithStore + Send {}
                    impl<_T: ?Sized> HostWithStore for _T
                    where
                        _T: wasmtime::component::HasData + HostTransactionWithStore
                            + Send,
                    {}
                    pub trait Host: HostTransaction + Send {}
                    impl<_T: Host + ?Sized + Send> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static + Send,
                    {
                        let mut inst = linker.instance("wasmledger:sql/transaction")?;
                        inst.resource_async(
                            "transaction",
                            wasmtime::component::ResourceType::host::<Transaction>(),
                            move |mut store, rep| {
                                wasmtime::component::__internal::Box::new(async move {
                                    HostTransaction::drop(
                                            &mut host_getter(store.data_mut()),
                                            wasmtime::component::Resource::new_own(rep),
                                        )
                                        .await
                                })
                            },
                        )?;
                        inst.func_wrap_concurrent(
                            "[static]transaction.commit",
                            move |
                                caller: &wasmtime::component::Accessor<T>,
                                (arg0,): (wasmtime::component::Resource<Transaction>,)|
                            {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostTransactionWithStore>::commit(host, arg0)
                                        .await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        inst.func_wrap_concurrent(
                            "[static]transaction.rollback",
                            move |
                                caller: &wasmtime::component::Accessor<T>,
                                (arg0,): (wasmtime::component::Resource<Transaction>,)|
                            {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostTransactionWithStore>::rollback(
                                            host,
                                            arg0,
                                        )
                                        .await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        Ok(())
                    }
                }
                #[allow(clippy::all)]
                pub mod connection {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
                    const _: () = {
                        if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Transaction = super::super::super::wasmledger::sql::transaction::Transaction;
                    pub use super::super::super::__with_name0 as Connection;
                    pub trait HostConnectionWithStore: wasmtime::component::HasData + Send {
                        fn begin_transaction<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                            self_: wasmtime::component::Resource<Connection>,
                        ) -> impl ::core::future::Future<
                            Output = Result<
                                wasmtime::component::Resource<Transaction>,
                                Error,
                            >,
                        > + Send;
                    }
                    pub trait HostConnection: Send {
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<Connection>,
                        ) -> impl ::core::future::Future<
                            Output = wasmtime::Result<()>,
                        > + Send;
                    }
                    impl<_T: HostConnection + ?Sized + Send> HostConnection for &mut _T {
                        async fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<Connection>,
                        ) -> wasmtime::Result<()> {
                            HostConnection::drop(*self, rep).await
                        }
                    }
                    pub trait HostWithStore: wasmtime::component::HasData + HostConnectionWithStore + Send {}
                    impl<_T: ?Sized> HostWithStore for _T
                    where
                        _T: wasmtime::component::HasData + HostConnectionWithStore
                            + Send,
                    {}
                    pub trait Host: HostConnection + Send {}
                    impl<_T: Host + ?Sized + Send> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static + Send,
                    {
                        let mut inst = linker.instance("wasmledger:sql/connection")?;
                        inst.resource_async(
                            "connection",
                            wasmtime::component::ResourceType::host::<Connection>(),
                            move |mut store, rep| {
                                wasmtime::component::__internal::Box::new(async move {
                                    HostConnection::drop(
                                            &mut host_getter(store.data_mut()),
                                            wasmtime::component::Resource::new_own(rep),
                                        )
                                        .await
                                })
                            },
                        )?;
                        inst.func_wrap_concurrent(
                            "[method]connection.begin-transaction",
                            move |
                                caller: &wasmtime::component::Accessor<T>,
                                (arg0,): (wasmtime::component::Resource<Connection>,)|
                            {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostConnectionWithStore>::begin_transaction(
                                            host,
                                            arg0,
                                        )
                                        .await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        Ok(())
                    }
                }
                #[allow(clippy::all)]
                pub mod pool {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
                    const _: () = {
                        if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Transaction = super::super::super::wasmledger::sql::transaction::Transaction;
                    pub type Connection = super::super::super::wasmledger::sql::connection::Connection;
                    pub trait HostWithStore: wasmtime::component::HasData + Send {
                        fn begin_transaction<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                        ) -> impl ::core::future::Future<
                            Output = Result<
                                wasmtime::component::Resource<Transaction>,
                                Error,
                            >,
                        > + Send;
                        fn acquire_connection<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                        ) -> impl ::core::future::Future<
                            Output = Result<
                                wasmtime::component::Resource<Connection>,
                                Error,
                            >,
                        > + Send;
                    }
                    pub trait Host: Send {}
                    impl<_T: Host + ?Sized + Send> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static + Send,
                    {
                        let mut inst = linker.instance("wasmledger:sql/pool")?;
                        inst.func_wrap_concurrent(
                            "begin-transaction",
                            move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostWithStore>::begin_transaction(host).await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        inst.func_wrap_concurrent(
                            "acquire-connection",
                            move |caller: &wasmtime::component::Accessor<T>, (): ()| {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostWithStore>::acquire_connection(host)
                                        .await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        Ok(())
                    }
                }
                #[allow(clippy::all)]
                pub mod query_types {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type SqlString = wasmtime::component::__internal::String;
                    const _: () = {
                        if !(8
                            == <SqlString as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <SqlString as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <SqlString as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <SqlString as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub use super::super::super::__with_name2 as SqlArguments;
                    pub trait HostSqlArgumentsWithStore: wasmtime::component::HasData {}
                    impl<_T: ?Sized> HostSqlArgumentsWithStore for _T
                    where
                        _T: wasmtime::component::HasData,
                    {}
                    pub trait HostSqlArguments {
                        fn new(
                            &mut self,
                        ) -> wasmtime::Result<
                            wasmtime::component::Resource<SqlArguments>,
                        >;
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<SqlArguments>,
                        ) -> wasmtime::Result<()>;
                    }
                    impl<_T: HostSqlArguments + ?Sized> HostSqlArguments for &mut _T {
                        fn new(
                            &mut self,
                        ) -> wasmtime::Result<
                            wasmtime::component::Resource<SqlArguments>,
                        > {
                            HostSqlArguments::new(*self)
                        }
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<SqlArguments>,
                        ) -> wasmtime::Result<()> {
                            HostSqlArguments::drop(*self, rep)
                        }
                    }
                    #[component(record)]
                    pub struct SqlQuery {
                        #[component(name = "sql")]
                        pub sql: SqlString,
                        #[component(name = "args")]
                        pub args: wasmtime::component::Resource<SqlArguments>,
                        #[component(name = "persistent")]
                        pub persistent: Option<bool>,
                    }
                    unsafe impl wasmtime::component::Lower for SqlQuery {
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
                                wasmtime::component::__internal::InterfaceType::Record(
                                    i,
                                ) => &cx.types[i],
                                _ => wasmtime::component::__internal::bad_type_info(),
                            };
                            wasmtime::component::Lower::linear_lower_to_flat(
                                &self.sql,
                                cx,
                                ty.fields[0usize].ty,
                                {
                                    #[allow(unused_unsafe, reason = "macro-generated code")]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).sql)
                                        }
                                    }
                                },
                            )?;
                            wasmtime::component::Lower::linear_lower_to_flat(
                                &self.args,
                                cx,
                                ty.fields[1usize].ty,
                                {
                                    #[allow(unused_unsafe, reason = "macro-generated code")]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).args)
                                        }
                                    }
                                },
                            )?;
                            wasmtime::component::Lower::linear_lower_to_flat(
                                &self.persistent,
                                cx,
                                ty.fields[2usize].ty,
                                {
                                    #[allow(unused_unsafe, reason = "macro-generated code")]
                                    {
                                        unsafe {
                                            use ::wasmtime::MaybeUninitExt;
                                            let m: &mut core::mem::MaybeUninit<_> = dst;
                                            m.map(|p| &raw mut (*p).persistent)
                                        }
                                    }
                                },
                            )?;
                            Ok(())
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
                            let ty = match ty {
                                wasmtime::component::__internal::InterfaceType::Record(
                                    i,
                                ) => &cx.types[i],
                                _ => wasmtime::component::__internal::bad_type_info(),
                            };
                            wasmtime::component::Lower::linear_lower_to_memory(
                                &self.sql,
                                cx,
                                ty.fields[0usize].ty,
                                <SqlString as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(&mut offset),
                            )?;
                            wasmtime::component::Lower::linear_lower_to_memory(
                                &self.args,
                                cx,
                                ty.fields[1usize].ty,
                                <wasmtime::component::Resource<
                                    SqlArguments,
                                > as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(&mut offset),
                            )?;
                            wasmtime::component::Lower::linear_lower_to_memory(
                                &self.persistent,
                                cx,
                                ty.fields[2usize].ty,
                                <Option<bool> as wasmtime::component::ComponentType>::ABI
                                    .next_field32_size(&mut offset),
                            )?;
                            Ok(())
                        }
                    }
                    unsafe impl wasmtime::component::Lift for SqlQuery {
                        #[inline]
                        fn linear_lift_from_flat(
                            cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                            ty: wasmtime::component::__internal::InterfaceType,
                            src: &Self::Lower,
                        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                            let ty = match ty {
                                wasmtime::component::__internal::InterfaceType::Record(
                                    i,
                                ) => &cx.types[i],
                                _ => wasmtime::component::__internal::bad_type_info(),
                            };
                            Ok(Self {
                                sql: <SqlString as wasmtime::component::Lift>::linear_lift_from_flat(
                                    cx,
                                    ty.fields[0usize].ty,
                                    &src.sql,
                                )?,
                                args: <wasmtime::component::Resource<
                                    SqlArguments,
                                > as wasmtime::component::Lift>::linear_lift_from_flat(
                                    cx,
                                    ty.fields[1usize].ty,
                                    &src.args,
                                )?,
                                persistent: <Option<
                                    bool,
                                > as wasmtime::component::Lift>::linear_lift_from_flat(
                                    cx,
                                    ty.fields[2usize].ty,
                                    &src.persistent,
                                )?,
                            })
                        }
                        #[inline]
                        fn linear_lift_from_memory(
                            cx: &mut wasmtime::component::__internal::LiftContext<'_>,
                            ty: wasmtime::component::__internal::InterfaceType,
                            bytes: &[u8],
                        ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                            let ty = match ty {
                                wasmtime::component::__internal::InterfaceType::Record(
                                    i,
                                ) => &cx.types[i],
                                _ => wasmtime::component::__internal::bad_type_info(),
                            };
                            if true {
                                if !((bytes.as_ptr() as usize)
                                    % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                        as usize) == 0)
                                {
                                    ::core::panicking::panic(
                                        "assertion failed: (bytes.as_ptr() as usize) %\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                                    )
                                }
                            }
                            let mut offset = 0;
                            Ok(Self {
                                sql: <SqlString as wasmtime::component::Lift>::linear_lift_from_memory(
                                    cx,
                                    ty.fields[0usize].ty,
                                    &bytes[<SqlString as wasmtime::component::ComponentType>::ABI
                                        .next_field32_size(
                                            &mut offset,
                                        )..][..<SqlString as wasmtime::component::ComponentType>::SIZE32],
                                )?,
                                args: <wasmtime::component::Resource<
                                    SqlArguments,
                                > as wasmtime::component::Lift>::linear_lift_from_memory(
                                    cx,
                                    ty.fields[1usize].ty,
                                    &bytes[<wasmtime::component::Resource<
                                        SqlArguments,
                                    > as wasmtime::component::ComponentType>::ABI
                                        .next_field32_size(
                                            &mut offset,
                                        )..][..<wasmtime::component::Resource<
                                        SqlArguments,
                                    > as wasmtime::component::ComponentType>::SIZE32],
                                )?,
                                persistent: <Option<
                                    bool,
                                > as wasmtime::component::Lift>::linear_lift_from_memory(
                                    cx,
                                    ty.fields[2usize].ty,
                                    &bytes[<Option<
                                        bool,
                                    > as wasmtime::component::ComponentType>::ABI
                                        .next_field32_size(
                                            &mut offset,
                                        )..][..<Option<
                                        bool,
                                    > as wasmtime::component::ComponentType>::SIZE32],
                                )?,
                            })
                        }
                    }
                    const _: () = {
                        #[doc(hidden)]
                        #[repr(C)]
                        pub struct LowerSqlQuery<T0: Copy, T1: Copy, T2: Copy> {
                            sql: T0,
                            args: T1,
                            persistent: T2,
                            _align: [wasmtime::ValRaw; 0],
                        }
                        #[automatically_derived]
                        impl<
                            T0: ::core::clone::Clone + Copy,
                            T1: ::core::clone::Clone + Copy,
                            T2: ::core::clone::Clone + Copy,
                        > ::core::clone::Clone for LowerSqlQuery<T0, T1, T2> {
                            #[inline]
                            fn clone(&self) -> LowerSqlQuery<T0, T1, T2> {
                                LowerSqlQuery {
                                    sql: ::core::clone::Clone::clone(&self.sql),
                                    args: ::core::clone::Clone::clone(&self.args),
                                    persistent: ::core::clone::Clone::clone(&self.persistent),
                                    _align: ::core::clone::Clone::clone(&self._align),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<
                            T0: ::core::marker::Copy + Copy,
                            T1: ::core::marker::Copy + Copy,
                            T2: ::core::marker::Copy + Copy,
                        > ::core::marker::Copy for LowerSqlQuery<T0, T1, T2> {}
                        unsafe impl wasmtime::component::ComponentType for SqlQuery {
                            type Lower = LowerSqlQuery<
                                <SqlString as wasmtime::component::ComponentType>::Lower,
                                <wasmtime::component::Resource<
                                    SqlArguments,
                                > as wasmtime::component::ComponentType>::Lower,
                                <Option<bool> as wasmtime::component::ComponentType>::Lower,
                            >;
                            const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                                &[
                                    <SqlString as wasmtime::component::ComponentType>::ABI,
                                    <wasmtime::component::Resource<
                                        SqlArguments,
                                    > as wasmtime::component::ComponentType>::ABI,
                                    <Option<bool> as wasmtime::component::ComponentType>::ABI,
                                ],
                            );
                            #[inline]
                            fn typecheck(
                                ty: &wasmtime::component::__internal::InterfaceType,
                                types: &wasmtime::component::__internal::InstanceType<'_>,
                            ) -> wasmtime::component::__internal::anyhow::Result<()> {
                                wasmtime::component::__internal::typecheck_record(
                                    ty,
                                    types,
                                    &[
                                        (
                                            "sql",
                                            <SqlString as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                        (
                                            "args",
                                            <wasmtime::component::Resource<
                                                SqlArguments,
                                            > as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                        (
                                            "persistent",
                                            <Option<
                                                bool,
                                            > as wasmtime::component::ComponentType>::typecheck,
                                        ),
                                    ],
                                )
                            }
                        }
                    };
                    impl core::fmt::Debug for SqlQuery {
                        fn fmt(
                            &self,
                            f: &mut core::fmt::Formatter<'_>,
                        ) -> core::fmt::Result {
                            f.debug_struct("SqlQuery")
                                .field("sql", &self.sql)
                                .field("args", &self.args)
                                .field("persistent", &self.persistent)
                                .finish()
                        }
                    }
                    const _: () = {
                        if !(16
                            == <SqlQuery as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 16 == <SqlQuery as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <SqlQuery as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <SqlQuery as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub use super::super::super::__with_name1 as QueryResults;
                    pub trait HostQueryResultsWithStore: wasmtime::component::HasData {}
                    impl<_T: ?Sized> HostQueryResultsWithStore for _T
                    where
                        _T: wasmtime::component::HasData,
                    {}
                    pub trait HostQueryResults {
                        fn row_count(
                            &mut self,
                            self_: wasmtime::component::Resource<QueryResults>,
                        ) -> wasmtime::Result<u64>;
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<QueryResults>,
                        ) -> wasmtime::Result<()>;
                    }
                    impl<_T: HostQueryResults + ?Sized> HostQueryResults for &mut _T {
                        fn row_count(
                            &mut self,
                            self_: wasmtime::component::Resource<QueryResults>,
                        ) -> wasmtime::Result<u64> {
                            HostQueryResults::row_count(*self, self_)
                        }
                        fn drop(
                            &mut self,
                            rep: wasmtime::component::Resource<QueryResults>,
                        ) -> wasmtime::Result<()> {
                            HostQueryResults::drop(*self, rep)
                        }
                    }
                    pub trait HostWithStore: wasmtime::component::HasData + HostSqlArgumentsWithStore + HostQueryResultsWithStore {}
                    impl<_T: ?Sized> HostWithStore for _T
                    where
                        _T: wasmtime::component::HasData + HostSqlArgumentsWithStore
                            + HostQueryResultsWithStore,
                    {}
                    pub trait Host: HostSqlArguments + HostQueryResults {}
                    impl<_T: Host + ?Sized> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static,
                    {
                        let mut inst = linker.instance("wasmledger:sql/query-types")?;
                        inst.resource(
                            "sql-arguments",
                            wasmtime::component::ResourceType::host::<SqlArguments>(),
                            move |mut store, rep| -> wasmtime::Result<()> {
                                let resource = wasmtime::component::Resource::new_own(rep);
                                HostSqlArguments::drop(
                                    &mut host_getter(store.data_mut()),
                                    resource,
                                )
                            },
                        )?;
                        inst.resource(
                            "query-results",
                            wasmtime::component::ResourceType::host::<QueryResults>(),
                            move |mut store, rep| -> wasmtime::Result<()> {
                                let resource = wasmtime::component::Resource::new_own(rep);
                                HostQueryResults::drop(
                                    &mut host_getter(store.data_mut()),
                                    resource,
                                )
                            },
                        )?;
                        inst.func_wrap(
                            "[constructor]sql-arguments",
                            move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                                let host = &mut host_getter(caller.data_mut());
                                let r = HostSqlArguments::new(host);
                                Ok((r?,))
                            },
                        )?;
                        inst.func_wrap(
                            "[method]query-results.row-count",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (arg0,): (wasmtime::component::Resource<QueryResults>,)|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = HostQueryResults::row_count(host, arg0);
                                Ok((r?,))
                            },
                        )?;
                        Ok(())
                    }
                }
                #[allow(clippy::all)]
                pub mod query {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type SqlQuery = super::super::super::wasmledger::sql::query_types::SqlQuery;
                    const _: () = {
                        if !(16
                            == <SqlQuery as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 16 == <SqlQuery as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <SqlQuery as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <SqlQuery as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type SqlString = super::super::super::wasmledger::sql::query_types::SqlString;
                    const _: () = {
                        if !(8
                            == <SqlString as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <SqlString as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <SqlString as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <SqlString as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type QueryResults = super::super::super::wasmledger::sql::query_types::QueryResults;
                    pub type Error = super::super::super::wasmledger::sql::util_types::Error;
                    const _: () = {
                        if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Connection = super::super::super::wasmledger::sql::connection::Connection;
                    pub type Transaction = super::super::super::wasmledger::sql::transaction::Transaction;
                    #[component(variant)]
                    pub enum QueryExecutor {
                        #[component(name = "pool")]
                        Pool,
                        #[component(name = "connection")]
                        Connection(wasmtime::component::Resource<Connection>),
                        #[component(name = "transaction")]
                        Transaction(wasmtime::component::Resource<Transaction>),
                    }
                    unsafe impl wasmtime::component::Lower for QueryExecutor {
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
                                Self::Pool => {
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
                                                        m.map(|p| &raw mut (*p).Pool)
                                                    }
                                                }
                                            },
                                            |dst| Ok(()),
                                        )
                                    }
                                }
                                Self::Connection(value) => {
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
                                                        m.map(|p| &raw mut (*p).Connection)
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
                                Self::Transaction(value) => {
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
                                        .write(wasmtime::ValRaw::u32(2u32));
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
                                                        m.map(|p| &raw mut (*p).Transaction)
                                                    }
                                                }
                                            },
                                            |dst| {
                                                value
                                                    .linear_lower_to_flat(
                                                        cx,
                                                        ty
                                                            .cases[2usize]
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
                                Self::Pool => {
                                    *cx.get::<1usize>(offset) = 0u8.to_le_bytes();
                                    Ok(())
                                }
                                Self::Connection(value) => {
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
                                Self::Transaction(value) => {
                                    *cx.get::<1usize>(offset) = 2u8.to_le_bytes();
                                    value
                                        .linear_lower_to_memory(
                                            cx,
                                            ty
                                                .cases[2usize]
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
                    unsafe impl wasmtime::component::Lift for QueryExecutor {
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
                                    0u32 => Self::Pool,
                                    1u32 => {
                                        Self::Connection(
                                            <wasmtime::component::Resource<
                                                Connection,
                                            > as wasmtime::component::Lift>::linear_lift_from_flat(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                unsafe { &src.payload.Connection },
                                            )?,
                                        )
                                    }
                                    2u32 => {
                                        Self::Transaction(
                                            <wasmtime::component::Resource<
                                                Transaction,
                                            > as wasmtime::component::Lift>::linear_lift_from_flat(
                                                cx,
                                                ty
                                                    .cases[2usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                unsafe { &src.payload.Transaction },
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
                                    0u8 => Self::Pool,
                                    1u8 => {
                                        Self::Connection(
                                            <wasmtime::component::Resource<
                                                Connection,
                                            > as wasmtime::component::Lift>::linear_lift_from_memory(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                &payload[..<wasmtime::component::Resource<
                                                    Connection,
                                                > as wasmtime::component::ComponentType>::SIZE32],
                                            )?,
                                        )
                                    }
                                    2u8 => {
                                        Self::Transaction(
                                            <wasmtime::component::Resource<
                                                Transaction,
                                            > as wasmtime::component::Lift>::linear_lift_from_memory(
                                                cx,
                                                ty
                                                    .cases[2usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                &payload[..<wasmtime::component::Resource<
                                                    Transaction,
                                                > as wasmtime::component::ComponentType>::SIZE32],
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
                        pub struct LowerQueryExecutor<T1: Copy, T2: Copy> {
                            tag: wasmtime::ValRaw,
                            payload: LowerPayloadQueryExecutor<T1, T2>,
                        }
                        #[automatically_derived]
                        impl<
                            T1: ::core::clone::Clone + Copy,
                            T2: ::core::clone::Clone + Copy,
                        > ::core::clone::Clone for LowerQueryExecutor<T1, T2> {
                            #[inline]
                            fn clone(&self) -> LowerQueryExecutor<T1, T2> {
                                LowerQueryExecutor {
                                    tag: ::core::clone::Clone::clone(&self.tag),
                                    payload: ::core::clone::Clone::clone(&self.payload),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<
                            T1: ::core::marker::Copy + Copy,
                            T2: ::core::marker::Copy + Copy,
                        > ::core::marker::Copy for LowerQueryExecutor<T1, T2> {}
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        #[repr(C)]
                        union LowerPayloadQueryExecutor<T1: Copy, T2: Copy> {
                            Pool: [wasmtime::ValRaw; 0],
                            Connection: T1,
                            Transaction: T2,
                        }
                        #[automatically_derived]
                        #[allow(non_snake_case)]
                        impl<
                            T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                            T2: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        > ::core::clone::Clone for LowerPayloadQueryExecutor<T1, T2> {
                            #[inline]
                            fn clone(&self) -> LowerPayloadQueryExecutor<T1, T2> {
                                let _: ::core::clone::AssertParamIsCopy<Self>;
                                *self
                            }
                        }
                        #[automatically_derived]
                        #[allow(non_snake_case)]
                        impl<
                            T1: ::core::marker::Copy + Copy,
                            T2: ::core::marker::Copy + Copy,
                        > ::core::marker::Copy for LowerPayloadQueryExecutor<T1, T2> {}
                        unsafe impl wasmtime::component::ComponentType
                        for QueryExecutor {
                            type Lower = LowerQueryExecutor<
                                <wasmtime::component::Resource<
                                    Connection,
                                > as wasmtime::component::ComponentType>::Lower,
                                <wasmtime::component::Resource<
                                    Transaction,
                                > as wasmtime::component::ComponentType>::Lower,
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
                                        ("pool", None),
                                        (
                                            "connection",
                                            Some(
                                                <wasmtime::component::Resource<
                                                    Connection,
                                                > as wasmtime::component::ComponentType>::typecheck,
                                            ),
                                        ),
                                        (
                                            "transaction",
                                            Some(
                                                <wasmtime::component::Resource<
                                                    Transaction,
                                                > as wasmtime::component::ComponentType>::typecheck,
                                            ),
                                        ),
                                    ],
                                )
                            }
                            const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                                &[
                                    None,
                                    Some(
                                        <wasmtime::component::Resource<
                                            Connection,
                                        > as wasmtime::component::ComponentType>::ABI,
                                    ),
                                    Some(
                                        <wasmtime::component::Resource<
                                            Transaction,
                                        > as wasmtime::component::ComponentType>::ABI,
                                    ),
                                ],
                            );
                        }
                        unsafe impl wasmtime::component::__internal::ComponentVariant
                        for QueryExecutor {
                            const CASES: &'static [Option<
                                wasmtime::component::__internal::CanonicalAbiInfo,
                            >] = &[
                                None,
                                Some(
                                    <wasmtime::component::Resource<
                                        Connection,
                                    > as wasmtime::component::ComponentType>::ABI,
                                ),
                                Some(
                                    <wasmtime::component::Resource<
                                        Transaction,
                                    > as wasmtime::component::ComponentType>::ABI,
                                ),
                            ];
                        }
                    };
                    impl core::fmt::Debug for QueryExecutor {
                        fn fmt(
                            &self,
                            f: &mut core::fmt::Formatter<'_>,
                        ) -> core::fmt::Result {
                            match self {
                                QueryExecutor::Pool => {
                                    f.debug_tuple("QueryExecutor::Pool").finish()
                                }
                                QueryExecutor::Connection(e) => {
                                    f.debug_tuple("QueryExecutor::Connection").field(e).finish()
                                }
                                QueryExecutor::Transaction(e) => {
                                    f.debug_tuple("QueryExecutor::Transaction")
                                        .field(e)
                                        .finish()
                                }
                            }
                        }
                    }
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
                    pub trait HostWithStore: wasmtime::component::HasData + Send {
                        fn fetch_all<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                            query: SqlQuery,
                            executor: QueryExecutor,
                        ) -> impl ::core::future::Future<
                            Output = Result<
                                wasmtime::component::Resource<QueryResults>,
                                Error,
                            >,
                        > + Send;
                        fn execute<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                            query: SqlQuery,
                            executor: QueryExecutor,
                        ) -> impl ::core::future::Future<
                            Output = Result<u64, Error>,
                        > + Send;
                        fn execute_raw<T>(
                            accessor: &wasmtime::component::Accessor<T, Self>,
                            query: SqlString,
                            executor: QueryExecutor,
                        ) -> impl ::core::future::Future<
                            Output = Result<(), Error>,
                        > + Send;
                    }
                    pub trait Host: Send {}
                    impl<_T: Host + ?Sized + Send> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static + Send,
                    {
                        let mut inst = linker.instance("wasmledger:sql/query")?;
                        inst.func_wrap_concurrent(
                            "fetch-all",
                            move |
                                caller: &wasmtime::component::Accessor<T>,
                                (arg0, arg1): (SqlQuery, QueryExecutor)|
                            {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostWithStore>::fetch_all(host, arg0, arg1)
                                        .await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        inst.func_wrap_concurrent(
                            "execute",
                            move |
                                caller: &wasmtime::component::Accessor<T>,
                                (arg0, arg1): (SqlQuery, QueryExecutor)|
                            {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostWithStore>::execute(host, arg0, arg1)
                                        .await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        inst.func_wrap_concurrent(
                            "execute-raw",
                            move |
                                caller: &wasmtime::component::Accessor<T>,
                                (arg0, arg1): (SqlString, QueryExecutor)|
                            {
                                wasmtime::component::__internal::Box::pin(async move {
                                    let host = &caller.with_getter(host_getter);
                                    let r = <D as HostWithStore>::execute_raw(host, arg0, arg1)
                                        .await;
                                    Ok((r,))
                                })
                            },
                        )?;
                        Ok(())
                    }
                }
            }
        }
        const _: &str = "package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n  use connection.{connection};\n\n  begin-transaction: async func() -> result<transaction, error>;\n  acquire-connection: async func() -> result<connection, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}\n\ninterface connection {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  resource connection {\n    begin-transaction: async func() -> result<transaction, error>;\n  }\n}";
        const _: &str = "package wasmledger:sql;\n\ninterface codecs {\n  use util-types.{error};\n\n  type push-result = result<_, error>;\n\n  type row-index = u64;\n\n  variant column-index {\n    index(u64),\n    column-name(string)\n  }\n\n  type value-position = tuple<row-index, column-index>;\n}";
        const _: &str = "package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use connection.{connection};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    connection(borrow<connection>),\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
        const _: &str = "package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// attempted to perform operation on already closed transaction\n    transaction-closed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results {\n    row-count: func() -> u64;\n  }\n}";
        const _: &str = "package wasmledger:sql-host;\n\nworld host {\n  import wasmledger:sql/pool;\n  import wasmledger:sql/transaction;\n  import wasmledger:sql/query;\n  import wasmledger:sql/query-types;\n}";
        #[allow(unused)]
        pub struct BindingsImplState {
            pub(crate) table: ResourceTable,
        }
        #[automatically_derived]
        #[allow(unused)]
        impl ::core::default::Default for BindingsImplState {
            #[inline]
            fn default() -> BindingsImplState {
                BindingsImplState {
                    table: ::core::default::Default::default(),
                }
            }
        }
        impl HasData for BindingsImplState {
            type Data<'a> = &'a mut BindingsImplState where Self: 'a;
        }
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
            use tokio::sync::{mpsc::error::SendError, oneshot::error::RecvError};
            use wasmtime::component::ResourceTableError;
            use crate::core::bindings::wasmledger::sql::util_types::Error;
            impl From<sqlx::Error> for Error {
                fn from(err: sqlx::Error) -> Self {
                    match err {
                        sqlx::Error::PoolTimedOut => Error::PoolTimedOut,
                        sqlx::Error::PoolClosed => Error::PoolClosed,
                        sqlx::Error::Decode(_) => Error::Decode(err.to_string()),
                        sqlx::Error::Encode(_) => Error::Encode(err.to_string()),
                        sqlx::Error::BeginFailed => Error::BeginFailed,
                        err => Error::Unexpected(err.to_string()),
                    }
                }
            }
            impl From<ResourceTableError> for Error {
                fn from(value: ResourceTableError) -> Self {
                    Self::Unexpected(value.to_string())
                }
            }
            impl From<RecvError> for Error {
                fn from(value: RecvError) -> Self {
                    Self::Unexpected(value.to_string())
                }
            }
            impl<T> From<SendError<T>> for Error {
                fn from(value: SendError<T>) -> Self {
                    Self::Unexpected(value.to_string())
                }
            }
        }
        pub(crate) mod pool {
            use std::sync::Arc;
            use tokio::sync::RwLock;
            use crate::core::bindings::{
                BindingsImplState, connection::ConnectionImpl, context::BindingsContext,
                executor::ErasedExecutor, transaction::TransactionImpl,
                wasmledger::sql::{
                    pool::Connection, transaction::Transaction, util_types::Error,
                },
            };
            impl crate::core::bindings::wasmledger::sql::pool::HostWithStore
            for BindingsImplState {
                async fn begin_transaction<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> Result<wasmtime::component::Resource<Transaction>, Error> {
                    let pool = BindingsContext::get_pool();
                    let tx = pool.begin().await?;
                    let tx_impl = TransactionImpl::Tx(Arc::new(RwLock::new(Some(tx))));
                    let tx_resource = accessor
                        .with(|mut access| {
                            let state = access.get();
                            state.table.push(tx_impl)
                        })?;
                    Ok(tx_resource)
                }
                async fn acquire_connection<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                ) -> Result<wasmtime::component::Resource<Connection>, Error> {
                    let pool = BindingsContext::get_pool();
                    let conn = pool.acquire().await?;
                    let conn_impl = ConnectionImpl {
                        connection: Arc::new(RwLock::new(conn)),
                    };
                    let resource = accessor
                        .with(|mut access| {
                            let state = access.get();
                            state.table.push(conn_impl)
                        })?;
                    Ok(resource)
                }
            }
        }
        pub(crate) mod query {
            use sqlx::any::AnyQueryResult;
            use crate::core::bindings::BindingsImplState;
            use crate::core::bindings::executor::{ErasedExecutor, QueryOrRaw};
            use crate::core::bindings::query_results::QueryResultsImpl;
            use crate::core::bindings::wasmledger::sql::query::QueryExecutor;
            use crate::core::bindings::wasmledger::sql::query_types::{
                QueryResults, SqlQuery, SqlString,
            };
            use crate::core::bindings::wasmledger::sql::util_types::Error;
            impl crate::core::bindings::wasmledger::sql::query::HostWithStore
            for BindingsImplState {
                async fn fetch_all<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    query: SqlQuery,
                    executor: QueryExecutor,
                ) -> Result<wasmtime::component::Resource<QueryResults>, Error> {
                    let results = executor
                        .fetch_all(QueryOrRaw::Query(query), accessor)
                        .await?;
                    let query_results = accessor
                        .with(|mut access| {
                            access.get().table.push(QueryResultsImpl { results })
                        })?;
                    Ok(query_results)
                }
                async fn execute<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    query: SqlQuery,
                    executor: QueryExecutor,
                ) -> Result<u64, Error> {
                    let results: AnyQueryResult = executor
                        .execute(QueryOrRaw::Query(query), accessor)
                        .await?
                        .into();
                    Ok(results.rows_affected())
                }
                async fn execute_raw<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    query: SqlString,
                    executor: QueryExecutor,
                ) -> Result<(), Error> {
                    executor.execute(QueryOrRaw::Raw(query), accessor).await?;
                    Ok(())
                }
            }
        }
        pub(crate) mod query_results {
            use crate::{
                core::bindings::{
                    BindingsImplState, wasmledger::sql::query_types::QueryResults,
                },
                sqldb::SqlDatabase,
            };
            #[allow(unused)]
            pub struct QueryResultsImpl {
                pub(crate) results: Vec<<SqlDatabase as sqlx::Database>::Row>,
            }
            impl crate::core::bindings::wasmledger::sql::query_types::HostQueryResults
            for BindingsImplState {
                fn row_count(
                    &mut self,
                    self_: wasmtime::component::Resource<QueryResults>,
                ) -> Result<u64, wasmtime::Error> {
                    let a = self.table.get(&self_)?;
                    Ok(a.results.len() as u64)
                }
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<QueryResults>,
                ) -> wasmtime::Result<()> {
                    self.table.delete(rep)?;
                    Ok(())
                }
            }
        }
        pub(crate) mod sql_arguments {
            use crate::core::bindings::wasmledger::sql::query_types::SqlArguments;
            use crate::{core::bindings::BindingsImplState, sqldb::SqlDatabase};
            use tokio::sync::RwLock;
            #[allow(unused)]
            pub struct SqlArgumentsImpl {
                pub(crate) args: RwLock<
                    <SqlDatabase as sqlx::Database>::Arguments<'static>,
                >,
            }
            impl crate::core::bindings::wasmledger::sql::query_types::HostSqlArguments
            for BindingsImplState {
                fn new(
                    &mut self,
                ) -> Result<
                    wasmtime::component::Resource<SqlArgumentsImpl>,
                    wasmtime::Error,
                > {
                    let args = self
                        .table
                        .push(SqlArgumentsImpl {
                            args: RwLock::new(
                                <SqlDatabase as sqlx::Database>::Arguments::default(),
                            ),
                        })?;
                    Ok(args)
                }
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<SqlArguments>,
                ) -> wasmtime::Result<()> {
                    self.table.delete(rep)?;
                    Ok(())
                }
            }
        }
        pub(crate) mod connection {
            use std::sync::Arc;
            use sqlx::pool::PoolConnection;
            use tokio::sync::RwLock;
            use crate::{
                core::bindings::{
                    BindingsImplState, executor::{ErasedExecutor, QueryOrRaw},
                    transaction::{ConnectionBoundTask, TransactionCommand},
                    wasmledger::sql::{
                        connection::Connection, transaction::Transaction,
                        util_types::Error,
                    },
                },
                execute_with, sqldb::SqlDatabase,
            };
            use crate::core::bindings::transaction::TransactionImpl;
            pub struct ConnectionImpl {
                pub(crate) connection: Arc<RwLock<PoolConnection<SqlDatabase>>>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ConnectionImpl {
                #[inline]
                fn clone(&self) -> ConnectionImpl {
                    ConnectionImpl {
                        connection: ::core::clone::Clone::clone(&self.connection),
                    }
                }
            }
            impl crate::core::bindings::wasmledger::sql::connection::HostConnectionWithStore
            for BindingsImplState {
                async fn begin_transaction<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    self_: wasmtime::component::Resource<Connection>,
                ) -> Result<wasmtime::component::Resource<Transaction>, Error> {
                    let (sender, receiver) = tokio::sync::mpsc::channel::<
                        TransactionCommand,
                    >(1);
                    let handle = accessor
                        .spawn(ConnectionBoundTask {
                            resource: self_,
                            receiver,
                        });
                    let tx_impl = TransactionImpl::ConnectionBound {
                        handle: Arc::new(handle),
                        sender,
                    };
                    let resource = accessor
                        .with(|mut access| {
                            let state = access.get();
                            state.table.push(tx_impl)
                        })?;
                    return Ok(resource);
                }
            }
            impl ErasedExecutor<BindingsImplState> for ConnectionImpl {
                async fn fetch_all<T>(
                    &self,
                    query: QueryOrRaw,
                    accessor: &wasmtime::component::Accessor<T, BindingsImplState>,
                ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error> {
                    let mut guard = self.connection.write().await;
                    {
                        match query {
                            QueryOrRaw::Query(query) => {
                                let query = {
                                    let args_resource = query.args;
                                    let args = accessor
                                        .with(|mut access| {
                                            let st = access.get();
                                            st.table.delete(args_resource).map(|x| x.args.into_inner())
                                        })?;
                                    sqlx::query_with(&query.sql, args)
                                        .persistent(query.persistent.unwrap_or(true))
                                };
                                let result = query.fetch_all(&mut **guard).await?;
                                Result::<
                                    _,
                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                >::Ok(result)
                            }
                            QueryOrRaw::Raw(sql) => {
                                let query = sqlx::raw_sql(&sql);
                                let result = query.fetch_all(&mut **guard).await?;
                                Result::<
                                    _,
                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                >::Ok(result)
                            }
                        }
                    }
                }
                async fn execute<T>(
                    &self,
                    query: QueryOrRaw,
                    accessor: &wasmtime::component::Accessor<T, BindingsImplState>,
                ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error> {
                    let mut guard = self.connection.write().await;
                    {
                        match query {
                            QueryOrRaw::Query(query) => {
                                let query = {
                                    let args_resource = query.args;
                                    let args = accessor
                                        .with(|mut access| {
                                            let st = access.get();
                                            st.table.delete(args_resource).map(|x| x.args.into_inner())
                                        })?;
                                    sqlx::query_with(&query.sql, args)
                                        .persistent(query.persistent.unwrap_or(true))
                                };
                                let result = query.execute(&mut **guard).await?;
                                Result::<
                                    _,
                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                >::Ok(result)
                            }
                            QueryOrRaw::Raw(sql) => {
                                let query = sqlx::raw_sql(&sql);
                                let result = query.execute(&mut **guard).await?;
                                Result::<
                                    _,
                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                >::Ok(result)
                            }
                        }
                    }
                }
            }
        }
        pub(crate) mod transaction {
            use sqlx::Acquire;
            use std::sync::Arc;
            use sqlx::Executor;
            use tokio::sync::{
                RwLock, mpsc::{Receiver, Sender},
                oneshot,
            };
            use wasmtime::component::{Accessor, AccessorTask, HasData, JoinHandle};
            use crate::{
                core::bindings::{
                    BindingsImplState, executor::{ErasedExecutor, QueryOrRaw},
                    wasmledger::sql::{
                        connection::Connection, transaction::Transaction,
                        util_types::Error,
                    },
                },
                execute_with, sqldb::SqlDatabase,
            };
            pub enum TransactionCommand {
                FetchAll {
                    query: QueryOrRaw,
                    cb: oneshot::Sender<
                        Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error>,
                    >,
                },
                Execute {
                    query: QueryOrRaw,
                    cb: oneshot::Sender<
                        Result<<SqlDatabase as sqlx::Database>::QueryResult, Error>,
                    >,
                },
                Commit { cb: oneshot::Sender<Result<(), Error>> },
                Rollback { cb: oneshot::Sender<Result<(), Error>> },
            }
            pub struct ConnectionBoundTask {
                pub(crate) resource: wasmtime::component::Resource<Connection>,
                pub(crate) receiver: Receiver<TransactionCommand>,
            }
            pub enum TransactionImpl {
                Tx(Arc<RwLock<Option<sqlx::Transaction<'static, SqlDatabase>>>>),
                ConnectionBound {
                    handle: Arc<JoinHandle>,
                    sender: Sender<TransactionCommand>,
                },
            }
            #[automatically_derived]
            impl ::core::clone::Clone for TransactionImpl {
                #[inline]
                fn clone(&self) -> TransactionImpl {
                    match self {
                        TransactionImpl::Tx(__self_0) => {
                            TransactionImpl::Tx(::core::clone::Clone::clone(__self_0))
                        }
                        TransactionImpl::ConnectionBound {
                            handle: __self_0,
                            sender: __self_1,
                        } => {
                            TransactionImpl::ConnectionBound {
                                handle: ::core::clone::Clone::clone(__self_0),
                                sender: ::core::clone::Clone::clone(__self_1),
                            }
                        }
                    }
                }
            }
            trait TransactionErasedOps {
                async fn commit(self) -> Result<(), Error>;
                async fn rollback(self) -> Result<(), Error>;
            }
            impl ErasedExecutor<BindingsImplState> for TransactionImpl {
                async fn fetch_all<T>(
                    &self,
                    query: QueryOrRaw,
                    accessor: &Accessor<T, BindingsImplState>,
                ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error> {
                    match self {
                        TransactionImpl::Tx(rw_lock) => {
                            let mut guard = rw_lock.write().await;
                            if let Some(tx) = guard.as_mut() {
                                {
                                    match query {
                                        QueryOrRaw::Query(query) => {
                                            let query = {
                                                let args_resource = query.args;
                                                let args = accessor
                                                    .with(|mut access| {
                                                        let st = access.get();
                                                        st.table.delete(args_resource).map(|x| x.args.into_inner())
                                                    })?;
                                                sqlx::query_with(&query.sql, args)
                                                    .persistent(query.persistent.unwrap_or(true))
                                            };
                                            let result = query.fetch_all(&mut **tx).await?;
                                            Result::<
                                                _,
                                                crate::core::bindings::wasmledger::sql::util_types::Error,
                                            >::Ok(result)
                                        }
                                        QueryOrRaw::Raw(sql) => {
                                            let query = sqlx::raw_sql(&sql);
                                            let result = query.fetch_all(&mut **tx).await?;
                                            Result::<
                                                _,
                                                crate::core::bindings::wasmledger::sql::util_types::Error,
                                            >::Ok(result)
                                        }
                                    }
                                }
                            } else {
                                Err(Error::TransactionClosed)
                            }
                        }
                        TransactionImpl::ConnectionBound { handle: _, sender } => {
                            let (s, r) = oneshot::channel();
                            sender
                                .send(TransactionCommand::FetchAll {
                                    query,
                                    cb: s,
                                })
                                .await?;
                            r.await?
                        }
                    }
                }
                async fn execute<T>(
                    &self,
                    query: super::executor::QueryOrRaw,
                    accessor: &Accessor<T, BindingsImplState>,
                ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error> {
                    match self {
                        TransactionImpl::Tx(rw_lock) => {
                            let mut guard = rw_lock.write().await;
                            if let Some(tx) = guard.as_mut() {
                                {
                                    match query {
                                        QueryOrRaw::Query(query) => {
                                            let query = {
                                                let args_resource = query.args;
                                                let args = accessor
                                                    .with(|mut access| {
                                                        let st = access.get();
                                                        st.table.delete(args_resource).map(|x| x.args.into_inner())
                                                    })?;
                                                sqlx::query_with(&query.sql, args)
                                                    .persistent(query.persistent.unwrap_or(true))
                                            };
                                            let result = query.execute(&mut **tx).await?;
                                            Result::<
                                                _,
                                                crate::core::bindings::wasmledger::sql::util_types::Error,
                                            >::Ok(result)
                                        }
                                        QueryOrRaw::Raw(sql) => {
                                            let query = sqlx::raw_sql(&sql);
                                            let result = query.execute(&mut **tx).await?;
                                            Result::<
                                                _,
                                                crate::core::bindings::wasmledger::sql::util_types::Error,
                                            >::Ok(result)
                                        }
                                    }
                                }
                            } else {
                                Err(Error::TransactionClosed)
                            }
                        }
                        TransactionImpl::ConnectionBound { handle: _, sender } => {
                            let (s, r) = oneshot::channel();
                            sender
                                .send(TransactionCommand::Execute {
                                    query,
                                    cb: s,
                                })
                                .await?;
                            r.await?
                        }
                    }
                }
            }
            impl TransactionErasedOps for TransactionImpl {
                async fn commit(self) -> Result<(), Error> {
                    match self {
                        TransactionImpl::Tx(rw_lock) => {
                            let mut guard = rw_lock.write().await;
                            if let Some(tx) = guard.take() {
                                Ok(tx.commit().await?)
                            } else {
                                Err(Error::TransactionClosed)
                            }
                        }
                        TransactionImpl::ConnectionBound { handle: _, sender } => {
                            let (s, r) = oneshot::channel();
                            sender
                                .send(TransactionCommand::Commit {
                                    cb: s,
                                })
                                .await?;
                            r.await?
                        }
                    }
                }
                async fn rollback(self) -> Result<(), Error> {
                    match self {
                        TransactionImpl::Tx(rw_lock) => {
                            let mut guard = rw_lock.write().await;
                            if let Some(tx) = guard.take() {
                                Ok(tx.rollback().await?)
                            } else {
                                Err(Error::TransactionClosed)
                            }
                        }
                        TransactionImpl::ConnectionBound { handle: _, sender } => {
                            let (s, r) = oneshot::channel();
                            sender
                                .send(TransactionCommand::Rollback {
                                    cb: s,
                                })
                                .await?;
                            r.await?
                        }
                    }
                }
            }
            impl<T> AccessorTask<T, BindingsImplState, Result<(), wasmtime::Error>>
            for ConnectionBoundTask {
                async fn run(
                    mut self,
                    accessor: &wasmtime::component::Accessor<T, BindingsImplState>,
                ) -> Result<(), wasmtime::Error> {
                    let conn = accessor
                        .with(|mut access| {
                            let state = access.get();
                            state.table.get(&self.resource).map(|x| x.connection.clone())
                        })?;
                    let mut guard = conn.write().await;
                    let mut tx = Some(guard.begin().await?);
                    while let Some(cmd) = self.receiver.recv().await {
                        match cmd {
                            TransactionCommand::FetchAll { query, cb } => {
                                if let Some(ref mut tx) = tx {
                                    let res = {
                                        match query {
                                            QueryOrRaw::Query(query) => {
                                                let query = {
                                                    let args_resource = query.args;
                                                    let args = accessor
                                                        .with(|mut access| {
                                                            let st = access.get();
                                                            st.table.delete(args_resource).map(|x| x.args.into_inner())
                                                        })?;
                                                    sqlx::query_with(&query.sql, args)
                                                        .persistent(query.persistent.unwrap_or(true))
                                                };
                                                let result = query.fetch_all(&mut **tx).await?;
                                                Result::<
                                                    _,
                                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                                >::Ok(result)
                                            }
                                            QueryOrRaw::Raw(sql) => {
                                                let query = sqlx::raw_sql(&sql);
                                                let result = query.fetch_all(&mut **tx).await?;
                                                Result::<
                                                    _,
                                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                                >::Ok(result)
                                            }
                                        }
                                    };
                                    cb.send(res);
                                } else {
                                    cb.send(Err(Error::TransactionClosed));
                                }
                            }
                            TransactionCommand::Execute { query, cb } => {
                                if let Some(ref mut tx) = tx {
                                    let res = {
                                        match query {
                                            QueryOrRaw::Query(query) => {
                                                let query = {
                                                    let args_resource = query.args;
                                                    let args = accessor
                                                        .with(|mut access| {
                                                            let st = access.get();
                                                            st.table.delete(args_resource).map(|x| x.args.into_inner())
                                                        })?;
                                                    sqlx::query_with(&query.sql, args)
                                                        .persistent(query.persistent.unwrap_or(true))
                                                };
                                                let result = query.execute(&mut **tx).await?;
                                                Result::<
                                                    _,
                                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                                >::Ok(result)
                                            }
                                            QueryOrRaw::Raw(sql) => {
                                                let query = sqlx::raw_sql(&sql);
                                                let result = query.execute(&mut **tx).await?;
                                                Result::<
                                                    _,
                                                    crate::core::bindings::wasmledger::sql::util_types::Error,
                                                >::Ok(result)
                                            }
                                        }
                                    };
                                    cb.send(res);
                                } else {
                                    cb.send(Err(Error::TransactionClosed));
                                }
                            }
                            TransactionCommand::Commit { cb } => {
                                if let Some(tx) = tx.take() {
                                    cb.send(tx.commit().await.map_err(|e| e.into()));
                                } else {
                                    cb.send(Err(Error::TransactionClosed));
                                }
                            }
                            TransactionCommand::Rollback { cb } => {
                                if let Some(tx) = tx.take() {
                                    cb.send(tx.rollback().await.map_err(|e| e.into()));
                                } else {
                                    cb.send(Err(Error::TransactionClosed));
                                }
                            }
                        }
                    }
                    Ok(())
                }
            }
            impl crate::core::bindings::wasmledger::sql::transaction::HostTransaction
            for BindingsImplState {
                async fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Transaction>,
                ) -> wasmtime::Result<()> {
                    self.table.delete(rep)?;
                    Ok(())
                }
            }
            impl crate::core::bindings::wasmledger::sql::transaction::HostTransactionWithStore
            for BindingsImplState {
                async fn commit<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    this: wasmtime::component::Resource<Transaction>,
                ) -> Result<(), Error> {
                    let tx_impl = accessor
                        .with(|mut access| access.get().table.delete(this))?;
                    tx_impl.commit().await
                }
                async fn rollback<T>(
                    accessor: &wasmtime::component::Accessor<T, Self>,
                    this: wasmtime::component::Resource<Transaction>,
                ) -> Result<(), Error> {
                    let tx_impl = accessor
                        .with(|mut access| access.get().table.delete(this))?;
                    tx_impl.rollback().await
                }
            }
        }
        pub(crate) mod executor {
            use wasmtime::component::Accessor;
            use crate::{
                core::bindings::{
                    BindingsImplState, context::BindingsContext,
                    wasmledger::sql::{
                        query::QueryExecutor, query_types::{SqlQuery, SqlString},
                        util_types::Error,
                    },
                },
                sqldb::SqlDatabase,
            };
            pub(crate) enum QueryOrRaw {
                Query(SqlQuery),
                Raw(SqlString),
            }
            pub(crate) trait ErasedExecutor<D: wasmtime::component::HasData> {
                async fn fetch_all<T>(
                    &self,
                    query: QueryOrRaw,
                    accessor: &Accessor<T, D>,
                ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error>;
                async fn execute<T>(
                    &self,
                    query: QueryOrRaw,
                    accessor: &Accessor<T, D>,
                ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error>;
            }
            impl ErasedExecutor<BindingsImplState> for QueryExecutor {
                async fn fetch_all<T>(
                    &self,
                    query: QueryOrRaw,
                    accessor: &Accessor<T, BindingsImplState>,
                ) -> Result<Vec<<SqlDatabase as sqlx::Database>::Row>, Error> {
                    match self {
                        QueryExecutor::Pool => {
                            let pool = BindingsContext::get_pool();
                            let rows = {
                                match query {
                                    QueryOrRaw::Query(query) => {
                                        let query = {
                                            let args_resource = query.args;
                                            let args = accessor
                                                .with(|mut access| {
                                                    let st = access.get();
                                                    st.table.delete(args_resource).map(|x| x.args.into_inner())
                                                })?;
                                            sqlx::query_with(&query.sql, args)
                                                .persistent(query.persistent.unwrap_or(true))
                                        };
                                        let result = query.fetch_all(pool).await?;
                                        Result::<
                                            _,
                                            crate::core::bindings::wasmledger::sql::util_types::Error,
                                        >::Ok(result)
                                    }
                                    QueryOrRaw::Raw(sql) => {
                                        let query = sqlx::raw_sql(&sql);
                                        let result = query.fetch_all(pool).await?;
                                        Result::<
                                            _,
                                            crate::core::bindings::wasmledger::sql::util_types::Error,
                                        >::Ok(result)
                                    }
                                }
                            }?;
                            Ok(rows)
                        }
                        QueryExecutor::Connection(resource) => {
                            let conn_impl = accessor
                                .with(|mut access| {
                                    let state = access.get();
                                    state.table.get(&resource).map(|conn| conn.clone())
                                })?;
                            conn_impl.fetch_all(query, accessor).await
                        }
                        QueryExecutor::Transaction(resource) => {
                            let tx_impl = accessor
                                .with(|mut access| {
                                    let state = access.get();
                                    state.table.get(&resource).map(|tx| tx.clone())
                                })?;
                            tx_impl.fetch_all(query, accessor).await
                        }
                    }
                }
                async fn execute<T>(
                    &self,
                    query: QueryOrRaw,
                    accessor: &Accessor<T, BindingsImplState>,
                ) -> Result<<SqlDatabase as sqlx::Database>::QueryResult, Error> {
                    match self {
                        QueryExecutor::Pool => {
                            let pool = BindingsContext::get_pool();
                            let rows = {
                                match query {
                                    QueryOrRaw::Query(query) => {
                                        let query = {
                                            let args_resource = query.args;
                                            let args = accessor
                                                .with(|mut access| {
                                                    let st = access.get();
                                                    st.table.delete(args_resource).map(|x| x.args.into_inner())
                                                })?;
                                            sqlx::query_with(&query.sql, args)
                                                .persistent(query.persistent.unwrap_or(true))
                                        };
                                        let result = query.execute(pool).await?;
                                        Result::<
                                            _,
                                            crate::core::bindings::wasmledger::sql::util_types::Error,
                                        >::Ok(result)
                                    }
                                    QueryOrRaw::Raw(sql) => {
                                        let query = sqlx::raw_sql(&sql);
                                        let result = query.execute(pool).await?;
                                        Result::<
                                            _,
                                            crate::core::bindings::wasmledger::sql::util_types::Error,
                                        >::Ok(result)
                                    }
                                }
                            }?;
                            Ok(rows)
                        }
                        QueryExecutor::Connection(resource) => {
                            let conn_impl = accessor
                                .with(|mut access| {
                                    let state = access.get();
                                    state.table.get(&resource).map(|conn| conn.clone())
                                })?;
                            conn_impl.execute(query, accessor).await
                        }
                        QueryExecutor::Transaction(resource) => {
                            let tx_impl = accessor
                                .with(|mut access| {
                                    let state = access.get();
                                    state.table.get(&resource).map(|tx| tx.clone())
                                })?;
                            tx_impl.execute(query, accessor).await
                        }
                    }
                }
            }
        }
    }
}
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
mod postgres {
    mod bindings {
        use wasmtime::component::{HasData, ResourceTable};
        #[doc(hidden)]
        pub use crate::core::bindings::wasmledger::sql::query_types as __with_name0;
        #[doc(hidden)]
        pub use crate::core::bindings::wasmledger::sql::util_types as __with_name1;
        /// Auto-generated bindings for a pre-instantiated version of a
        /// component which implements the world `codecs-postgres`.
        ///
        /// This structure is created through [`CodecsPostgresPre::new`] which
        /// takes a [`InstancePre`](wasmtime::component::InstancePre) that
        /// has been created through a [`Linker`](wasmtime::component::Linker).
        ///
        /// For more information see [`CodecsPostgres`] as well.
        pub struct CodecsPostgresPre<T: 'static> {
            instance_pre: wasmtime::component::InstancePre<T>,
            indices: CodecsPostgresIndices,
        }
        impl<T: 'static> Clone for CodecsPostgresPre<T> {
            fn clone(&self) -> Self {
                Self {
                    instance_pre: self.instance_pre.clone(),
                    indices: self.indices.clone(),
                }
            }
        }
        impl<_T: 'static> CodecsPostgresPre<_T> {
            /// Creates a new copy of `CodecsPostgresPre` bindings which can then
            /// be used to instantiate into a particular store.
            ///
            /// This method may fail if the component behind `instance_pre`
            /// does not have the required exports.
            pub fn new(
                instance_pre: wasmtime::component::InstancePre<_T>,
            ) -> wasmtime::Result<Self> {
                let indices = CodecsPostgresIndices::new(&instance_pre)?;
                Ok(Self { instance_pre, indices })
            }
            pub fn engine(&self) -> &wasmtime::Engine {
                self.instance_pre.engine()
            }
            pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
                &self.instance_pre
            }
            /// Instantiates a new instance of [`CodecsPostgres`] within the
            /// `store` provided.
            ///
            /// This function will use `self` as the pre-instantiated
            /// instance to perform instantiation. Afterwards the preloaded
            /// indices in `self` are used to lookup all exports on the
            /// resulting instance.
            pub fn instantiate(
                &self,
                mut store: impl wasmtime::AsContextMut<Data = _T>,
            ) -> wasmtime::Result<CodecsPostgres> {
                let mut store = store.as_context_mut();
                let instance = self.instance_pre.instantiate(&mut store)?;
                self.indices.load(&mut store, &instance)
            }
        }
        impl<_T: Send + 'static> CodecsPostgresPre<_T> {
            /// Same as [`Self::instantiate`], except with `async`.
            pub async fn instantiate_async(
                &self,
                mut store: impl wasmtime::AsContextMut<Data = _T>,
            ) -> wasmtime::Result<CodecsPostgres> {
                let mut store = store.as_context_mut();
                let instance = self.instance_pre.instantiate_async(&mut store).await?;
                self.indices.load(&mut store, &instance)
            }
        }
        /// Auto-generated bindings for index of the exports of
        /// `codecs-postgres`.
        ///
        /// This is an implementation detail of [`CodecsPostgresPre`] and can
        /// be constructed if needed as well.
        ///
        /// For more information see [`CodecsPostgres`] as well.
        pub struct CodecsPostgresIndices {}
        #[automatically_derived]
        impl ::core::clone::Clone for CodecsPostgresIndices {
            #[inline]
            fn clone(&self) -> CodecsPostgresIndices {
                CodecsPostgresIndices {}
            }
        }
        /// Auto-generated bindings for an instance a component which
        /// implements the world `codecs-postgres`.
        ///
        /// This structure can be created through a number of means
        /// depending on your requirements and what you have on hand:
        ///
        /// * The most convenient way is to use
        ///   [`CodecsPostgres::instantiate`] which only needs a
        ///   [`Store`], [`Component`], and [`Linker`].
        ///
        /// * Alternatively you can create a [`CodecsPostgresPre`] ahead of
        ///   time with a [`Component`] to front-load string lookups
        ///   of exports once instead of per-instantiation. This
        ///   method then uses [`CodecsPostgresPre::instantiate`] to
        ///   create a [`CodecsPostgres`].
        ///
        /// * If you've instantiated the instance yourself already
        ///   then you can use [`CodecsPostgres::new`].
        ///
        /// These methods are all equivalent to one another and move
        /// around the tradeoff of what work is performed when.
        ///
        /// [`Store`]: wasmtime::Store
        /// [`Component`]: wasmtime::component::Component
        /// [`Linker`]: wasmtime::component::Linker
        pub struct CodecsPostgres {}
        const _: () = {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            impl CodecsPostgresIndices {
                /// Creates a new copy of `CodecsPostgresIndices` bindings which can then
                /// be used to instantiate into a particular store.
                ///
                /// This method may fail if the component does not have the
                /// required exports.
                pub fn new<_T>(
                    _instance_pre: &wasmtime::component::InstancePre<_T>,
                ) -> wasmtime::Result<Self> {
                    let _component = _instance_pre.component();
                    let _instance_type = _instance_pre.instance_type();
                    Ok(CodecsPostgresIndices {})
                }
                /// Uses the indices stored in `self` to load an instance
                /// of [`CodecsPostgres`] from the instance provided.
                ///
                /// Note that at this time this method will additionally
                /// perform type-checks of all exports.
                pub fn load(
                    &self,
                    mut store: impl wasmtime::AsContextMut,
                    instance: &wasmtime::component::Instance,
                ) -> wasmtime::Result<CodecsPostgres> {
                    let _ = &mut store;
                    let _instance = instance;
                    Ok(CodecsPostgres {})
                }
            }
            impl CodecsPostgres {
                /// Convenience wrapper around [`CodecsPostgresPre::new`] and
                /// [`CodecsPostgresPre::instantiate`].
                pub fn instantiate<_T>(
                    store: impl wasmtime::AsContextMut<Data = _T>,
                    component: &wasmtime::component::Component,
                    linker: &wasmtime::component::Linker<_T>,
                ) -> wasmtime::Result<CodecsPostgres> {
                    let pre = linker.instantiate_pre(component)?;
                    CodecsPostgresPre::new(pre)?.instantiate(store)
                }
                /// Convenience wrapper around [`CodecsPostgresIndices::new`] and
                /// [`CodecsPostgresIndices::load`].
                pub fn new(
                    mut store: impl wasmtime::AsContextMut,
                    instance: &wasmtime::component::Instance,
                ) -> wasmtime::Result<CodecsPostgres> {
                    let indices = CodecsPostgresIndices::new(
                        &instance.instance_pre(&store),
                    )?;
                    indices.load(&mut store, instance)
                }
                /// Convenience wrapper around [`CodecsPostgresPre::new`] and
                /// [`CodecsPostgresPre::instantiate_async`].
                pub async fn instantiate_async<_T>(
                    store: impl wasmtime::AsContextMut<Data = _T>,
                    component: &wasmtime::component::Component,
                    linker: &wasmtime::component::Linker<_T>,
                ) -> wasmtime::Result<CodecsPostgres>
                where
                    _T: Send,
                {
                    let pre = linker.instantiate_pre(component)?;
                    CodecsPostgresPre::new(pre)?.instantiate_async(store).await
                }
                pub fn add_to_linker<T, D>(
                    linker: &mut wasmtime::component::Linker<T>,
                    host_getter: fn(&mut T) -> D::Data<'_>,
                ) -> wasmtime::Result<()>
                where
                    D: __with_name0::HostWithStore + __with_name1::HostWithStore
                        + wasmledger::sql::codecs::HostWithStore
                        + wasmledger::sql_postgres::postgres_codecs::HostWithStore,
                    for<'a> D::Data<
                        'a,
                    >: __with_name0::Host + __with_name1::Host
                        + wasmledger::sql::codecs::Host
                        + wasmledger::sql_postgres::postgres_codecs::Host,
                    T: 'static,
                {
                    __with_name0::add_to_linker::<T, D>(linker, host_getter)?;
                    __with_name1::add_to_linker::<T, D>(linker, host_getter)?;
                    wasmledger::sql::codecs::add_to_linker::<T, D>(linker, host_getter)?;
                    wasmledger::sql_postgres::postgres_codecs::add_to_linker::<
                        T,
                        D,
                    >(linker, host_getter)?;
                    Ok(())
                }
            }
        };
        pub mod wasmledger {
            pub mod sql {
                pub mod query_types {
                    #[allow(unused_imports)]
                    pub use super::super::super::__with_name0::*;
                }
                pub mod util_types {
                    #[allow(unused_imports)]
                    pub use super::super::super::__with_name1::*;
                }
                #[allow(clippy::all)]
                pub mod codecs {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type Error = super::super::super::__with_name1::Error;
                    const _: () = {
                        if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type PushResult = Result<(), Error>;
                    const _: () = {
                        if !(16
                            == <PushResult as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 16 == <PushResult as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <PushResult as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <PushResult as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type RowIndex = u64;
                    const _: () = {
                        if !(8
                            == <RowIndex as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <RowIndex as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(8
                            == <RowIndex as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <RowIndex as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    #[component(variant)]
                    pub enum ColumnIndex {
                        #[component(name = "index")]
                        Index(u64),
                        #[component(name = "column-name")]
                        ColumnName(wasmtime::component::__internal::String),
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
                    unsafe impl wasmtime::component::Lower for ColumnIndex {
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
                                Self::Index(value) => {
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
                                                        m.map(|p| &raw mut (*p).Index)
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
                                Self::ColumnName(value) => {
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
                                                        m.map(|p| &raw mut (*p).ColumnName)
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
                                Self::Index(value) => {
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
                                Self::ColumnName(value) => {
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
                    unsafe impl wasmtime::component::Lift for ColumnIndex {
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
                                        Self::Index(
                                            <u64 as wasmtime::component::Lift>::linear_lift_from_flat(
                                                cx,
                                                ty
                                                    .cases[0usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                unsafe { &src.payload.Index },
                                            )?,
                                        )
                                    }
                                    1u32 => {
                                        Self::ColumnName(
                                            <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_flat(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                unsafe { &src.payload.ColumnName },
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
                                        Self::Index(
                                            <u64 as wasmtime::component::Lift>::linear_lift_from_memory(
                                                cx,
                                                ty
                                                    .cases[0usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                &payload[..<u64 as wasmtime::component::ComponentType>::SIZE32],
                                            )?,
                                        )
                                    }
                                    1u8 => {
                                        Self::ColumnName(
                                            <wasmtime::component::__internal::String as wasmtime::component::Lift>::linear_lift_from_memory(
                                                cx,
                                                ty
                                                    .cases[1usize]
                                                    .unwrap_or_else(
                                                        wasmtime::component::__internal::bad_type_info,
                                                    ),
                                                &payload[..<wasmtime::component::__internal::String as wasmtime::component::ComponentType>::SIZE32],
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
                        pub struct LowerColumnIndex<T0: Copy, T1: Copy> {
                            tag: wasmtime::ValRaw,
                            payload: LowerPayloadColumnIndex<T0, T1>,
                        }
                        #[automatically_derived]
                        impl<
                            T0: ::core::clone::Clone + Copy,
                            T1: ::core::clone::Clone + Copy,
                        > ::core::clone::Clone for LowerColumnIndex<T0, T1> {
                            #[inline]
                            fn clone(&self) -> LowerColumnIndex<T0, T1> {
                                LowerColumnIndex {
                                    tag: ::core::clone::Clone::clone(&self.tag),
                                    payload: ::core::clone::Clone::clone(&self.payload),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<
                            T0: ::core::marker::Copy + Copy,
                            T1: ::core::marker::Copy + Copy,
                        > ::core::marker::Copy for LowerColumnIndex<T0, T1> {}
                        #[doc(hidden)]
                        #[allow(non_snake_case)]
                        #[repr(C)]
                        union LowerPayloadColumnIndex<T0: Copy, T1: Copy> {
                            Index: T0,
                            ColumnName: T1,
                        }
                        #[automatically_derived]
                        #[allow(non_snake_case)]
                        impl<
                            T0: ::core::marker::Copy + ::core::clone::Clone + Copy,
                            T1: ::core::marker::Copy + ::core::clone::Clone + Copy,
                        > ::core::clone::Clone for LowerPayloadColumnIndex<T0, T1> {
                            #[inline]
                            fn clone(&self) -> LowerPayloadColumnIndex<T0, T1> {
                                let _: ::core::clone::AssertParamIsCopy<Self>;
                                *self
                            }
                        }
                        #[automatically_derived]
                        #[allow(non_snake_case)]
                        impl<
                            T0: ::core::marker::Copy + Copy,
                            T1: ::core::marker::Copy + Copy,
                        > ::core::marker::Copy for LowerPayloadColumnIndex<T0, T1> {}
                        unsafe impl wasmtime::component::ComponentType for ColumnIndex {
                            type Lower = LowerColumnIndex<
                                <u64 as wasmtime::component::ComponentType>::Lower,
                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::Lower,
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
                                            "index",
                                            Some(<u64 as wasmtime::component::ComponentType>::typecheck),
                                        ),
                                        (
                                            "column-name",
                                            Some(
                                                <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::typecheck,
                                            ),
                                        ),
                                    ],
                                )
                            }
                            const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::variant_static(
                                &[
                                    Some(<u64 as wasmtime::component::ComponentType>::ABI),
                                    Some(
                                        <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                    ),
                                ],
                            );
                        }
                        unsafe impl wasmtime::component::__internal::ComponentVariant
                        for ColumnIndex {
                            const CASES: &'static [Option<
                                wasmtime::component::__internal::CanonicalAbiInfo,
                            >] = &[
                                Some(<u64 as wasmtime::component::ComponentType>::ABI),
                                Some(
                                    <wasmtime::component::__internal::String as wasmtime::component::ComponentType>::ABI,
                                ),
                            ];
                        }
                    };
                    impl core::fmt::Debug for ColumnIndex {
                        fn fmt(
                            &self,
                            f: &mut core::fmt::Formatter<'_>,
                        ) -> core::fmt::Result {
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
                    const _: () = {
                        if !(16
                            == <ColumnIndex as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 16 == <ColumnIndex as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(8
                            == <ColumnIndex as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <ColumnIndex as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type ValuePosition = (RowIndex, ColumnIndex);
                    const _: () = {
                        if !(24
                            == <ValuePosition as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 24 == <ValuePosition as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(8
                            == <ValuePosition as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <ValuePosition as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub trait HostWithStore: wasmtime::component::HasData {}
                    impl<_T: ?Sized> HostWithStore for _T
                    where
                        _T: wasmtime::component::HasData,
                    {}
                    pub trait Host {}
                    impl<_T: Host + ?Sized> Host for &mut _T {}
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static,
                    {
                        let mut inst = linker.instance("wasmledger:sql/codecs")?;
                        Ok(())
                    }
                }
            }
            pub mod sql_postgres {
                #[allow(clippy::all)]
                pub mod postgres_codecs {
                    #[allow(unused_imports)]
                    use wasmtime::component::__internal::{anyhow, Box};
                    pub type SqlArguments = super::super::super::__with_name0::SqlArguments;
                    pub type QueryResults = super::super::super::__with_name0::QueryResults;
                    pub type Error = super::super::super::__with_name1::Error;
                    const _: () = {
                        if !(12 == <Error as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 12 == <Error as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Error as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Error as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type PushResult = super::super::super::wasmledger::sql::codecs::PushResult;
                    const _: () = {
                        if !(16
                            == <PushResult as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 16 == <PushResult as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <PushResult as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <PushResult as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type ValuePosition = super::super::super::wasmledger::sql::codecs::ValuePosition;
                    const _: () = {
                        if !(24
                            == <ValuePosition as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 24 == <ValuePosition as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(8
                            == <ValuePosition as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <ValuePosition as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Uuid = wasmtime::component::__internal::String;
                    const _: () = {
                        if !(8 == <Uuid as wasmtime::component::ComponentType>::SIZE32) {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <Uuid as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4 == <Uuid as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Uuid as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub type Hstore = wasmtime::component::__internal::Vec<
                        (
                            wasmtime::component::__internal::String,
                            Option<wasmtime::component::__internal::String>,
                        ),
                    >;
                    const _: () = {
                        if !(8 == <Hstore as wasmtime::component::ComponentType>::SIZE32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 8 == <Hstore as wasmtime::component::ComponentType>::SIZE32",
                            )
                        }
                        if !(4
                            == <Hstore as wasmtime::component::ComponentType>::ALIGN32)
                        {
                            ::core::panicking::panic(
                                "assertion failed: 4 == <Hstore as wasmtime::component::ComponentType>::ALIGN32",
                            )
                        }
                    };
                    pub trait HostWithStore: wasmtime::component::HasData {}
                    impl<_T: ?Sized> HostWithStore for _T
                    where
                        _T: wasmtime::component::HasData,
                    {}
                    pub trait Host {
                        fn push_int16(
                            &mut self,
                            value: Option<i16>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_int16(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<i16>, Error>;
                        fn push_int32(
                            &mut self,
                            value: Option<i32>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_int32(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<i32>, Error>;
                        fn push_int64(
                            &mut self,
                            value: Option<i64>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_int64(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<i64>, Error>;
                        fn push_float32(
                            &mut self,
                            value: Option<f32>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_float32(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<f32>, Error>;
                        fn push_float64(
                            &mut self,
                            value: Option<f64>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_float64(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<f64>, Error>;
                        fn push_string(
                            &mut self,
                            value: Option<wasmtime::component::__internal::String>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_string(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<
                            Option<wasmtime::component::__internal::String>,
                            Error,
                        >;
                        fn push_bool(
                            &mut self,
                            value: Option<bool>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_bool(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<bool>, Error>;
                        fn push_json(
                            &mut self,
                            value: Option<wasmtime::component::__internal::String>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_json(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<
                            Option<wasmtime::component::__internal::String>,
                            Error,
                        >;
                        fn push_uuid(
                            &mut self,
                            value: Option<Uuid>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_uuid(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<Uuid>, Error>;
                        fn push_hstore(
                            &mut self,
                            value: Option<Hstore>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult;
                        fn get_hstore(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<Hstore>, Error>;
                    }
                    impl<_T: Host + ?Sized> Host for &mut _T {
                        fn push_int16(
                            &mut self,
                            value: Option<i16>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_int16(*self, value, to)
                        }
                        fn get_int16(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<i16>, Error> {
                            Host::get_int16(*self, result, position)
                        }
                        fn push_int32(
                            &mut self,
                            value: Option<i32>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_int32(*self, value, to)
                        }
                        fn get_int32(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<i32>, Error> {
                            Host::get_int32(*self, result, position)
                        }
                        fn push_int64(
                            &mut self,
                            value: Option<i64>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_int64(*self, value, to)
                        }
                        fn get_int64(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<i64>, Error> {
                            Host::get_int64(*self, result, position)
                        }
                        fn push_float32(
                            &mut self,
                            value: Option<f32>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_float32(*self, value, to)
                        }
                        fn get_float32(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<f32>, Error> {
                            Host::get_float32(*self, result, position)
                        }
                        fn push_float64(
                            &mut self,
                            value: Option<f64>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_float64(*self, value, to)
                        }
                        fn get_float64(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<f64>, Error> {
                            Host::get_float64(*self, result, position)
                        }
                        fn push_string(
                            &mut self,
                            value: Option<wasmtime::component::__internal::String>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_string(*self, value, to)
                        }
                        fn get_string(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<
                            Option<wasmtime::component::__internal::String>,
                            Error,
                        > {
                            Host::get_string(*self, result, position)
                        }
                        fn push_bool(
                            &mut self,
                            value: Option<bool>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_bool(*self, value, to)
                        }
                        fn get_bool(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<bool>, Error> {
                            Host::get_bool(*self, result, position)
                        }
                        fn push_json(
                            &mut self,
                            value: Option<wasmtime::component::__internal::String>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_json(*self, value, to)
                        }
                        fn get_json(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<
                            Option<wasmtime::component::__internal::String>,
                            Error,
                        > {
                            Host::get_json(*self, result, position)
                        }
                        fn push_uuid(
                            &mut self,
                            value: Option<Uuid>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_uuid(*self, value, to)
                        }
                        fn get_uuid(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<Uuid>, Error> {
                            Host::get_uuid(*self, result, position)
                        }
                        fn push_hstore(
                            &mut self,
                            value: Option<Hstore>,
                            to: wasmtime::component::Resource<SqlArguments>,
                        ) -> PushResult {
                            Host::push_hstore(*self, value, to)
                        }
                        fn get_hstore(
                            &mut self,
                            result: wasmtime::component::Resource<QueryResults>,
                            position: ValuePosition,
                        ) -> Result<Option<Hstore>, Error> {
                            Host::get_hstore(*self, result, position)
                        }
                    }
                    pub fn add_to_linker<T, D>(
                        linker: &mut wasmtime::component::Linker<T>,
                        host_getter: fn(&mut T) -> D::Data<'_>,
                    ) -> wasmtime::Result<()>
                    where
                        D: HostWithStore,
                        for<'a> D::Data<'a>: Host,
                        T: 'static,
                    {
                        let mut inst = linker
                            .instance("wasmledger:sql-postgres/postgres-codecs")?;
                        inst.func_wrap(
                            "push-int16",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<i16>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_int16(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-int16",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_int16(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-int32",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<i32>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_int32(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-int32",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_int32(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-int64",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<i64>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_int64(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-int64",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_int64(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-float32",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<f32>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_float32(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-float32",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_float32(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-float64",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<f64>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_float64(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-float64",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_float64(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-string",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<wasmtime::component::__internal::String>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_string(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-string",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_string(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-bool",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<bool>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_bool(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-bool",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_bool(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-json",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<wasmtime::component::__internal::String>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_json(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-json",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_json(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-uuid",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<Uuid>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_uuid(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-uuid",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_uuid(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "push-hstore",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    Option<Hstore>,
                                    wasmtime::component::Resource<SqlArguments>,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::push_hstore(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        inst.func_wrap(
                            "get-hstore",
                            move |
                                mut caller: wasmtime::StoreContextMut<'_, T>,
                                (
                                    arg0,
                                    arg1,
                                ): (
                                    wasmtime::component::Resource<QueryResults>,
                                    ValuePosition,
                                )|
                            {
                                let host = &mut host_getter(caller.data_mut());
                                let r = Host::get_hstore(host, arg0, arg1);
                                Ok((r,))
                            },
                        )?;
                        Ok(())
                    }
                }
            }
        }
        const _: &str = "package wasmledger:sql;\n\ninterface pool {\n  use util-types.{error};\n  use transaction.{transaction};\n  use connection.{connection};\n\n  begin-transaction: async func() -> result<transaction, error>;\n  acquire-connection: async func() -> result<connection, error>;\n}\n\ninterface transaction {\n  use util-types.{error};\n\n  resource transaction {\n    commit: static async func(this: transaction) -> result<_, error>;\n    rollback: static async func(this: transaction) -> result<_, error>;\n  }\n}\n\ninterface connection {\n  use util-types.{error};\n  use transaction.{transaction};\n\n  resource connection {\n    begin-transaction: async func() -> result<transaction, error>;\n  }\n}";
        const _: &str = "package wasmledger:sql;\n\ninterface codecs {\n  use util-types.{error};\n\n  type push-result = result<_, error>;\n\n  type row-index = u64;\n\n  variant column-index {\n    index(u64),\n    column-name(string)\n  }\n\n  type value-position = tuple<row-index, column-index>;\n}";
        const _: &str = "package wasmledger:sql;\n\ninterface query {\n  use query-types.{sql-query, sql-string, query-results};\n  use util-types.{error};\n  use connection.{connection};\n  use transaction.{transaction};\n\n  variant query-executor {\n    pool,\n    connection(borrow<connection>),\n    transaction(borrow<transaction>)\n  }\n\n  fetch-all: async func(query: sql-query, executor: query-executor) -> result<query-results, error>;\n\n  execute: async func(\n    query: sql-query, executor: query-executor\n  ) -> result<u64, error>;\n\n  execute-raw: async func(query: sql-string, executor: query-executor) -> result<_, error>;\n}";
        const _: &str = "package wasmledger:sql;\n\n/// Types used by components and providers of a sql\ninterface util-types {\n  variant error {\n    /// error occurred while encoding a value\n    encode(string),\n\n    /// error occurred while decoding\n    decode(string),\n\n    /// pool timed out while waiting for an open connection\n    pool-timed-out,\n\n    /// attempted to acquire a connection on a closed pool\n    pool-closed,\n\n    /// got unexpected connection status after attempting to begin transaction\n    begin-failed,\n\n    /// attempted to perform operation on already closed transaction\n    transaction-closed,\n\n    /// generic unexpected error\n    unexpected(string)\n  }\n}\n\ninterface query-types {\n\n  type sql-string = string;\n\n  record sql-query {\n    sql: sql-string,\n    args: sql-arguments,\n    persistent: option<bool>,\n  }\n\n  resource sql-arguments {\n    constructor();\n  }\n\n  resource query-results {\n    row-count: func() -> u64;\n  }\n}";
        const _: &str = "package wasmledger:sql-postgres;\n\ninterface postgres-codecs {\n  use wasmledger:sql/query-types.{sql-arguments, query-results};\n  use wasmledger:sql/util-types.{error};\n  use wasmledger:sql/codecs.{push-result, value-position};\n\n  push-int16: func(value: option<s16>, to: borrow<sql-arguments>) -> push-result;\n  get-int16: func(%result: borrow<query-results>, position: value-position) -> result<option<s16>, error>;\n\n  push-int32: func(value: option<s32>, to: borrow<sql-arguments>) -> push-result;\n  get-int32: func(%result: borrow<query-results>, position: value-position) -> result<option<s32>, error>;\n\n  push-int64: func(value: option<s64>, to: borrow<sql-arguments>) -> push-result;\n  get-int64: func(%result: borrow<query-results>, position: value-position) -> result<option<s64>, error>;\n\n  push-float32: func(value: option<f32>, to: borrow<sql-arguments>) -> push-result;\n  get-float32: func(%result: borrow<query-results>, position: value-position) -> result<option<f32>, error>;\n\n  push-float64: func(value: option<f64>, to: borrow<sql-arguments>) -> push-result;\n  get-float64: func(%result: borrow<query-results>, position: value-position) -> result<option<f64>, error>;\n\n  push-string: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-string: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n  push-bool: func(value: option<bool>, to: borrow<sql-arguments>) -> push-result;\n  get-bool: func(%result: borrow<query-results>, position: value-position) -> result<option<bool>, error>;\n\n  push-json: func(value: option<string>, to: borrow<sql-arguments>) -> push-result;\n  get-json: func(%result: borrow<query-results>, position: value-position) -> result<option<string>, error>;\n\n  type uuid = string;\n\n  push-uuid: func(value: option<uuid>, to: borrow<sql-arguments>) -> push-result;\n  get-uuid: func(%result: borrow<query-results>, position: value-position) -> result<option<uuid>, error>;\n\n  type hstore = list<tuple<string, option<string>>>;\n\n  push-hstore: func(value: option<hstore>, to: borrow<sql-arguments>) -> push-result;\n  get-hstore: func(%result: borrow<query-results>, position: value-position) -> result<option<hstore>, error>;\n\n\n  // variant pg-value {\n  //   /// SQL: NULL\n  //   null,\n\n  //   /// SQL: BIGINT, INT8\n  //   int64(s64),\n  //   /// SQL: BIGINT[], INT8[]\n  //   int64-array(list<s64>),\n\n  //   /// SQL: INTEGER, INT, INT4\n  //   int32(s32),\n  //   /// SQL: INTEGER[], INT4[]\n  //   int32-array(list<s32>),\n\n  //   /// SQL: SMALLINT, INT2\n  //   int2(s16),\n  //   /// SQL: SMALLINT[], INT2[]\n  //   int2-array(list<s16>),\n\n  //   /// SQL: DOUBLE PRECISION, FLOAT8\n  //   float8(hashable-f64),\n  //   /// SQL: DOUBLE PRECISION[], FLOAT8[]\n  //   float8-array(list<hashable-f64>),\n\n  //   /// SQL: REAL, FLOAT4\n  //   float4(hashable-f32),\n  //   /// SQL: REAL[], FLOAT4[]\n  //   float4-array(list<hashable-f32>),\n\n  //   /// SQL: BOOLEAN, BOOL\n  //   %bool(bool),\n  //   /// SQL: BOOLEAN[], BOOL[]\n  //   %bool-array(list<bool>),\n\n  //   /// SQL: NUMERIC, DECIMAL\n  //   numeric(numeric),\n  //   /// SQL: NUMERIC[], DECIMAL[]\n  //   numeric-array(list<numeric>),\n\n  //   /// SQL: BIT(n)\n  //   bit(list<u8>),\n  //   /// SQL: BIT(n)[]\n  //   bit-array(list<list<u8>>),\n\n  //   /// SQL: VARBIT(n)\n  //   varbit(list<u8>),\n  //   /// SQL: BIT VARYING[], VARBIT[]\n  //   varbit-array(list<list<u8>>),\n\n  //   /// SQL: BYTEA\n  //   bytea(list<u8>),\n  //   /// SQL: BYTEA[]\n  //   bytea-array(list<list<u8>>),\n\n  //   /// SQL: CHAR(n), CHARACTER(n)\n  //   %char(string),\n  //   /// SQL: CHAR(n)[]\n  //   %char-array(list<string>),\n\n  //   /// SQL: VARCHAR(n), CHARACTER VARYING(n)\n  //   varchar(string),\n  //   /// SQL: VARCHAR(n)[]\n  //   varchar-array(list<string>),\n\n  //   // Networking\n  //   /// SQL: CIDR\n  //   cidr(string),\n  //   /// SQL: CIDR[]\n  //   cidr-array(list<string>),\n\n  //   /// SQL: INET\n  //   inet(string),\n  //   /// SQL: INET[]\n  //   inet-array(list<string>),\n\n  //   /// SQL: MACADDR (EUI-48)\n  //   macaddr(mac-address-eui48),\n  //   /// SQL: MACADDR[]\n  //   macaddr-array(list<mac-address-eui48>),\n\n  //   /// SQL: MACADDR8 (EUI-64, deprecated)\n  //   macaddr8(mac-address-eui64),\n  //   /// SQL: MACADDR8[]\n  //   macaddr8-array(list<mac-address-eui64>),\n\n  //   // Date-time\n  //   /// SQL: DATE\n  //   date(date),\n  //   /// SQL: DATE[]\n  //   date-array(list<date>),\n\n  //   /// SQL: INTERVAL\n  //   interval(interval),\n  //   /// SQL: INTERVAL[]\n  //   interval-array(list<interval>),\n\n  //   /// SQL: TIME WITHOUT TIME ZONE\n  //   time(time),\n  //   /// SQL: TIME[]\n  //   time-array(list<time>),\n\n  //   /// SQL: TIME WITH TIME ZONE\n  //   time-tz(time-tz),\n  //   /// SQL: TIMETZ[]\n  //   time-tz-array(list<time-tz>),\n\n  //   /// SQL: TIMESTAMP WITHOUT TIME ZONE\n  //   timestamp(timestamp),\n  //   /// SQL: TIMESTAMP[]\n  //   timestamp-array(list<timestamp>),\n\n  //   /// SQL: TIMESTAMP WITH TIME ZONE, TIMESTAMPTZ\n  //   timestamp-tz(timestamp-tz),\n  //   /// SQL: TIMESTAMPTZ[]\n  //   timestamp-tz-array(list<timestamp-tz>),\n\n  //   // JSON\n  //   /// SQL: JSON\n  //   json(string),\n  //   /// SQL: JSON[]\n  //   json-array(list<string>),\n\n  //   /// SQL: JSONB\n  //   jsonb(string),\n  //   /// SQL: JSONB[]\n  //   jsonb-array(list<string>),\n\n  //   /// SQL: MONEY (internal fixed-point type)\n  //   money(numeric),\n  //   /// SQL: MONEY[]\n  //   money-array(list<numeric>),\n\n  //   // Text\n  //   /// SQL: NAME (system identifier type)\n  //   name(string),\n  //   /// SQL: NAME[]\n  //   name-array(list<string>),\n\n  //   /// SQL: TEXT\n  //   text(string),\n  //   /// SQL: TEXT[]\n  //   text-array(list<string>),\n\n  //   /// SQL: XML\n  //   xml(string),\n  //   /// SQL: XML[]\n  //   xml-array(list<string>),\n\n  //   // UUIDs\n  //   /// SQL: UUID\n  //   uuid(string),\n  //   /// SQL: UUID[]\n  //   uuid-array(list<string>),\n\n  //   // Containers\n  //   /// SQL: HSTORE (extension)\n  //   hstore(list<tuple<string, option<string>>>),\n  // }\n}";
        const _: &str = "package wasmledger:sql-host;\n\nworld host {\n  import wasmledger:sql/pool;\n  import wasmledger:sql/transaction;\n  import wasmledger:sql/query;\n  import wasmledger:sql/query-types;\n}";
        const _: &str = "package wasmledger:sql-host-postgres-codecs;\n\nworld codecs-postgres {\n  import wasmledger:sql-postgres/postgres-codecs;\n}";
        pub(crate) struct BindingsImplState {
            table: ResourceTable,
        }
        #[automatically_derived]
        impl ::core::default::Default for BindingsImplState {
            #[inline]
            fn default() -> BindingsImplState {
                BindingsImplState {
                    table: ::core::default::Default::default(),
                }
            }
        }
        impl HasData for BindingsImplState {
            type Data<'a> = &'a mut BindingsImplState where Self: 'a;
        }
        mod codecs {
            use crate::{
                core::bindings::wasmledger::sql::{
                    query::QueryResults, query_types::SqlArguments, util_types::Error,
                },
                postgres::bindings::{
                    BindingsImplState, utils::CodecsUtils,
                    wasmledger::{
                        sql::codecs::{PushResult, ValuePosition},
                        sql_postgres::postgres_codecs::{Hstore, Uuid},
                    },
                },
            };
            use sqlx::types::JsonRawValue;
            use sqlx::{postgres::types::PgHstore, types::Json};
            impl crate::postgres::bindings::wasmledger::sql_postgres::postgres_codecs::Host
            for BindingsImplState {
                fn push_int16(
                    &mut self,
                    value: Option<i16>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_int16(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<i16>, Error> {
                    CodecsUtils::decode(self.table.get(&result)?, position)
                }
                fn push_int32(
                    &mut self,
                    value: Option<i32>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_int32(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<i32>, Error> {
                    CodecsUtils::decode(self.table.get(&result)?, position)
                }
                fn push_int64(
                    &mut self,
                    value: Option<i64>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_int64(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<i64>, Error> {
                    CodecsUtils::decode(self.table.get(&result)?, position)
                }
                fn push_float32(
                    &mut self,
                    value: Option<f32>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_float32(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<f32>, Error> {
                    CodecsUtils::decode(self.table.get(&result)?, position)
                }
                fn push_float64(
                    &mut self,
                    value: Option<f64>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_float64(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<f64>, Error> {
                    CodecsUtils::decode(self.table.get(&result)?, position)
                }
                fn push_string(
                    &mut self,
                    value: Option<wasmtime::component::__internal::String>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_string(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<wasmtime::component::__internal::String>, Error> {
                    CodecsUtils::decode(self.table.get(&result)?, position)
                }
                fn push_bool(
                    &mut self,
                    value: Option<bool>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_bool(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<bool>, Error> {
                    CodecsUtils::decode(self.table.get(&result)?, position)
                }
                fn push_json(
                    &mut self,
                    value: Option<wasmtime::component::__internal::String>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    let to = self.table.get(&to)?;
                    match value {
                        Some(value) => {
                            let raw_value = JsonRawValue::from_string(value)
                                .map_err(|e| Error::Encode(e.to_string()))?;
                            CodecsUtils::encode(Json(raw_value), to)
                        }
                        None => CodecsUtils::encode(value, to),
                    }
                }
                fn get_json(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<wasmtime::component::__internal::String>, Error> {
                    let a = CodecsUtils::decode::<
                        Option<&JsonRawValue>,
                    >(self.table.get(&result)?, position)?;
                    Ok(a.map(|x| x.get().to_string()))
                }
                fn push_uuid(
                    &mut self,
                    value: Option<Uuid>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    let value = value
                        .map(|v| sqlx::types::Uuid::try_parse(v.as_str()))
                        .transpose()
                        .map_err(|e| Error::Encode(e.to_string()))?;
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_uuid(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<Uuid>, Error> {
                    let value: Option<Uuid> = CodecsUtils::decode(
                        self.table.get(&result)?,
                        position,
                    )?;
                    Ok(value.map(|v| v.to_string()))
                }
                fn push_hstore(
                    &mut self,
                    value: Option<Hstore>,
                    to: wasmtime::component::Resource<SqlArguments>,
                ) -> PushResult {
                    let value = value
                        .map(|v: Vec<(String, Option<String>)>| PgHstore(
                            v.into_iter().collect(),
                        ));
                    CodecsUtils::encode(value, self.table.get(&to)?)
                }
                fn get_hstore(
                    &mut self,
                    result: wasmtime::component::Resource<QueryResults>,
                    position: ValuePosition,
                ) -> Result<Option<Hstore>, Error> {
                    let value: Option<PgHstore> = CodecsUtils::decode(
                        self.table.get(&result)?,
                        position,
                    )?;
                    Ok(value.map(|v| v.into_iter().collect()))
                }
            }
        }
        mod utils {
            use std::num::TryFromIntError;
            use crate::{
                core::bindings::{
                    wasmledger::sql::util_types::Error,
                    wasmledger::sql::{query::QueryResults, query_types::SqlArguments},
                },
                postgres::bindings::wasmledger::sql::codecs::{
                    ColumnIndex, PushResult, ValuePosition,
                },
            };
            use sqlx::{Arguments, Encode, Postgres, Type};
            use sqlx::{Decode, Row};
            pub(crate) struct CodecsUtils;
            impl CodecsUtils {
                pub(crate) fn encode<T>(value: T, to: &SqlArguments) -> PushResult
                where
                    T: 'static + Encode<'static, Postgres> + Type<Postgres>,
                {
                    let mut args = to.args.blocking_write();
                    args.add(value).map_err(|e| Error::Encode(e.to_string()))
                }
                pub(crate) fn decode<'a, R>(
                    result: &'a QueryResults,
                    position: ValuePosition,
                ) -> Result<R, Error>
                where
                    R: Decode<'a, Postgres> + Type<Postgres>,
                {
                    let row_index = Self::_index_to_usize(position.0)?;
                    let row = result.results.get(row_index);
                    match row {
                        None => {
                            Err(
                                Error::Decode(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("row index {0} is out of bounds ", row_index),
                                        )
                                    }),
                                ),
                            )
                        }
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
        }
    }
}

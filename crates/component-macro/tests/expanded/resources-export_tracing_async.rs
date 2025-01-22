/// Auto-generated bindings for a pre-instantiated version of a
/// component which implements the world `w`.
///
/// This structure is created through [`WPre::new`] which
/// takes a [`InstancePre`](wasmtime::component::InstancePre) that
/// has been created through a [`Linker`](wasmtime::component::Linker).
///
/// For more information see [`W`] as well.
pub struct WPre<T> {
    instance_pre: wasmtime::component::InstancePre<T>,
    indices: WIndices,
}
impl<T> Clone for WPre<T> {
    fn clone(&self) -> Self {
        Self {
            instance_pre: self.instance_pre.clone(),
            indices: self.indices.clone(),
        }
    }
}
impl<_T> WPre<_T> {
    /// Creates a new copy of `WPre` bindings which can then
    /// be used to instantiate into a particular store.
    ///
    /// This method may fail if the component behind `instance_pre`
    /// does not have the required exports.
    pub fn new(
        instance_pre: wasmtime::component::InstancePre<_T>,
    ) -> wasmtime::Result<Self> {
        let indices = WIndices::new(instance_pre.component())?;
        Ok(Self { instance_pre, indices })
    }
    pub fn engine(&self) -> &wasmtime::Engine {
        self.instance_pre.engine()
    }
    pub fn instance_pre(&self) -> &wasmtime::component::InstancePre<_T> {
        &self.instance_pre
    }
    /// Instantiates a new instance of [`W`] within the
    /// `store` provided.
    ///
    /// This function will use `self` as the pre-instantiated
    /// instance to perform instantiation. Afterwards the preloaded
    /// indices in `self` are used to lookup all exports on the
    /// resulting instance.
    pub async fn instantiate_async(
        &self,
        mut store: impl wasmtime::AsContextMut<Data = _T>,
    ) -> wasmtime::Result<W>
    where
        _T: Send,
    {
        let mut store = store.as_context_mut();
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        self.indices.load(&mut store, &instance)
    }
}
/// Auto-generated bindings for index of the exports of
/// `w`.
///
/// This is an implementation detail of [`WPre`] and can
/// be constructed if needed as well.
///
/// For more information see [`W`] as well.
#[derive(Clone)]
pub struct WIndices {
    interface0: exports::foo::foo::simple_export::GuestIndices,
    interface1: exports::foo::foo::export_using_import::GuestIndices,
    interface2: exports::foo::foo::export_using_export1::GuestIndices,
    interface3: exports::foo::foo::export_using_export2::GuestIndices,
}
/// Auto-generated bindings for an instance a component which
/// implements the world `w`.
///
/// This structure can be created through a number of means
/// depending on your requirements and what you have on hand:
///
/// * The most convenient way is to use
///   [`W::instantiate_async`] which only needs a
///   [`Store`], [`Component`], and [`Linker`].
///
/// * Alternatively you can create a [`WPre`] ahead of
///   time with a [`Component`] to front-load string lookups
///   of exports once instead of per-instantiation. This
///   method then uses [`WPre::instantiate_async`] to
///   create a [`W`].
///
/// * If you've instantiated the instance yourself already
///   then you can use [`W::new`].
///
/// * You can also access the guts of instantiation through
///   [`WIndices::new_instance`] followed
///   by [`WIndices::load`] to crate an instance of this
///   type.
///
/// These methods are all equivalent to one another and move
/// around the tradeoff of what work is performed when.
///
/// [`Store`]: wasmtime::Store
/// [`Component`]: wasmtime::component::Component
/// [`Linker`]: wasmtime::component::Linker
pub struct W {
    interface0: exports::foo::foo::simple_export::Guest,
    interface1: exports::foo::foo::export_using_import::Guest,
    interface2: exports::foo::foo::export_using_export1::Guest,
    interface3: exports::foo::foo::export_using_export2::Guest,
}
const _: () = {
    #[allow(unused_imports)]
    use wasmtime::component::__internal::anyhow;
    impl WIndices {
        /// Creates a new copy of `WIndices` bindings which can then
        /// be used to instantiate into a particular store.
        ///
        /// This method may fail if the component does not have the
        /// required exports.
        pub fn new(
            component: &wasmtime::component::Component,
        ) -> wasmtime::Result<Self> {
            let _component = component;
            let interface0 = exports::foo::foo::simple_export::GuestIndices::new(
                _component,
            )?;
            let interface1 = exports::foo::foo::export_using_import::GuestIndices::new(
                _component,
            )?;
            let interface2 = exports::foo::foo::export_using_export1::GuestIndices::new(
                _component,
            )?;
            let interface3 = exports::foo::foo::export_using_export2::GuestIndices::new(
                _component,
            )?;
            Ok(WIndices {
                interface0,
                interface1,
                interface2,
                interface3,
            })
        }
        /// Creates a new instance of [`WIndices`] from an
        /// instantiated component.
        ///
        /// This method of creating a [`W`] will perform string
        /// lookups for all exports when this method is called. This
        /// will only succeed if the provided instance matches the
        /// requirements of [`W`].
        pub fn new_instance(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let _instance = instance;
            let interface0 = exports::foo::foo::simple_export::GuestIndices::new_instance(
                &mut store,
                _instance,
            )?;
            let interface1 = exports::foo::foo::export_using_import::GuestIndices::new_instance(
                &mut store,
                _instance,
            )?;
            let interface2 = exports::foo::foo::export_using_export1::GuestIndices::new_instance(
                &mut store,
                _instance,
            )?;
            let interface3 = exports::foo::foo::export_using_export2::GuestIndices::new_instance(
                &mut store,
                _instance,
            )?;
            Ok(WIndices {
                interface0,
                interface1,
                interface2,
                interface3,
            })
        }
        /// Uses the indices stored in `self` to load an instance
        /// of [`W`] from the instance provided.
        ///
        /// Note that at this time this method will additionally
        /// perform type-checks of all exports.
        pub fn load(
            &self,
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<W> {
            let _instance = instance;
            let interface0 = self.interface0.load(&mut store, &_instance)?;
            let interface1 = self.interface1.load(&mut store, &_instance)?;
            let interface2 = self.interface2.load(&mut store, &_instance)?;
            let interface3 = self.interface3.load(&mut store, &_instance)?;
            Ok(W {
                interface0,
                interface1,
                interface2,
                interface3,
            })
        }
    }
    impl W {
        /// Convenience wrapper around [`WPre::new`] and
        /// [`WPre::instantiate_async`].
        pub async fn instantiate_async<_T>(
            mut store: impl wasmtime::AsContextMut<Data = _T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<_T>,
        ) -> wasmtime::Result<W>
        where
            _T: Send,
        {
            let pre = linker.instantiate_pre(component)?;
            WPre::new(pre)?.instantiate_async(store).await
        }
        /// Convenience wrapper around [`WIndices::new_instance`] and
        /// [`WIndices::load`].
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<W> {
            let indices = WIndices::new_instance(&mut store, instance)?;
            indices.load(store, instance)
        }
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            T: Send,
            U: foo::foo::transitive_import::Host + Send,
        {
            foo::foo::transitive_import::add_to_linker(linker, get)?;
            Ok(())
        }
        pub fn foo_foo_simple_export(&self) -> &exports::foo::foo::simple_export::Guest {
            &self.interface0
        }
        pub fn foo_foo_export_using_import(
            &self,
        ) -> &exports::foo::foo::export_using_import::Guest {
            &self.interface1
        }
        pub fn foo_foo_export_using_export1(
            &self,
        ) -> &exports::foo::foo::export_using_export1::Guest {
            &self.interface2
        }
        pub fn foo_foo_export_using_export2(
            &self,
        ) -> &exports::foo::foo::export_using_export2::Guest {
            &self.interface3
        }
    }
};
pub mod foo {
    pub mod foo {
        #[allow(clippy::all)]
        pub mod transitive_import {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};
            pub enum Y {}
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait HostY: Sized {
                async fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Y>,
                ) -> wasmtime::Result<()>;
            }
            impl<_T: HostY + ?Sized + Send> HostY for &mut _T {
                async fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Y>,
                ) -> wasmtime::Result<()> {
                    HostY::drop(*self, rep).await
                }
            }
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait Host: Send + HostY + Sized {}
            pub trait GetHost<
                T,
                D,
            >: Fn(T) -> <Self as GetHost<T, D>>::Host + Send + Sync + Copy + 'static {
                type Host: Host + Send;
            }
            impl<F, T, D, O> GetHost<T, D> for F
            where
                F: Fn(T) -> O + Send + Sync + Copy + 'static,
                O: Host + Send,
            {
                type Host = O;
            }
            pub fn add_to_linker_get_host<
                T,
                G: for<'a> GetHost<&'a mut T, T, Host: Host + Send>,
            >(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: G,
            ) -> wasmtime::Result<()>
            where
                T: Send,
            {
                let mut inst = linker.instance("foo:foo/transitive-import")?;
                inst.resource_async(
                    "y",
                    wasmtime::component::ResourceType::host::<Y>(),
                    move |mut store, rep| {
                        wasmtime::component::__internal::Box::new(async move {
                            HostY::drop(
                                    &mut host_getter(store.data_mut()),
                                    wasmtime::component::Resource::new_own(rep),
                                )
                                .await
                        })
                    },
                )?;
                Ok(())
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host + Send,
                T: Send,
            {
                add_to_linker_get_host(linker, get)
            }
            impl<_T: Host + ?Sized + Send> Host for &mut _T {}
        }
    }
}
pub mod exports {
    pub mod foo {
        pub mod foo {
            #[allow(clippy::all)]
            pub mod simple_export {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub type A = wasmtime::component::ResourceAny;
                pub struct GuestA<'a> {
                    funcs: &'a Guest,
                }
                pub struct Guest {
                    constructor_a_constructor: wasmtime::component::Func,
                    static_a_static_a: wasmtime::component::Func,
                    method_a_method_a: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    constructor_a_constructor: wasmtime::component::ComponentExportIndex,
                    static_a_static_a: wasmtime::component::ComponentExportIndex,
                    method_a_method_a: wasmtime::component::ComponentExportIndex,
                }
                impl GuestIndices {
                    /// Constructor for [`GuestIndices`] which takes a
                    /// [`Component`](wasmtime::component::Component) as input and can be executed
                    /// before instantiation.
                    ///
                    /// This constructor can be used to front-load string lookups to find exports
                    /// within a component.
                    pub fn new(
                        component: &wasmtime::component::Component,
                    ) -> wasmtime::Result<GuestIndices> {
                        let (_, instance) = component
                            .export_index(None, "foo:foo/simple-export")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/simple-export`"
                                )
                            })?;
                        Self::_new(|name| {
                            component.export_index(Some(&instance), name).map(|p| p.1)
                        })
                    }
                    /// This constructor is similar to [`GuestIndices::new`] except that it
                    /// performs string lookups after instantiation time.
                    pub fn new_instance(
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<GuestIndices> {
                        let instance_export = instance
                            .get_export(&mut store, None, "foo:foo/simple-export")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/simple-export`"
                                )
                            })?;
                        Self::_new(|name| {
                            instance.get_export(&mut store, Some(&instance_export), name)
                        })
                    }
                    fn _new(
                        mut lookup: impl FnMut(
                            &str,
                        ) -> Option<wasmtime::component::ComponentExportIndex>,
                    ) -> wasmtime::Result<GuestIndices> {
                        let mut lookup = move |name| {
                            lookup(name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/simple-export` does \
                  not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let constructor_a_constructor = lookup("[constructor]a")?;
                        let static_a_static_a = lookup("[static]a.static-a")?;
                        let method_a_method_a = lookup("[method]a.method-a")?;
                        Ok(GuestIndices {
                            constructor_a_constructor,
                            static_a_static_a,
                            method_a_method_a,
                        })
                    }
                    pub fn load(
                        &self,
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<Guest> {
                        let mut store = store.as_context_mut();
                        let _ = &mut store;
                        let _instance = instance;
                        let constructor_a_constructor = *_instance
                            .get_typed_func::<
                                (),
                                (wasmtime::component::ResourceAny,),
                            >(&mut store, &self.constructor_a_constructor)?
                            .func();
                        let static_a_static_a = *_instance
                            .get_typed_func::<
                                (),
                                (u32,),
                            >(&mut store, &self.static_a_static_a)?
                            .func();
                        let method_a_method_a = *_instance
                            .get_typed_func::<
                                (wasmtime::component::ResourceAny,),
                                (u32,),
                            >(&mut store, &self.method_a_method_a)?
                            .func();
                        Ok(Guest {
                            constructor_a_constructor,
                            static_a_static_a,
                            method_a_method_a,
                        })
                    }
                }
                impl Guest {
                    pub fn a(&self) -> GuestA<'_> {
                        GuestA { funcs: self }
                    }
                }
                impl GuestA<'_> {
                    pub async fn call_constructor<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<wasmtime::component::ResourceAny>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/simple-export", function = "[constructor]a",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (wasmtime::component::ResourceAny,),
                            >::new_unchecked(self.funcs.constructor_a_constructor)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_static_a<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<u32>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/simple-export", function = "[static]a.static-a",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (u32,),
                            >::new_unchecked(self.funcs.static_a_static_a)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_method_a<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                    ) -> wasmtime::Result<u32>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/simple-export", function = "[method]a.method-a",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::ResourceAny,),
                                (u32,),
                            >::new_unchecked(self.funcs.method_a_method_a)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod export_using_import {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub type Y = super::super::super::super::foo::foo::transitive_import::Y;
                pub type A = wasmtime::component::ResourceAny;
                pub struct GuestA<'a> {
                    funcs: &'a Guest,
                }
                pub struct Guest {
                    constructor_a_constructor: wasmtime::component::Func,
                    static_a_static_a: wasmtime::component::Func,
                    method_a_method_a: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    constructor_a_constructor: wasmtime::component::ComponentExportIndex,
                    static_a_static_a: wasmtime::component::ComponentExportIndex,
                    method_a_method_a: wasmtime::component::ComponentExportIndex,
                }
                impl GuestIndices {
                    /// Constructor for [`GuestIndices`] which takes a
                    /// [`Component`](wasmtime::component::Component) as input and can be executed
                    /// before instantiation.
                    ///
                    /// This constructor can be used to front-load string lookups to find exports
                    /// within a component.
                    pub fn new(
                        component: &wasmtime::component::Component,
                    ) -> wasmtime::Result<GuestIndices> {
                        let (_, instance) = component
                            .export_index(None, "foo:foo/export-using-import")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/export-using-import`"
                                )
                            })?;
                        Self::_new(|name| {
                            component.export_index(Some(&instance), name).map(|p| p.1)
                        })
                    }
                    /// This constructor is similar to [`GuestIndices::new`] except that it
                    /// performs string lookups after instantiation time.
                    pub fn new_instance(
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<GuestIndices> {
                        let instance_export = instance
                            .get_export(&mut store, None, "foo:foo/export-using-import")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/export-using-import`"
                                )
                            })?;
                        Self::_new(|name| {
                            instance.get_export(&mut store, Some(&instance_export), name)
                        })
                    }
                    fn _new(
                        mut lookup: impl FnMut(
                            &str,
                        ) -> Option<wasmtime::component::ComponentExportIndex>,
                    ) -> wasmtime::Result<GuestIndices> {
                        let mut lookup = move |name| {
                            lookup(name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/export-using-import` does \
                  not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let constructor_a_constructor = lookup("[constructor]a")?;
                        let static_a_static_a = lookup("[static]a.static-a")?;
                        let method_a_method_a = lookup("[method]a.method-a")?;
                        Ok(GuestIndices {
                            constructor_a_constructor,
                            static_a_static_a,
                            method_a_method_a,
                        })
                    }
                    pub fn load(
                        &self,
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<Guest> {
                        let mut store = store.as_context_mut();
                        let _ = &mut store;
                        let _instance = instance;
                        let constructor_a_constructor = *_instance
                            .get_typed_func::<
                                (wasmtime::component::Resource<Y>,),
                                (wasmtime::component::ResourceAny,),
                            >(&mut store, &self.constructor_a_constructor)?
                            .func();
                        let static_a_static_a = *_instance
                            .get_typed_func::<
                                (),
                                (wasmtime::component::Resource<Y>,),
                            >(&mut store, &self.static_a_static_a)?
                            .func();
                        let method_a_method_a = *_instance
                            .get_typed_func::<
                                (
                                    wasmtime::component::ResourceAny,
                                    wasmtime::component::Resource<Y>,
                                ),
                                (wasmtime::component::Resource<Y>,),
                            >(&mut store, &self.method_a_method_a)?
                            .func();
                        Ok(Guest {
                            constructor_a_constructor,
                            static_a_static_a,
                            method_a_method_a,
                        })
                    }
                }
                impl Guest {
                    pub fn a(&self) -> GuestA<'_> {
                        GuestA { funcs: self }
                    }
                }
                impl GuestA<'_> {
                    pub async fn call_constructor<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::Resource<Y>,
                    ) -> wasmtime::Result<wasmtime::component::ResourceAny>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/export-using-import", function = "[constructor]a",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::Resource<Y>,),
                                (wasmtime::component::ResourceAny,),
                            >::new_unchecked(self.funcs.constructor_a_constructor)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_static_a<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<wasmtime::component::Resource<Y>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/export-using-import", function =
                            "[static]a.static-a",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (wasmtime::component::Resource<Y>,),
                            >::new_unchecked(self.funcs.static_a_static_a)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                    pub async fn call_method_a<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                        arg1: wasmtime::component::Resource<Y>,
                    ) -> wasmtime::Result<wasmtime::component::Resource<Y>>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/export-using-import", function =
                            "[method]a.method-a",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (
                                    wasmtime::component::ResourceAny,
                                    wasmtime::component::Resource<Y>,
                                ),
                                (wasmtime::component::Resource<Y>,),
                            >::new_unchecked(self.funcs.method_a_method_a)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), (arg0, arg1))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod export_using_export1 {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub type A = wasmtime::component::ResourceAny;
                pub struct GuestA<'a> {
                    funcs: &'a Guest,
                }
                pub struct Guest {
                    constructor_a_constructor: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    constructor_a_constructor: wasmtime::component::ComponentExportIndex,
                }
                impl GuestIndices {
                    /// Constructor for [`GuestIndices`] which takes a
                    /// [`Component`](wasmtime::component::Component) as input and can be executed
                    /// before instantiation.
                    ///
                    /// This constructor can be used to front-load string lookups to find exports
                    /// within a component.
                    pub fn new(
                        component: &wasmtime::component::Component,
                    ) -> wasmtime::Result<GuestIndices> {
                        let (_, instance) = component
                            .export_index(None, "foo:foo/export-using-export1")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/export-using-export1`"
                                )
                            })?;
                        Self::_new(|name| {
                            component.export_index(Some(&instance), name).map(|p| p.1)
                        })
                    }
                    /// This constructor is similar to [`GuestIndices::new`] except that it
                    /// performs string lookups after instantiation time.
                    pub fn new_instance(
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<GuestIndices> {
                        let instance_export = instance
                            .get_export(&mut store, None, "foo:foo/export-using-export1")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/export-using-export1`"
                                )
                            })?;
                        Self::_new(|name| {
                            instance.get_export(&mut store, Some(&instance_export), name)
                        })
                    }
                    fn _new(
                        mut lookup: impl FnMut(
                            &str,
                        ) -> Option<wasmtime::component::ComponentExportIndex>,
                    ) -> wasmtime::Result<GuestIndices> {
                        let mut lookup = move |name| {
                            lookup(name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/export-using-export1` does \
                  not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let constructor_a_constructor = lookup("[constructor]a")?;
                        Ok(GuestIndices {
                            constructor_a_constructor,
                        })
                    }
                    pub fn load(
                        &self,
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<Guest> {
                        let mut store = store.as_context_mut();
                        let _ = &mut store;
                        let _instance = instance;
                        let constructor_a_constructor = *_instance
                            .get_typed_func::<
                                (),
                                (wasmtime::component::ResourceAny,),
                            >(&mut store, &self.constructor_a_constructor)?
                            .func();
                        Ok(Guest { constructor_a_constructor })
                    }
                }
                impl Guest {
                    pub fn a(&self) -> GuestA<'_> {
                        GuestA { funcs: self }
                    }
                }
                impl GuestA<'_> {
                    pub async fn call_constructor<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                    ) -> wasmtime::Result<wasmtime::component::ResourceAny>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/export-using-export1", function = "[constructor]a",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (),
                                (wasmtime::component::ResourceAny,),
                            >::new_unchecked(self.funcs.constructor_a_constructor)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), ())
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod export_using_export2 {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::{anyhow, Box};
                pub type A = super::super::super::super::exports::foo::foo::export_using_export1::A;
                pub type B = wasmtime::component::ResourceAny;
                pub struct GuestB<'a> {
                    funcs: &'a Guest,
                }
                pub struct Guest {
                    constructor_b_constructor: wasmtime::component::Func,
                }
                #[derive(Clone)]
                pub struct GuestIndices {
                    constructor_b_constructor: wasmtime::component::ComponentExportIndex,
                }
                impl GuestIndices {
                    /// Constructor for [`GuestIndices`] which takes a
                    /// [`Component`](wasmtime::component::Component) as input and can be executed
                    /// before instantiation.
                    ///
                    /// This constructor can be used to front-load string lookups to find exports
                    /// within a component.
                    pub fn new(
                        component: &wasmtime::component::Component,
                    ) -> wasmtime::Result<GuestIndices> {
                        let (_, instance) = component
                            .export_index(None, "foo:foo/export-using-export2")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/export-using-export2`"
                                )
                            })?;
                        Self::_new(|name| {
                            component.export_index(Some(&instance), name).map(|p| p.1)
                        })
                    }
                    /// This constructor is similar to [`GuestIndices::new`] except that it
                    /// performs string lookups after instantiation time.
                    pub fn new_instance(
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<GuestIndices> {
                        let instance_export = instance
                            .get_export(&mut store, None, "foo:foo/export-using-export2")
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "no exported instance named `foo:foo/export-using-export2`"
                                )
                            })?;
                        Self::_new(|name| {
                            instance.get_export(&mut store, Some(&instance_export), name)
                        })
                    }
                    fn _new(
                        mut lookup: impl FnMut(
                            &str,
                        ) -> Option<wasmtime::component::ComponentExportIndex>,
                    ) -> wasmtime::Result<GuestIndices> {
                        let mut lookup = move |name| {
                            lookup(name)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "instance export `foo:foo/export-using-export2` does \
                  not have export `{name}`"
                                    )
                                })
                        };
                        let _ = &mut lookup;
                        let constructor_b_constructor = lookup("[constructor]b")?;
                        Ok(GuestIndices {
                            constructor_b_constructor,
                        })
                    }
                    pub fn load(
                        &self,
                        mut store: impl wasmtime::AsContextMut,
                        instance: &wasmtime::component::Instance,
                    ) -> wasmtime::Result<Guest> {
                        let mut store = store.as_context_mut();
                        let _ = &mut store;
                        let _instance = instance;
                        let constructor_b_constructor = *_instance
                            .get_typed_func::<
                                (wasmtime::component::ResourceAny,),
                                (wasmtime::component::ResourceAny,),
                            >(&mut store, &self.constructor_b_constructor)?
                            .func();
                        Ok(Guest { constructor_b_constructor })
                    }
                }
                impl Guest {
                    pub fn b(&self) -> GuestB<'_> {
                        GuestB { funcs: self }
                    }
                }
                impl GuestB<'_> {
                    pub async fn call_constructor<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                    ) -> wasmtime::Result<wasmtime::component::ResourceAny>
                    where
                        <S as wasmtime::AsContext>::Data: Send,
                    {
                        use tracing::Instrument;
                        let span = tracing::span!(
                            tracing::Level::TRACE, "wit-bindgen export", module =
                            "foo:foo/export-using-export2", function = "[constructor]b",
                        );
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::ResourceAny,),
                                (wasmtime::component::ResourceAny,),
                            >::new_unchecked(self.funcs.constructor_b_constructor)
                        };
                        let (ret0,) = callee
                            .call_async(store.as_context_mut(), (arg0,))
                            .instrument(span.clone())
                            .await?;
                        callee
                            .post_return_async(store.as_context_mut())
                            .instrument(span)
                            .await?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}

//! Owned window and display handles.

use crate::borrowed::{DisplayHandle, HasDisplayHandle, HasWindowHandle, WindowHandle};
use crate::{HandleError, RawDisplayHandle, RawWindowHandle};

use core::fmt;
use core::ptr::NonNull;

macro_rules! handle_type {
    (
        $name: ident,
        $sync: ident,
        $vtable: ident,
        $raw: ident,
        $borrowed: ident,
        $as_borrowed: ident,
        $as_borrowed_method: ident,
        $type: literal
    ) => {
        #[doc = concat!("An owned ", $type, " handle.")]
        ///
        /// This type represents an owned pointer to a reference counted
        #[doc = concat!($type, " handle. The underlying ", $type, " is expected")]
        /// to stay alive as long as this type exists, and must not be closed
        #[doc = concat!("or deallocated until all `", stringify!($name), "`s")]
        /// produced from it have been dropped.
        ///
        /// In other words, this structure is effectively a type-erased
        #[doc = concat!("[`Rc<dyn ", stringify!($as_borrowed), ">`](crate::", stringify!($as_borrowed), ").")]
        /// On some platforms, it's possible to separate the underlying producer
        #[doc = concat!("of a ", $type, " from the actual backing pointer.")]
        ///
        #[doc = concat!("The `", stringify!($name), "` consists of two main")]
        /// parts. The first is a data pointer that represents the underlying
        /// allocation or index of the handle object. The second is a virtual
        /// function table pointer that describes how to interpret the
        /// aforementioned pointer. This virtual function table currently contains
        /// three functions:
        ///
        #[doc = concat!(" - `get`, which returns the underlying [`", stringify!($raw), "`]")]
        ///   that represents the handle.
        #[doc = concat!(" - `retain`, which increments the refcount of the ", $type, ".")]
        #[doc = concat!(" - `release`, which decreases the refcount of the ", $type)]
        ///   and deallocates it if the refcount drops to zero.
        ///
        /// It is expected that the handle is already `retain`ed before it is
        /// created. In practical terms, if `retain` is called `N` times,
        /// `release` will be called `N + 1` times. At the final invocation,
        #[doc = concat!("the handle and ", $type, " will be deallocated.")]
        ///
        /// `retain`ing a handle and then immediately `release`ing the handle
        /// should have no side effects. The `get` function is allowed to be
        /// unpure and have side effects.
        ///
        /// This handle can be created in an unsafe way, from the pointer and
        #[doc = concat!("vtable, using the [`from_raw_parts`](", stringify!($name), "::from_raw_parts)")]
        /// method. Alternatively, this type can be created safely as well. It
        #[doc = concat!("implements `From<Rc<impl [", stringify!($as_borrowed), "](crate::", stringify!($as_borrowed), ")>>`")]
        #[doc = concat!("for types that implement [`", stringify!($as_borrowed), "`].")]
        ///
        /// For a thread-safe equivalent of this handle type, please consult
        #[doc = concat!("[`", stringify!($sync), "`].")]
        // TODO: Add examples
        pub struct $name {
            /// Underlying representative data for the handle.
            data: NonNull<()>,

            /// Pointer to the virtual function table for this handle.
            vtable: &'static $vtable,
        }

        impl fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.$as_borrowed_method() {
                    Ok(handle) => f.debug_tuple(stringify!($name)).field(&handle).finish(),
                    Err(_) => f.debug_tuple(stringify!($name)).field(&format_args!("<error>")).finish()
                }
            }
        }

        impl $name {
            #[doc = concat!("Create a new ", $type, " handle from a")]
            /// data pointer and a virtual function table.
            ///
            /// # Safety
            ///
            /// - The data pointer must be a [valid](https://doc.rust-lang.org/std/ptr/index.html#safety)
            #[doc = concat!("   pointer to an object representing a ", $type, ".")]
            /// - `retain` must increment the handle's reference count. The pointer
            ///   returned by `retain` should be another valid pointer that can be
            ///   used with the active virtual function table.
            /// - `release` must decrement the handle's reference count.
            /// - While a single `retain`ed handle exists, the
            #[doc = concat!("   ", $type, " must remain valid (i.e. not freed or deallocated.")]
            #[doc = concat!("    Note that the term \"", $type, "\" refers to")]
            ///      the underlying system resource.
            #[doc = concat!(" - The [`", stringify!($raw), "`] returned by get must")]
            ///   be valid for the lifetime of this handle and upholds the guarantees in
            #[doc = concat!("    [`", stringify!($borrowed), "::borrow_raw`].")]
            ///
            #[doc = concat!("See the [structure level description](", stringify!($name), ")")]
            /// for further elaboration on the expectations for each vtable method.
            // TODO: Add examples
            #[inline]
            pub const unsafe fn from_raw_parts(ptr: NonNull<()>, vtable: &'static $vtable) -> Self {
                Self { data: ptr, vtable }
            }

            /// Create a new handle from a static reference.
            ///
            /// This allows for a handle to be created from an
            /// `&'static impl HasWindowHandle`. There is no reference counting
            /// needed, so cloning is very cheap.
            ///
            /// This method is currently not exposed publicly, because we don't
            /// want downstream users to stash windows into static variables.
            /// Once we discuss this further, we may expose this publicly.
            #[inline]
            fn from_static<T: $as_borrowed + 'static>(inner: &'static T) -> Self {
                struct Static<T>(T);

                // Implementation: Refcounting does nothing, `get` just calls
                // the `window_handle` method.
                impl<T: $as_borrowed + 'static> Static<T> {
                    const VTABLE: $vtable = $vtable::new(Self::retain, Self::release, Self::get);

                    unsafe fn retain(_ptr: NonNull<()>) {}
                    unsafe fn release(_: NonNull<()>) {}

                    #[inline]
                    unsafe fn get(ptr: NonNull<()>) -> Result<$raw, HandleError> {
                        // SAFETY: `ptr` is always `&'static T`.
                        let inner: &'static T = unsafe {
                            ptr.cast().as_ref()
                        };

                        // Just call the inner method.
                        inner.$as_borrowed_method().map(Into::into)
                    }
                }

                // SAFETY: Above vtable matches all conditions.
                unsafe {
                    $name::from_raw_parts(
                        NonNull::from(inner).cast(),
                        &Static::<T>::VTABLE
                    )
                }
            }

            #[doc = concat!("Create a [`", stringify!($name), "`] that always returns `HandleError::Unavailable`.")]
            ///
            /// ## Example
            ///
            /// ```
            /// use raw_window_handle::{
            #[doc = concat!("   ", stringify!($as_borrowed), ", ", stringify!($name), ", HandleError")]
            /// };
            ///
            #[doc = concat!("let handle = ", stringify!($name), "::unavailable();")]
            #[doc = concat!("let result = handle.", stringify!($as_borrowed_method), "();")]
            /// assert!(matches!(result.unwrap_err(), HandleError::Unavailable));
            /// ```
            #[inline]
            pub fn unavailable() -> Self {
                $sync::unavailable().into()
            }

            #[doc = concat!("Create a [`", stringify!($name), "`] that always returns `HandleError::NotSupported`.")]
            ///
            /// ## Example
            ///
            /// ```
            /// use raw_window_handle::{
            #[doc = concat!("   ", stringify!($as_borrowed), ", ", stringify!($name), ", HandleError")]
            /// };
            ///
            #[doc = concat!("let handle = ", stringify!($name), "::not_supported();")]
            #[doc = concat!("let result = handle.", stringify!($as_borrowed_method), "();")]
            /// assert!(matches!(result.unwrap_err(), HandleError::NotSupported));
            /// ```
            #[inline]
            pub fn not_supported() -> Self {
                $sync::not_supported().into()
            }
        }

        impl $as_borrowed for $name {
            #[inline]
            fn $as_borrowed_method(&self) -> Result<$borrowed<'_>, HandleError> {
                unsafe {
                    // SAFETY: `get` returns a valid raw handle.
                    let raw = (self.vtable.get)(self.data)?;

                    // SAFETY: `get` returns a handle with a lifetime of at least the window handle.
                    Ok($borrowed::borrow_raw(raw))
                }
            }
        }

        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
	            unsafe {
	                // SAFETY: `from_raw_parts` guarantees that `retain` increments
	                // the refcount for the provided pointer.
	                (self.vtable.retain)(self.data);

	                // SAFETY: The `retain`ed pointer is known to be valid for
	                // the current vtable.
	                Self::from_raw_parts(self.data, self.vtable)
	            }
            }
        }

        #[cfg(feature = "alloc")]
        impl<T: $as_borrowed + 'static> From<alloc::rc::Rc<T>> for $name {
            #[doc = concat!("Create a [`", stringify!($name), "`] from a reference counted allocation.")]
            ///
            #[doc = concat!("This allows users to create a [`", stringify!($name), "`]")]
            /// without needing to invoke unsafe code. Since the handle is just
            #[doc = concat!("an `Rc<dyn ", stringify!($as_borrowed), ">`, this allows")]
            #[doc = concat!("for creating a ", $type, " handle from an actual")]
            #[doc = concat!("`Rc<impl ", stringify!($as_borrowed), ">`.")]
            ///
            /// ## Example
            ///
            /// ```no_run
            /// use raw_window_handle::{
            #[doc = concat!("   ", stringify!($as_borrowed), ", ", stringify!($borrowed), ", ", stringify!($name), ", ", stringify!($raw), ", HandleError")]
            /// };
            /// use std::rc::Rc;
            ///
            /// struct Foo;
            ///
            #[doc = concat!("impl ", stringify!($as_borrowed), " for Foo {")]
            ///     #[inline]
            #[doc = concat!("    fn ", stringify!($as_borrowed_method), "(&self) -> Result<", stringify!($borrowed), "<'_>, HandleError> {")]
            ///         /* ... */
            /// # todo!()
            ///     }
            /// }
            ///
            /// let inner = Rc::new(Foo);
            #[doc = concat!("let handle = ", stringify!($name), "::from(inner);")]
            /// ```
            // TODO: expand example
            #[inline]
            fn from(value: alloc::rc::Rc<T>) -> Self {
                struct Refcounted<T>(T);

                impl<T: $as_borrowed + 'static> Refcounted<T> {
                    const VTABLE: $vtable = $vtable::new(Self::retain, Self::release, Self::get);

                    unsafe fn retain(ptr: NonNull<()>) {
                        // SAFETY: Increments `Rc` refcount.
                        unsafe {
                            alloc::rc::Rc::increment_strong_count(ptr.as_ptr().cast::<T>());
                        }
                    }

                    unsafe fn release(ptr: NonNull<()>) {
                        // SAFETY: Decrements `Rc` refcount.
                        unsafe {
                            alloc::rc::Rc::decrement_strong_count(ptr.as_ptr().cast::<T>());
                        }
                    }

                    unsafe fn get(ptr: NonNull<()>) -> Result<$raw, HandleError> {
                        let value: &T = unsafe { ptr.cast().as_ref() };
                        value.$as_borrowed_method().map(Into::into)
                    }
                }

                // SAFETY: Above vtable is valid for this `Rc`
                unsafe {
                    $name::from_raw_parts(
                        NonNull::new_unchecked(alloc::rc::Rc::into_raw(value).cast::<()>() as *mut ()),
                        &Refcounted::<T>::VTABLE
                    )
                }
            }
        }

        #[cfg(feature = "alloc")]
        impl<T: $as_borrowed + Send + Sync + 'static> From<alloc::sync::Arc<T>> for $name {
            #[doc = concat!("Create a [`", stringify!($name), "`] from a reference counted allocation.")]
            ///
            #[doc = concat!("This allows users to create a [`", stringify!($name), "`]")]
            /// without needing to invoke unsafe code. Since the handle is just
            #[doc = concat!("an `Arc<dyn ", stringify!($as_borrowed), ">`, this allows")]
            #[doc = concat!("for creating a ", $type, " handle from an actual")]
            #[doc = concat!("`Arc<impl ", stringify!($as_borrowed), ">`.")]
            ///
            /// ## Example
            ///
            /// ```no_run
            /// use raw_window_handle::{
            #[doc = concat!("   ", stringify!($as_borrowed), ", ", stringify!($borrowed), ", ", stringify!($name), ", ", stringify!($raw), ", HandleError")]
            /// };
            /// use std::sync::Arc;
            ///
            /// struct Foo;
            ///
            #[doc = concat!("impl ", stringify!($as_borrowed), " for Foo {")]
            ///     #[inline]
            #[doc = concat!("    fn ", stringify!($as_borrowed_method), "(&self) -> Result<", stringify!($borrowed), "<'_>, HandleError> {")]
            ///         /* ... */
            /// # todo!()
            ///     }
            /// }
            ///
            /// let inner = Arc::new(Foo);
            #[doc = concat!("let handle = ", stringify!($name), "::from(inner);")]
            /// ```
            // TODO: expand example
            #[inline]
            fn from(value: alloc::sync::Arc<T>) -> Self {
                $sync::from(value).into()
            }
        }

        impl Drop for $name {
            #[inline]
            fn drop(&mut self) {
                // SAFETY: `release` is guaranteed to properly decrement the refcount.
                unsafe {
                    (self.vtable.release)(self.data);
                }
            }
        }

        #[doc = concat!("Thread safe equivalent of [`", stringify!($name), "`].")]
        ///
        #[doc = concat!("See [`", stringify!($name), "`] for more information on")]
        /// the purpose of this type.
        #[derive(Debug, Clone)]
        pub struct $sync {
            inner: $name
        }

        unsafe impl Send for $sync {}
        unsafe impl Sync for $sync {}

        impl $sync {
            #[doc = concat!("Create a new [`", stringify!($sync), "`] from a [`", stringify!($name), "`].")]
            ///
            /// ## Safety
            ///
            /// In addition to the other safety requirements, this handle is
            /// required to be atomic. Therefore, `retain` and `release` must be
            /// atomic. The inner data must also be `Send` and `Sync`.
            #[inline]
            pub const unsafe fn new(handle: $name) -> Self {
                Self { inner: handle }
            }

            /// Create a new handle from a static reference.
            ///
            /// This allows for a handle to be created from an
            /// `&'static impl HasWindowHandle`. There is no reference counting
            /// needed, so cloning is very cheap.
            ///
            /// This method is currently not exposed publicly, because we don't
            /// want downstream users to stash windows into static variables.
            /// Once we discuss this further, we may expose this publicly.
            #[inline]
            fn from_static<T: $as_borrowed + Sync + 'static>(inner: &'static T) -> Self {
                let data = $name::from_static(inner);

                // SAFETY: T is `Sync`, so &T is `Send + Sync`.
                unsafe { Self::new(data) }
            }

            #[doc = concat!("Create a [`", stringify!($sync), "`] that always returns `HandleError::Unavailable`.")]
            ///
            /// ## Example
            ///
            /// ```
            /// use raw_window_handle::{
            #[doc = concat!("   ", stringify!($as_borrowed), ", ", stringify!($sync), ", HandleError")]
            /// };
            ///
            #[doc = concat!("let handle = ", stringify!($sync), "::unavailable();")]
            #[doc = concat!("let result = handle.", stringify!($as_borrowed_method), "();")]
            /// assert!(matches!(result.unwrap_err(), HandleError::Unavailable));
            /// ```
            #[inline]
            pub fn unavailable() -> Self {
                struct Unavailable;

                impl $as_borrowed for Unavailable {
                    #[inline]
                    fn $as_borrowed_method(&self) -> Result<$borrowed<'_>, HandleError> {
                        Err(HandleError::Unavailable)
                    }
                }

                Self::from_static(&Unavailable)
            }

            #[doc = concat!("Create a [`", stringify!($sync), "`] that always returns `HandleError::NotSupported`.")]
            ///
            /// ## Example
            ///
            /// ```
            /// use raw_window_handle::{
            #[doc = concat!("   ", stringify!($as_borrowed), ", ", stringify!($sync), ", HandleError")]
            /// };
            ///
            #[doc = concat!("let handle = ", stringify!($sync), "::not_supported();")]
            #[doc = concat!("let result = handle.", stringify!($as_borrowed_method), "();")]
            /// assert!(matches!(result.unwrap_err(), HandleError::NotSupported));
            /// ```
            #[inline]
            pub fn not_supported() -> Self {
                struct NotSupported;

                impl $as_borrowed for NotSupported {
                    #[inline]
                    fn $as_borrowed_method(&self) -> Result<$borrowed<'_>, HandleError> {
                        Err(HandleError::NotSupported)
                    }
                }

                Self::from_static(&NotSupported)
            }
        }

        impl $as_borrowed for $sync {
            #[inline]
            fn $as_borrowed_method(&self) -> Result<$borrowed<'_>, HandleError> {
                self.inner.$as_borrowed_method()
            }
        }

        impl From<$sync> for $name {
            #[inline]
            fn from(value: $sync) -> $name {
                value.inner
            }
        }

        #[cfg(feature = "alloc")]
        impl<T: $as_borrowed + Send + Sync + 'static> From<alloc::sync::Arc<T>> for $sync {
            #[doc = concat!("Create a [`", stringify!($name), "`] from a reference counted allocation.")]
            ///
            #[doc = concat!("This allows users to create a [`", stringify!($sync), "`]")]
            /// without needing to invoke unsafe code. Since the handle is just
            #[doc = concat!("an `Arc<dyn ", stringify!($as_borrowed), ">`, this allows")]
            #[doc = concat!("for creating a ", $type, " handle from an actual")]
            #[doc = concat!("`Arc<impl ", stringify!($as_borrowed), ">`.")]
            ///
            /// ## Example
            ///
            /// ```no_run
            /// use raw_window_handle::{
            #[doc = concat!("   ", stringify!($as_borrowed), ", ", stringify!($borrowed), ", ", stringify!($name), ", ", stringify!($raw), ", HandleError")]
            /// };
            /// use std::sync::Arc;
            ///
            /// struct Foo;
            ///
            #[doc = concat!("impl ", stringify!($as_borrowed), " for Foo {")]
            ///     #[inline]
            #[doc = concat!("    fn ", stringify!($as_borrowed_method), "(&self) -> Result<", stringify!($borrowed), "<'_>, HandleError> {")]
            ///         /* ... */
            /// # todo!()
            ///     }
            /// }
            ///
            /// let inner = Arc::new(Foo);
            #[doc = concat!("let handle = ", stringify!($name), "::from(inner);")]
            /// ```
            // TODO: expand example
            #[inline]
            fn from(value: alloc::sync::Arc<T>) -> Self {
                struct Atomic<T>(T);

                impl<T: $as_borrowed + 'static> Atomic<T> {
                    const VTABLE: $vtable = $vtable::new(Self::retain, Self::release, Self::get);

                    unsafe fn retain(ptr: NonNull<()>) {
                        // SAFETY: Increments `Arc` refcount.
                        unsafe {
                            alloc::sync::Arc::increment_strong_count(ptr.as_ptr().cast::<T>());
                        }
                    }

                    unsafe fn release(ptr: NonNull<()>) {
                        // SAFETY: Decrements `Arc` refcount.
                        unsafe {
                            alloc::sync::Arc::decrement_strong_count(ptr.as_ptr().cast::<T>());
                        }
                    }

                    unsafe fn get(ptr: NonNull<()>) -> Result<$raw, HandleError> {
                        let value: &T = unsafe { ptr.cast().as_ref() };
                        value.$as_borrowed_method().map(Into::into)
                    }
                }

                // SAFETY: Above vtable is valid for this `Arc`
                unsafe {
                    $sync::new($name::from_raw_parts(
                        NonNull::new_unchecked(alloc::sync::Arc::into_raw(value).cast::<()>() as *mut ()),
                        &Atomic::<T>::VTABLE
                    ))
                }
            }
        }

        #[doc = concat!("Virtual function table for [`", stringify!($name), "`].")]
        ///
        #[doc = concat!("See the documentation for [`", stringify!($name), "`]")]
        /// for more information.
        pub struct $vtable {
            /// Increment the refcount of the handle.
            retain: unsafe fn(NonNull<()>),

            /// Decrement the refcount of the handle.
            release: unsafe fn(NonNull<()>),

            /// Get the raw handle.
            get: unsafe fn(NonNull<()>) -> Result<$raw, HandleError>
        }

        impl $vtable {
            #[doc = concat!("Create a new [`", stringify!($vtable), "`].")]
            ///
            #[doc = concat!("See the [`", stringify!($name), "`] type for more information")]
            /// about the function of each of these methods.
            pub const fn new(
                retain: unsafe fn(NonNull<()>),
                release: unsafe fn(NonNull<()>),
                get: unsafe fn(NonNull<()>) -> Result<$raw, HandleError>
            ) -> Self {
                Self { retain, release, get }
            }
        }
    };
}

handle_type! {
    OwnedWindowHandle,
    SyncWindowHandle,
    WindowHandleVtable,
    RawWindowHandle,
    WindowHandle,
    HasWindowHandle,
    window_handle,
    "window"
}

handle_type! {
    OwnedDisplayHandle,
    SyncDisplayHandle,
    DisplayHandleVtable,
    RawDisplayHandle,
    DisplayHandle,
    HasDisplayHandle,
    display_handle,
    "display"
}

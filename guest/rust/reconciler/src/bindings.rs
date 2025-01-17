#[derive(Clone)]
pub struct ReconcileResult {
    pub requeue: bool,
    pub requeue_after: u32,
    pub object: _rt::String,
}
impl ::core::fmt::Debug for ReconcileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ReconcileResult")
            .field("requeue", &self.requeue)
            .field("requeue-after", &self.requeue_after)
            .field("object", &self.object)
            .finish()
    }
}
#[derive(Clone)]
pub struct ReconcileError {
    pub code: u32,
    pub message: _rt::String,
}
impl ::core::fmt::Debug for ReconcileError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ReconcileError")
            .field("code", &self.code)
            .field("message", &self.message)
            .finish()
    }
}
impl ::core::fmt::Display for ReconcileError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for ReconcileError {}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_reconcile_cabi<T: Guest>(arg0: *mut u8, arg1: usize) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let result1 = T::reconcile(_rt::string_lift(bytes0));
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result1 {
        Ok(e) => {
            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
            let ReconcileResult {
                requeue: requeue3,
                requeue_after: requeue_after3,
                object: object3,
            } = e;
            *ptr2.add(4).cast::<u8>() = (match requeue3 {
                true => 1,
                false => 0,
            }) as u8;
            *ptr2.add(8).cast::<i32>() = _rt::as_i32(requeue_after3);
            let vec4 = (object3.into_bytes()).into_boxed_slice();
            let ptr4 = vec4.as_ptr().cast::<u8>();
            let len4 = vec4.len();
            ::core::mem::forget(vec4);
            *ptr2.add(16).cast::<usize>() = len4;
            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
        }
        Err(e) => {
            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
            let ReconcileError { code: code5, message: message5 } = e;
            *ptr2.add(4).cast::<i32>() = _rt::as_i32(code5);
            let vec6 = (message5.into_bytes()).into_boxed_slice();
            let ptr6 = vec6.as_ptr().cast::<u8>();
            let len6 = vec6.len();
            ::core::mem::forget(vec6);
            *ptr2.add(12).cast::<usize>() = len6;
            *ptr2.add(8).cast::<*mut u8>() = ptr6.cast_mut();
        }
    };
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_reconcile<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(12).cast::<*mut u8>();
            let l2 = *arg0.add(16).cast::<usize>();
            _rt::cabi_dealloc(l1, l2, 1);
        }
        _ => {
            let l3 = *arg0.add(8).cast::<*mut u8>();
            let l4 = *arg0.add(12).cast::<usize>();
            _rt::cabi_dealloc(l3, l4, 1);
        }
    }
}
pub trait Guest {
    /// The `reconcile` function is the main entry point for the reconciler.
    /// It takes a JSON input and returns a JSON output or an error.
    fn reconcile(object: _rt::String) -> Result<ReconcileResult, ReconcileError>;
}
#[doc(hidden)]
macro_rules! __export_world_reconciler_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "reconcile"] unsafe extern "C" fn
        export_reconcile(arg0 : * mut u8, arg1 : usize,) -> * mut u8 {
        $($path_to_types)*:: _export_reconcile_cabi::<$ty > (arg0, arg1) } #[export_name
        = "cabi_post_reconcile"] unsafe extern "C" fn _post_return_reconcile(arg0 : * mut
        u8,) { $($path_to_types)*:: __post_return_reconcile::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_reconciler_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 20]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 20]);
#[allow(dead_code)]
pub mod example {
    #[allow(dead_code)]
    pub mod reconciler {
        #[allow(dead_code, clippy::all)]
        pub mod retrieve {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get(name: &str) -> _rt::String {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = name;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "example:reconciler/retrieve@0.1.0")]
                    extern "C" {
                        #[link_name = "get"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                    _rt::string_lift(bytes4)
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod cli {
        #[allow(dead_code, clippy::all)]
        pub mod stdout {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stdout() -> OutputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stdout@0.2.0")]
                    extern "C" {
                        #[link_name = "get-stdout"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod environment {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_environment() -> _rt::Vec<(_rt::String, _rt::String)> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.0")]
                    extern "C" {
                        #[link_name = "get-environment"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base9 = l1;
                    let len9 = l2;
                    let mut result9 = _rt::Vec::with_capacity(len9);
                    for i in 0..len9 {
                        let base = base9.add(i * 16);
                        let e9 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            let l6 = *base.add(8).cast::<*mut u8>();
                            let l7 = *base.add(12).cast::<usize>();
                            let len8 = l7;
                            let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);
                            (_rt::string_lift(bytes5), _rt::string_lift(bytes8))
                        };
                        result9.push(e9);
                    }
                    _rt::cabi_dealloc(base9, len9 * 16, 4);
                    result9
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_arguments() -> _rt::Vec<_rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.0")]
                    extern "C" {
                        #[link_name = "get-arguments"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base6 = l1;
                    let len6 = l2;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 8);
                        let e6 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            _rt::string_lift(bytes5)
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 8, 4);
                    result6
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn initial_cwd() -> Option<_rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.0")]
                    extern "C" {
                        #[link_name = "initial-cwd"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<*mut u8>();
                                let l3 = *ptr0.add(8).cast::<usize>();
                                let len4 = l3;
                                let bytes4 = _rt::Vec::from_raw_parts(
                                    l2.cast(),
                                    len4,
                                    len4,
                                );
                                _rt::string_lift(bytes4)
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stderr {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stderr() -> OutputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stderr@0.2.0")]
                    extern "C" {
                        #[link_name = "get-stderr"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stdin {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stdin() -> InputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stdin@0.2.0")]
                    extern "C" {
                        #[link_name = "get-stdin"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::InputStream::from_handle(
                        ret as u32,
                    )
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod clocks {
        #[allow(dead_code, clippy::all)]
        pub mod monotonic_clock {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Instant = u64;
            pub type Duration = u64;
            #[allow(unused_unsafe, clippy::all)]
            pub fn now() -> Instant {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "now"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn resolution() -> Duration {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "resolution"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn subscribe_instant(when: Instant) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "subscribe-instant"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(
                        ret as u32,
                    )
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn subscribe_duration(when: Duration) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "subscribe-duration"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod wall_clock {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Datetime {
                pub seconds: u64,
                pub nanoseconds: u32,
            }
            impl ::core::fmt::Debug for Datetime {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Datetime")
                        .field("seconds", &self.seconds)
                        .field("nanoseconds", &self.nanoseconds)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn now() -> Datetime {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/wall-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "now"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i32>();
                    Datetime {
                        seconds: l1 as u64,
                        nanoseconds: l2 as u32,
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn resolution() -> Datetime {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/wall-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "resolution"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i32>();
                    Datetime {
                        seconds: l1 as u64,
                        nanoseconds: l2 as u32,
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod filesystem {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type Error = super::super::super::wasi::io::streams::Error;
            pub type Datetime = super::super::super::wasi::clocks::wall_clock::Datetime;
            pub type Filesize = u64;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum DescriptorType {
                Unknown,
                BlockDevice,
                CharacterDevice,
                Directory,
                Fifo,
                SymbolicLink,
                RegularFile,
                Socket,
            }
            impl ::core::fmt::Debug for DescriptorType {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        DescriptorType::Unknown => {
                            f.debug_tuple("DescriptorType::Unknown").finish()
                        }
                        DescriptorType::BlockDevice => {
                            f.debug_tuple("DescriptorType::BlockDevice").finish()
                        }
                        DescriptorType::CharacterDevice => {
                            f.debug_tuple("DescriptorType::CharacterDevice").finish()
                        }
                        DescriptorType::Directory => {
                            f.debug_tuple("DescriptorType::Directory").finish()
                        }
                        DescriptorType::Fifo => {
                            f.debug_tuple("DescriptorType::Fifo").finish()
                        }
                        DescriptorType::SymbolicLink => {
                            f.debug_tuple("DescriptorType::SymbolicLink").finish()
                        }
                        DescriptorType::RegularFile => {
                            f.debug_tuple("DescriptorType::RegularFile").finish()
                        }
                        DescriptorType::Socket => {
                            f.debug_tuple("DescriptorType::Socket").finish()
                        }
                    }
                }
            }
            impl DescriptorType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> DescriptorType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => DescriptorType::Unknown,
                        1 => DescriptorType::BlockDevice,
                        2 => DescriptorType::CharacterDevice,
                        3 => DescriptorType::Directory,
                        4 => DescriptorType::Fifo,
                        5 => DescriptorType::SymbolicLink,
                        6 => DescriptorType::RegularFile,
                        7 => DescriptorType::Socket,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct DescriptorFlags : u8 { const READ = 1 << 0; const WRITE = 1 << 1;
                const FILE_INTEGRITY_SYNC = 1 << 2; const DATA_INTEGRITY_SYNC = 1 << 3;
                const REQUESTED_WRITE_SYNC = 1 << 4; const MUTATE_DIRECTORY = 1 << 5; }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct PathFlags : u8 { const SYMLINK_FOLLOW = 1 << 0; }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct OpenFlags : u8 { const CREATE = 1 << 0; const DIRECTORY = 1 << 1;
                const EXCLUSIVE = 1 << 2; const TRUNCATE = 1 << 3; }
            }
            pub type LinkCount = u64;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct DescriptorStat {
                pub type_: DescriptorType,
                pub link_count: LinkCount,
                pub size: Filesize,
                pub data_access_timestamp: Option<Datetime>,
                pub data_modification_timestamp: Option<Datetime>,
                pub status_change_timestamp: Option<Datetime>,
            }
            impl ::core::fmt::Debug for DescriptorStat {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("DescriptorStat")
                        .field("type", &self.type_)
                        .field("link-count", &self.link_count)
                        .field("size", &self.size)
                        .field("data-access-timestamp", &self.data_access_timestamp)
                        .field(
                            "data-modification-timestamp",
                            &self.data_modification_timestamp,
                        )
                        .field("status-change-timestamp", &self.status_change_timestamp)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum NewTimestamp {
                NoChange,
                Now,
                Timestamp(Datetime),
            }
            impl ::core::fmt::Debug for NewTimestamp {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        NewTimestamp::NoChange => {
                            f.debug_tuple("NewTimestamp::NoChange").finish()
                        }
                        NewTimestamp::Now => f.debug_tuple("NewTimestamp::Now").finish(),
                        NewTimestamp::Timestamp(e) => {
                            f.debug_tuple("NewTimestamp::Timestamp").field(e).finish()
                        }
                    }
                }
            }
            #[derive(Clone)]
            pub struct DirectoryEntry {
                pub type_: DescriptorType,
                pub name: _rt::String,
            }
            impl ::core::fmt::Debug for DirectoryEntry {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("DirectoryEntry")
                        .field("type", &self.type_)
                        .field("name", &self.name)
                        .finish()
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum ErrorCode {
                Access,
                WouldBlock,
                Already,
                BadDescriptor,
                Busy,
                Deadlock,
                Quota,
                Exist,
                FileTooLarge,
                IllegalByteSequence,
                InProgress,
                Interrupted,
                Invalid,
                Io,
                IsDirectory,
                Loop,
                TooManyLinks,
                MessageSize,
                NameTooLong,
                NoDevice,
                NoEntry,
                NoLock,
                InsufficientMemory,
                InsufficientSpace,
                NotDirectory,
                NotEmpty,
                NotRecoverable,
                Unsupported,
                NoTty,
                NoSuchDevice,
                Overflow,
                NotPermitted,
                Pipe,
                ReadOnly,
                InvalidSeek,
                TextFileBusy,
                CrossDevice,
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => "access",
                        ErrorCode::WouldBlock => "would-block",
                        ErrorCode::Already => "already",
                        ErrorCode::BadDescriptor => "bad-descriptor",
                        ErrorCode::Busy => "busy",
                        ErrorCode::Deadlock => "deadlock",
                        ErrorCode::Quota => "quota",
                        ErrorCode::Exist => "exist",
                        ErrorCode::FileTooLarge => "file-too-large",
                        ErrorCode::IllegalByteSequence => "illegal-byte-sequence",
                        ErrorCode::InProgress => "in-progress",
                        ErrorCode::Interrupted => "interrupted",
                        ErrorCode::Invalid => "invalid",
                        ErrorCode::Io => "io",
                        ErrorCode::IsDirectory => "is-directory",
                        ErrorCode::Loop => "loop",
                        ErrorCode::TooManyLinks => "too-many-links",
                        ErrorCode::MessageSize => "message-size",
                        ErrorCode::NameTooLong => "name-too-long",
                        ErrorCode::NoDevice => "no-device",
                        ErrorCode::NoEntry => "no-entry",
                        ErrorCode::NoLock => "no-lock",
                        ErrorCode::InsufficientMemory => "insufficient-memory",
                        ErrorCode::InsufficientSpace => "insufficient-space",
                        ErrorCode::NotDirectory => "not-directory",
                        ErrorCode::NotEmpty => "not-empty",
                        ErrorCode::NotRecoverable => "not-recoverable",
                        ErrorCode::Unsupported => "unsupported",
                        ErrorCode::NoTty => "no-tty",
                        ErrorCode::NoSuchDevice => "no-such-device",
                        ErrorCode::Overflow => "overflow",
                        ErrorCode::NotPermitted => "not-permitted",
                        ErrorCode::Pipe => "pipe",
                        ErrorCode::ReadOnly => "read-only",
                        ErrorCode::InvalidSeek => "invalid-seek",
                        ErrorCode::TextFileBusy => "text-file-busy",
                        ErrorCode::CrossDevice => "cross-device",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => "",
                        ErrorCode::WouldBlock => "",
                        ErrorCode::Already => "",
                        ErrorCode::BadDescriptor => "",
                        ErrorCode::Busy => "",
                        ErrorCode::Deadlock => "",
                        ErrorCode::Quota => "",
                        ErrorCode::Exist => "",
                        ErrorCode::FileTooLarge => "",
                        ErrorCode::IllegalByteSequence => "",
                        ErrorCode::InProgress => "",
                        ErrorCode::Interrupted => "",
                        ErrorCode::Invalid => "",
                        ErrorCode::Io => "",
                        ErrorCode::IsDirectory => "",
                        ErrorCode::Loop => "",
                        ErrorCode::TooManyLinks => "",
                        ErrorCode::MessageSize => "",
                        ErrorCode::NameTooLong => "",
                        ErrorCode::NoDevice => "",
                        ErrorCode::NoEntry => "",
                        ErrorCode::NoLock => "",
                        ErrorCode::InsufficientMemory => "",
                        ErrorCode::InsufficientSpace => "",
                        ErrorCode::NotDirectory => "",
                        ErrorCode::NotEmpty => "",
                        ErrorCode::NotRecoverable => "",
                        ErrorCode::Unsupported => "",
                        ErrorCode::NoTty => "",
                        ErrorCode::NoSuchDevice => "",
                        ErrorCode::Overflow => "",
                        ErrorCode::NotPermitted => "",
                        ErrorCode::Pipe => "",
                        ErrorCode::ReadOnly => "",
                        ErrorCode::InvalidSeek => "",
                        ErrorCode::TextFileBusy => "",
                        ErrorCode::CrossDevice => "",
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), * self as i32)
                }
            }
            impl std::error::Error for ErrorCode {}
            impl ErrorCode {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ErrorCode {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ErrorCode::Access,
                        1 => ErrorCode::WouldBlock,
                        2 => ErrorCode::Already,
                        3 => ErrorCode::BadDescriptor,
                        4 => ErrorCode::Busy,
                        5 => ErrorCode::Deadlock,
                        6 => ErrorCode::Quota,
                        7 => ErrorCode::Exist,
                        8 => ErrorCode::FileTooLarge,
                        9 => ErrorCode::IllegalByteSequence,
                        10 => ErrorCode::InProgress,
                        11 => ErrorCode::Interrupted,
                        12 => ErrorCode::Invalid,
                        13 => ErrorCode::Io,
                        14 => ErrorCode::IsDirectory,
                        15 => ErrorCode::Loop,
                        16 => ErrorCode::TooManyLinks,
                        17 => ErrorCode::MessageSize,
                        18 => ErrorCode::NameTooLong,
                        19 => ErrorCode::NoDevice,
                        20 => ErrorCode::NoEntry,
                        21 => ErrorCode::NoLock,
                        22 => ErrorCode::InsufficientMemory,
                        23 => ErrorCode::InsufficientSpace,
                        24 => ErrorCode::NotDirectory,
                        25 => ErrorCode::NotEmpty,
                        26 => ErrorCode::NotRecoverable,
                        27 => ErrorCode::Unsupported,
                        28 => ErrorCode::NoTty,
                        29 => ErrorCode::NoSuchDevice,
                        30 => ErrorCode::Overflow,
                        31 => ErrorCode::NotPermitted,
                        32 => ErrorCode::Pipe,
                        33 => ErrorCode::ReadOnly,
                        34 => ErrorCode::InvalidSeek,
                        35 => ErrorCode::TextFileBusy,
                        36 => ErrorCode::CrossDevice,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum Advice {
                Normal,
                Sequential,
                Random,
                WillNeed,
                DontNeed,
                NoReuse,
            }
            impl ::core::fmt::Debug for Advice {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Advice::Normal => f.debug_tuple("Advice::Normal").finish(),
                        Advice::Sequential => {
                            f.debug_tuple("Advice::Sequential").finish()
                        }
                        Advice::Random => f.debug_tuple("Advice::Random").finish(),
                        Advice::WillNeed => f.debug_tuple("Advice::WillNeed").finish(),
                        Advice::DontNeed => f.debug_tuple("Advice::DontNeed").finish(),
                        Advice::NoReuse => f.debug_tuple("Advice::NoReuse").finish(),
                    }
                }
            }
            impl Advice {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Advice {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Advice::Normal,
                        1 => Advice::Sequential,
                        2 => Advice::Random,
                        3 => Advice::WillNeed,
                        4 => Advice::DontNeed,
                        5 => Advice::NoReuse,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct MetadataHashValue {
                pub lower: u64,
                pub upper: u64,
            }
            impl ::core::fmt::Debug for MetadataHashValue {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("MetadataHashValue")
                        .field("lower", &self.lower)
                        .field("upper", &self.upper)
                        .finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Descriptor {
                handle: _rt::Resource<Descriptor>,
            }
            impl Descriptor {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
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
            unsafe impl _rt::WasmResource for Descriptor {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]descriptor"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct DirectoryEntryStream {
                handle: _rt::Resource<DirectoryEntryStream>,
            }
            impl DirectoryEntryStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
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
            unsafe impl _rt::WasmResource for DirectoryEntryStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]directory-entry-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_via_stream(
                    &self,
                    offset: Filesize,
                ) -> Result<InputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read-via-stream"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(offset), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::InputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write_via_stream(
                    &self,
                    offset: Filesize,
                ) -> Result<OutputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.write-via-stream"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(offset), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn append_via_stream(&self) -> Result<OutputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.append-via-stream"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn advise(
                    &self,
                    offset: Filesize,
                    length: Filesize,
                    advice: Advice,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.advise"]
                            fn wit_import(_: i32, _: i64, _: i64, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(offset),
                            _rt::as_i64(length),
                            advice.clone() as i32,
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn sync_data(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.sync-data"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_flags(&self) -> Result<DescriptorFlags, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.get-flags"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    DescriptorFlags::empty()
                                        | DescriptorFlags::from_bits_retain(((l2 as u8) << 0) as _)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_type(&self) -> Result<DescriptorType, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.get-type"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    DescriptorType::_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_size(&self, size: Filesize) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(size), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_times(
                    &self,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let (result1_0, result1_1, result1_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds0,
                                    nanoseconds: nanoseconds0,
                                } = e;
                                (2i32, _rt::as_i64(seconds0), _rt::as_i32(nanoseconds0))
                            }
                        };
                        let (result3_0, result3_1, result3_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (2i32, _rt::as_i64(seconds2), _rt::as_i32(nanoseconds2))
                            }
                        };
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-times"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                            result3_0,
                            result3_1,
                            result3_2,
                            ptr4,
                        );
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        match l5 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr4.add(1).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read(
                    &self,
                    length: Filesize,
                    offset: Filesize,
                ) -> Result<(_rt::Vec<u8>, bool), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read"]
                            fn wit_import(_: i32, _: i64, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(length),
                            _rt::as_i64(offset),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let l5 = i32::from(*ptr0.add(12).cast::<u8>());
                                    (
                                        _rt::Vec::from_raw_parts(l2.cast(), len4, len4),
                                        _rt::bool_lift(l5 as u8),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write(
                    &self,
                    buffer: &[u8],
                    offset: Filesize,
                ) -> Result<Filesize, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let vec0 = buffer;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.write"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i64,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            _rt::as_i64(offset),
                            ptr1,
                        );
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(8).cast::<i64>();
                                    l3 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr1.add(8).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_directory(&self) -> Result<DirectoryEntryStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read-directory"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    DirectoryEntryStream::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn sync(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.sync"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn create_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.create-directory-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn stat(&self) -> Result<DescriptorStat, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 104]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 104],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.stat"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let l3 = *ptr0.add(16).cast::<i64>();
                                    let l4 = *ptr0.add(24).cast::<i64>();
                                    let l5 = i32::from(*ptr0.add(32).cast::<u8>());
                                    let l8 = i32::from(*ptr0.add(56).cast::<u8>());
                                    let l11 = i32::from(*ptr0.add(80).cast::<u8>());
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l2 as u8),
                                        link_count: l3 as u64,
                                        size: l4 as u64,
                                        data_access_timestamp: match l5 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l6 = *ptr0.add(40).cast::<i64>();
                                                    let l7 = *ptr0.add(48).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l6 as u64,
                                                        nanoseconds: l7 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l8 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l9 = *ptr0.add(64).cast::<i64>();
                                                    let l10 = *ptr0.add(72).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l9 as u64,
                                                        nanoseconds: l10 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l11 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l12 = *ptr0.add(88).cast::<i64>();
                                                    let l13 = *ptr0.add(96).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l12 as u64,
                                                        nanoseconds: l13 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l14 = i32::from(*ptr0.add(8).cast::<u8>());
                                    ErrorCode::_lift(l14 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn stat_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<DescriptorStat, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 104]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 104],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.stat-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(8).cast::<u8>());
                                    let l5 = *ptr2.add(16).cast::<i64>();
                                    let l6 = *ptr2.add(24).cast::<i64>();
                                    let l7 = i32::from(*ptr2.add(32).cast::<u8>());
                                    let l10 = i32::from(*ptr2.add(56).cast::<u8>());
                                    let l13 = i32::from(*ptr2.add(80).cast::<u8>());
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l4 as u8),
                                        link_count: l5 as u64,
                                        size: l6 as u64,
                                        data_access_timestamp: match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr2.add(40).cast::<i64>();
                                                    let l9 = *ptr2.add(48).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l8 as u64,
                                                        nanoseconds: l9 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l10 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l11 = *ptr2.add(64).cast::<i64>();
                                                    let l12 = *ptr2.add(72).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l11 as u64,
                                                        nanoseconds: l12 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l13 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l14 = *ptr2.add(88).cast::<i64>();
                                                    let l15 = *ptr2.add(96).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l14 as u64,
                                                        nanoseconds: l15 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l16 = i32::from(*ptr2.add(8).cast::<u8>());
                                    ErrorCode::_lift(l16 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_times_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let (result3_0, result3_1, result3_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (2i32, _rt::as_i64(seconds2), _rt::as_i32(nanoseconds2))
                            }
                        };
                        let (result5_0, result5_1, result5_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds4,
                                    nanoseconds: nanoseconds4,
                                } = e;
                                (2i32, _rt::as_i64(seconds4), _rt::as_i32(nanoseconds4))
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-times-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            result3_0,
                            result3_1,
                            result3_2,
                            result5_0,
                            result5_1,
                            result5_2,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    ErrorCode::_lift(l8 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn link_at(
                    &self,
                    old_path_flags: PathFlags,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let flags0 = old_path_flags;
                        let vec1 = old_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let vec2 = new_path;
                        let ptr2 = vec2.as_ptr().cast::<u8>();
                        let len2 = vec2.len();
                        let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.link-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            (new_descriptor).handle() as i32,
                            ptr2.cast_mut(),
                            len2,
                            ptr3,
                        );
                        let l4 = i32::from(*ptr3.add(0).cast::<u8>());
                        match l4 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr3.add(1).cast::<u8>());
                                    ErrorCode::_lift(l5 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn open_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    open_flags: OpenFlags,
                    flags: DescriptorFlags,
                ) -> Result<Descriptor, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let flags2 = open_flags;
                        let flags3 = flags;
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.open-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            (flags2.bits() >> 0) as i32,
                            (flags3.bits() >> 0) as i32,
                            ptr4,
                        );
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        match l5 {
                            0 => {
                                let e = {
                                    let l6 = *ptr4.add(4).cast::<i32>();
                                    Descriptor::from_handle(l6 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr4.add(4).cast::<u8>());
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn readlink_at(&self, path: &str) -> Result<_rt::String, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.readlink-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<*mut u8>();
                                    let l4 = *ptr1.add(8).cast::<usize>();
                                    let len5 = l4;
                                    let bytes5 = _rt::Vec::from_raw_parts(
                                        l3.cast(),
                                        len5,
                                        len5,
                                    );
                                    _rt::string_lift(bytes5)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr1.add(4).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn remove_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.remove-directory-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn rename_at(
                    &self,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.rename-at"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            (new_descriptor).handle() as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn symlink_at(
                    &self,
                    old_path: &str,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.symlink-at"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn unlink_file_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.unlink-file-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn is_same_object(&self, other: &Descriptor) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.is-same-object"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            (other).handle() as i32,
                        );
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn metadata_hash(&self) -> Result<MetadataHashValue, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 24],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.metadata-hash"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    let l3 = *ptr0.add(16).cast::<i64>();
                                    MetadataHashValue {
                                        lower: l2 as u64,
                                        upper: l3 as u64,
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn metadata_hash_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<MetadataHashValue, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 24],
                        );
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.metadata-hash-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = *ptr2.add(8).cast::<i64>();
                                    let l5 = *ptr2.add(16).cast::<i64>();
                                    MetadataHashValue {
                                        lower: l4 as u64,
                                        upper: l5 as u64,
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr2.add(8).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl DirectoryEntryStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_directory_entry(
                    &self,
                ) -> Result<Option<DirectoryEntry>, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 20]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 20],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]directory-entry-stream.read-directory-entry"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    match l2 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                                let l4 = *ptr0.add(12).cast::<*mut u8>();
                                                let l5 = *ptr0.add(16).cast::<usize>();
                                                let len6 = l5;
                                                let bytes6 = _rt::Vec::from_raw_parts(
                                                    l4.cast(),
                                                    len6,
                                                    len6,
                                                );
                                                DirectoryEntry {
                                                    type_: DescriptorType::_lift(l3 as u8),
                                                    name: _rt::string_lift(bytes6),
                                                }
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
                                    let l7 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn filesystem_error_code(err: &Error) -> Option<ErrorCode> {
                unsafe {
                    #[repr(align(1))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                    extern "C" {
                        #[link_name = "filesystem-error-code"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((err).handle() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                ErrorCode::_lift(l2 as u8)
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod preopens {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Descriptor = super::super::super::wasi::filesystem::types::Descriptor;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_directories() -> _rt::Vec<(Descriptor, _rt::String)> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:filesystem/preopens@0.2.0")]
                    extern "C" {
                        #[link_name = "get-directories"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base7 = l1;
                    let len7 = l2;
                    let mut result7 = _rt::Vec::with_capacity(len7);
                    for i in 0..len7 {
                        let base = base7.add(i * 12);
                        let e7 = {
                            let l3 = *base.add(0).cast::<i32>();
                            let l4 = *base.add(4).cast::<*mut u8>();
                            let l5 = *base.add(8).cast::<usize>();
                            let len6 = l5;
                            let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
                            (
                                super::super::super::wasi::filesystem::types::Descriptor::from_handle(
                                    l3 as u32,
                                ),
                                _rt::string_lift(bytes6),
                            )
                        };
                        result7.push(e7);
                    }
                    _rt::cabi_dealloc(base7, len7 * 12, 4);
                    result7
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod io {
        #[allow(dead_code, clippy::all)]
        pub mod error {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Error {
                handle: _rt::Resource<Error>,
            }
            impl Error {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
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
            unsafe impl _rt::WasmResource for Error {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]error"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Error {
                #[allow(unused_unsafe, clippy::all)]
                pub fn to_debug_string(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]error.to-debug-string"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod poll {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Pollable {
                handle: _rt::Resource<Pollable>,
            }
            impl Pollable {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
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
            unsafe impl _rt::WasmResource for Pollable {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]pollable"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                pub fn ready(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.ready"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                pub fn block(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.block"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn poll(in_: &[&Pollable]) -> _rt::Vec<u32> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = in_;
                    let len0 = vec0.len();
                    let layout0 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec0.len() * 4,
                        4,
                    );
                    let result0 = if layout0.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout0);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = (e).handle() as i32;
                        }
                    }
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                    extern "C" {
                        #[link_name = "poll"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(result0, len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    if layout0.size() != 0 {
                        _rt::alloc::dealloc(result0.cast(), layout0);
                    }
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod streams {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Error = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub enum StreamError {
                LastOperationFailed(Error),
                Closed,
            }
            impl ::core::fmt::Debug for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        StreamError::LastOperationFailed(e) => {
                            f.debug_tuple("StreamError::LastOperationFailed")
                                .field(e)
                                .finish()
                        }
                        StreamError::Closed => {
                            f.debug_tuple("StreamError::Closed").finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for StreamError {}
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct InputStream {
                handle: _rt::Resource<InputStream>,
            }
            impl InputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
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
            unsafe impl _rt::WasmResource for InputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]input-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutputStream {
                handle: _rt::Resource<OutputStream>,
            }
            impl OutputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
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
            unsafe impl _rt::WasmResource for OutputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]output-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read(&self, len: u64) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_read(
                    &self,
                    len: u64,
                ) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn check_write(&self) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.check-write"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write(&self, contents: &[u8]) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_write_and_flush(
                    &self,
                    contents: &[u8],
                ) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-and-flush"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write_zeroes(&self, len: u64) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write-zeroes"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_write_zeroes_and_flush(
                    &self,
                    len: u64,
                ) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-zeroes-and-flush"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod random {
        #[allow(dead_code, clippy::all)]
        pub mod random {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_random_bytes(len: u64) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.0")]
                    extern "C" {
                        #[link_name = "get-random-bytes"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(&len), ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_random_u64() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.0")]
                    extern "C" {
                        #[link_name = "get-random-u64"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
    }
}
mod _rt {
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
            debug_assert!(handle != u32::MAX);
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
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub use alloc_crate::alloc;
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
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
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
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_reconciler_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_reconciler_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_reconciler_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:example:reconciler@0.1.0:reconciler:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 5502] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xfd)\x01A\x02\x01A(\x01\
r\x03\x07requeue\x7f\x0drequeue-aftery\x06objects\x03\0\x10reconcile-result\x03\0\
\0\x01r\x02\x04codey\x07messages\x03\0\x0freconcile-error\x03\0\x02\x01B\x04\x04\
\0\x05error\x03\x01\x01h\0\x01@\x01\x04self\x01\0s\x04\0\x1d[method]error.to-deb\
ug-string\x01\x02\x03\0\x13wasi:io/error@0.2.0\x05\x04\x01B\x0a\x04\0\x08pollabl\
e\x03\x01\x01h\0\x01@\x01\x04self\x01\0\x7f\x04\0\x16[method]pollable.ready\x01\x02\
\x01@\x01\x04self\x01\x01\0\x04\0\x16[method]pollable.block\x01\x03\x01p\x01\x01\
py\x01@\x01\x02in\x04\0\x05\x04\0\x04poll\x01\x06\x03\0\x12wasi:io/poll@0.2.0\x05\
\x05\x02\x03\0\0\x05error\x02\x03\0\x01\x08pollable\x01B(\x02\x03\x02\x01\x06\x04\
\0\x05error\x03\0\0\x02\x03\x02\x01\x07\x04\0\x08pollable\x03\0\x02\x01i\x01\x01\
q\x02\x15last-operation-failed\x01\x04\0\x06closed\0\0\x04\0\x0cstream-error\x03\
\0\x05\x04\0\x0cinput-stream\x03\x01\x04\0\x0doutput-stream\x03\x01\x01h\x07\x01\
p}\x01j\x01\x0a\x01\x06\x01@\x02\x04self\x09\x03lenw\0\x0b\x04\0\x19[method]inpu\
t-stream.read\x01\x0c\x04\0\"[method]input-stream.blocking-read\x01\x0c\x01j\x01\
w\x01\x06\x01@\x02\x04self\x09\x03lenw\0\x0d\x04\0\x19[method]input-stream.skip\x01\
\x0e\x04\0\"[method]input-stream.blocking-skip\x01\x0e\x01i\x03\x01@\x01\x04self\
\x09\0\x0f\x04\0\x1e[method]input-stream.subscribe\x01\x10\x01h\x08\x01@\x01\x04\
self\x11\0\x0d\x04\0![method]output-stream.check-write\x01\x12\x01j\0\x01\x06\x01\
@\x02\x04self\x11\x08contents\x0a\0\x13\x04\0\x1b[method]output-stream.write\x01\
\x14\x04\0.[method]output-stream.blocking-write-and-flush\x01\x14\x01@\x01\x04se\
lf\x11\0\x13\x04\0\x1b[method]output-stream.flush\x01\x15\x04\0$[method]output-s\
tream.blocking-flush\x01\x15\x01@\x01\x04self\x11\0\x0f\x04\0\x1f[method]output-\
stream.subscribe\x01\x16\x01@\x02\x04self\x11\x03lenw\0\x13\x04\0\"[method]outpu\
t-stream.write-zeroes\x01\x17\x04\05[method]output-stream.blocking-write-zeroes-\
and-flush\x01\x17\x01@\x03\x04self\x11\x03src\x09\x03lenw\0\x0d\x04\0\x1c[method\
]output-stream.splice\x01\x18\x04\0%[method]output-stream.blocking-splice\x01\x18\
\x03\0\x15wasi:io/streams@0.2.0\x05\x08\x02\x03\0\x02\x0doutput-stream\x01B\x05\x02\
\x03\x02\x01\x09\x04\0\x0doutput-stream\x03\0\0\x01i\x01\x01@\0\0\x02\x04\0\x0ag\
et-stdout\x01\x03\x03\0\x15wasi:cli/stdout@0.2.0\x05\x0a\x01B\x05\x01p}\x01@\x01\
\x03lenw\0\0\x04\0\x10get-random-bytes\x01\x01\x01@\0\0w\x04\0\x0eget-random-u64\
\x01\x02\x03\0\x18wasi:random/random@0.2.0\x05\x0b\x01B\x0a\x01o\x02ss\x01p\0\x01\
@\0\0\x01\x04\0\x0fget-environment\x01\x02\x01ps\x01@\0\0\x03\x04\0\x0dget-argum\
ents\x01\x04\x01ks\x01@\0\0\x05\x04\0\x0binitial-cwd\x01\x06\x03\0\x1awasi:cli/e\
nvironment@0.2.0\x05\x0c\x01B\x05\x02\x03\x02\x01\x09\x04\0\x0doutput-stream\x03\
\0\0\x01i\x01\x01@\0\0\x02\x04\0\x0aget-stderr\x01\x03\x03\0\x15wasi:cli/stderr@\
0.2.0\x05\x0d\x02\x03\0\x02\x0cinput-stream\x01B\x05\x02\x03\x02\x01\x0e\x04\0\x0c\
input-stream\x03\0\0\x01i\x01\x01@\0\0\x02\x04\0\x09get-stdin\x01\x03\x03\0\x14w\
asi:cli/stdin@0.2.0\x05\x0f\x01B\x0f\x02\x03\x02\x01\x07\x04\0\x08pollable\x03\0\
\0\x01w\x04\0\x07instant\x03\0\x02\x01w\x04\0\x08duration\x03\0\x04\x01@\0\0\x03\
\x04\0\x03now\x01\x06\x01@\0\0\x05\x04\0\x0aresolution\x01\x07\x01i\x01\x01@\x01\
\x04when\x03\0\x08\x04\0\x11subscribe-instant\x01\x09\x01@\x01\x04when\x05\0\x08\
\x04\0\x12subscribe-duration\x01\x0a\x03\0!wasi:clocks/monotonic-clock@0.2.0\x05\
\x10\x01B\x05\x01r\x02\x07secondsw\x0bnanosecondsy\x04\0\x08datetime\x03\0\0\x01\
@\0\0\x01\x04\0\x03now\x01\x02\x04\0\x0aresolution\x01\x02\x03\0\x1cwasi:clocks/\
wall-clock@0.2.0\x05\x11\x02\x03\0\x02\x05error\x02\x03\0\x09\x08datetime\x01Br\x02\
\x03\x02\x01\x0e\x04\0\x0cinput-stream\x03\0\0\x02\x03\x02\x01\x09\x04\0\x0doutp\
ut-stream\x03\0\x02\x02\x03\x02\x01\x12\x04\0\x05error\x03\0\x04\x02\x03\x02\x01\
\x13\x04\0\x08datetime\x03\0\x06\x01w\x04\0\x08filesize\x03\0\x08\x01m\x08\x07un\
known\x0cblock-device\x10character-device\x09directory\x04fifo\x0dsymbolic-link\x0c\
regular-file\x06socket\x04\0\x0fdescriptor-type\x03\0\x0a\x01n\x06\x04read\x05wr\
ite\x13file-integrity-sync\x13data-integrity-sync\x14requested-write-sync\x10mut\
ate-directory\x04\0\x10descriptor-flags\x03\0\x0c\x01n\x01\x0esymlink-follow\x04\
\0\x0apath-flags\x03\0\x0e\x01n\x04\x06create\x09directory\x09exclusive\x08trunc\
ate\x04\0\x0aopen-flags\x03\0\x10\x01w\x04\0\x0alink-count\x03\0\x12\x01k\x07\x01\
r\x06\x04type\x0b\x0alink-count\x13\x04size\x09\x15data-access-timestamp\x14\x1b\
data-modification-timestamp\x14\x17status-change-timestamp\x14\x04\0\x0fdescript\
or-stat\x03\0\x15\x01q\x03\x09no-change\0\0\x03now\0\0\x09timestamp\x01\x07\0\x04\
\0\x0dnew-timestamp\x03\0\x17\x01r\x02\x04type\x0b\x04names\x04\0\x0fdirectory-e\
ntry\x03\0\x19\x01m%\x06access\x0bwould-block\x07already\x0ebad-descriptor\x04bu\
sy\x08deadlock\x05quota\x05exist\x0efile-too-large\x15illegal-byte-sequence\x0bi\
n-progress\x0binterrupted\x07invalid\x02io\x0cis-directory\x04loop\x0etoo-many-l\
inks\x0cmessage-size\x0dname-too-long\x09no-device\x08no-entry\x07no-lock\x13ins\
ufficient-memory\x12insufficient-space\x0dnot-directory\x09not-empty\x0fnot-reco\
verable\x0bunsupported\x06no-tty\x0eno-such-device\x08overflow\x0dnot-permitted\x04\
pipe\x09read-only\x0cinvalid-seek\x0etext-file-busy\x0ccross-device\x04\0\x0aerr\
or-code\x03\0\x1b\x01m\x06\x06normal\x0asequential\x06random\x09will-need\x09don\
t-need\x08no-reuse\x04\0\x06advice\x03\0\x1d\x01r\x02\x05lowerw\x05upperw\x04\0\x13\
metadata-hash-value\x03\0\x1f\x04\0\x0adescriptor\x03\x01\x04\0\x16directory-ent\
ry-stream\x03\x01\x01h!\x01i\x01\x01j\x01$\x01\x1c\x01@\x02\x04self#\x06offset\x09\
\0%\x04\0\"[method]descriptor.read-via-stream\x01&\x01i\x03\x01j\x01'\x01\x1c\x01\
@\x02\x04self#\x06offset\x09\0(\x04\0#[method]descriptor.write-via-stream\x01)\x01\
@\x01\x04self#\0(\x04\0$[method]descriptor.append-via-stream\x01*\x01j\0\x01\x1c\
\x01@\x04\x04self#\x06offset\x09\x06length\x09\x06advice\x1e\0+\x04\0\x19[method\
]descriptor.advise\x01,\x01@\x01\x04self#\0+\x04\0\x1c[method]descriptor.sync-da\
ta\x01-\x01j\x01\x0d\x01\x1c\x01@\x01\x04self#\0.\x04\0\x1c[method]descriptor.ge\
t-flags\x01/\x01j\x01\x0b\x01\x1c\x01@\x01\x04self#\00\x04\0\x1b[method]descript\
or.get-type\x011\x01@\x02\x04self#\x04size\x09\0+\x04\0\x1b[method]descriptor.se\
t-size\x012\x01@\x03\x04self#\x15data-access-timestamp\x18\x1bdata-modification-\
timestamp\x18\0+\x04\0\x1c[method]descriptor.set-times\x013\x01p}\x01o\x024\x7f\x01\
j\x015\x01\x1c\x01@\x03\x04self#\x06length\x09\x06offset\x09\06\x04\0\x17[method\
]descriptor.read\x017\x01j\x01\x09\x01\x1c\x01@\x03\x04self#\x06buffer4\x06offse\
t\x09\08\x04\0\x18[method]descriptor.write\x019\x01i\"\x01j\x01:\x01\x1c\x01@\x01\
\x04self#\0;\x04\0![method]descriptor.read-directory\x01<\x04\0\x17[method]descr\
iptor.sync\x01-\x01@\x02\x04self#\x04paths\0+\x04\0&[method]descriptor.create-di\
rectory-at\x01=\x01j\x01\x16\x01\x1c\x01@\x01\x04self#\0>\x04\0\x17[method]descr\
iptor.stat\x01?\x01@\x03\x04self#\x0apath-flags\x0f\x04paths\0>\x04\0\x1a[method\
]descriptor.stat-at\x01@\x01@\x05\x04self#\x0apath-flags\x0f\x04paths\x15data-ac\
cess-timestamp\x18\x1bdata-modification-timestamp\x18\0+\x04\0\x1f[method]descri\
ptor.set-times-at\x01A\x01@\x05\x04self#\x0eold-path-flags\x0f\x08old-paths\x0en\
ew-descriptor#\x08new-paths\0+\x04\0\x1a[method]descriptor.link-at\x01B\x01i!\x01\
j\x01\xc3\0\x01\x1c\x01@\x05\x04self#\x0apath-flags\x0f\x04paths\x0aopen-flags\x11\
\x05flags\x0d\0\xc4\0\x04\0\x1a[method]descriptor.open-at\x01E\x01j\x01s\x01\x1c\
\x01@\x02\x04self#\x04paths\0\xc6\0\x04\0\x1e[method]descriptor.readlink-at\x01G\
\x04\0&[method]descriptor.remove-directory-at\x01=\x01@\x04\x04self#\x08old-path\
s\x0enew-descriptor#\x08new-paths\0+\x04\0\x1c[method]descriptor.rename-at\x01H\x01\
@\x03\x04self#\x08old-paths\x08new-paths\0+\x04\0\x1d[method]descriptor.symlink-\
at\x01I\x04\0![method]descriptor.unlink-file-at\x01=\x01@\x02\x04self#\x05other#\
\0\x7f\x04\0![method]descriptor.is-same-object\x01J\x01j\x01\x20\x01\x1c\x01@\x01\
\x04self#\0\xcb\0\x04\0\x20[method]descriptor.metadata-hash\x01L\x01@\x03\x04sel\
f#\x0apath-flags\x0f\x04paths\0\xcb\0\x04\0#[method]descriptor.metadata-hash-at\x01\
M\x01h\"\x01k\x1a\x01j\x01\xcf\0\x01\x1c\x01@\x01\x04self\xce\0\0\xd0\0\x04\03[m\
ethod]directory-entry-stream.read-directory-entry\x01Q\x01h\x05\x01k\x1c\x01@\x01\
\x03err\xd2\0\0\xd3\0\x04\0\x15filesystem-error-code\x01T\x03\0\x1bwasi:filesyst\
em/types@0.2.0\x05\x14\x02\x03\0\x0a\x0adescriptor\x01B\x07\x02\x03\x02\x01\x15\x04\
\0\x0adescriptor\x03\0\0\x01i\x01\x01o\x02\x02s\x01p\x03\x01@\0\0\x04\x04\0\x0fg\
et-directories\x01\x05\x03\0\x1ewasi:filesystem/preopens@0.2.0\x05\x16\x01B\x02\x01\
@\x01\x04names\0s\x04\0\x03get\x01\0\x03\0!example:reconciler/retrieve@0.1.0\x05\
\x17\x01j\x01\x01\x01\x03\x01@\x01\x06objects\0\x18\x04\0\x09reconcile\x01\x19\x04\
\0#example:reconciler/reconciler@0.1.0\x04\0\x0b\x10\x01\0\x0areconciler\x03\0\0\
\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bind\
gen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

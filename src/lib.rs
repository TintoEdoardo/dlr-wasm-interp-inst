#![no_std]
extern crate alloc;
extern crate wasm;

use alloc::vec::Vec;
use core::panic::PanicInfo;
use core::{alloc::GlobalAlloc, mem::MaybeUninit};

/// A Global Allocator taken from:
/// https://github.com/DLR-FT/dynamic-multicore-interference-mitigation/
pub struct SimpleAlloc {
    inner: spin::mutex::SpinMutex<RawSimpleAlloc>,
}

impl SimpleAlloc {
    pub const fn new() -> Self {
        Self {
            inner: spin::mutex::SpinMutex::new(RawSimpleAlloc::new()),
        }
    }

    pub unsafe fn init(&self, buf: &[MaybeUninit<u8>]) {
        unsafe { self.inner.lock().init(buf) };
    }
}

unsafe impl GlobalAlloc for SimpleAlloc {
    unsafe fn alloc(&self, layout: ::core::alloc::Layout) -> *mut u8 {
        let mut raw = self.inner.lock();
        unsafe { raw.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: ::core::alloc::Layout) {
        let mut raw = self.inner.lock();
        unsafe { raw.dealloc(ptr, layout) }
    }
}

struct RawSimpleAlloc {
    start_addr: usize,
    end_addr: usize,
}

impl RawSimpleAlloc {
    const fn new() -> Self {
        Self {
            start_addr: 0,
            end_addr: 0,
        }
    }

    unsafe fn init(&mut self, buf: &[MaybeUninit<u8>]) {
        self.start_addr = (buf as *const _ as *const u8).addr();
        self.end_addr = unsafe { (buf as *const _ as *const u8).add(buf.len()) }.addr();
    }

    unsafe fn alloc(&mut self, layout: ::core::alloc::Layout) -> *mut u8 {
        let mut ptr = self.start_addr as *mut u8;
        let end_ptr = self.end_addr as *mut u8;

        let align_offset = ptr.align_offset(layout.align());
        ptr = unsafe { ptr.byte_add(align_offset) };

        let next_ptr = unsafe { ptr.byte_add(layout.size()) };

        if (ptr >= end_ptr) || (next_ptr >= end_ptr) {
            panic!("alloc failed: out of memory.")
        }

        self.start_addr = next_ptr.addr();
        self.end_addr = end_ptr.addr();

        ptr
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: ::core::alloc::Layout) {}
}
#[global_allocator]
pub static ALLOCATOR: SimpleAlloc = SimpleAlloc::new();

#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link(name = "example_of_import")]
unsafe extern "C" {
    fn example_of_import();
}

#[unsafe(no_mangle)]
pub extern "C" fn start_the_interpreter() -> i32 {

    unsafe { example_of_import(); }

    let heap_buf : [MaybeUninit<u8>; 1024 * 1024] = unsafe { MaybeUninit::uninit().assume_init() };

    unsafe { ALLOCATOR.init(&heap_buf) };

    let wasm_bytes : Vec<u8> = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 127, 1, 127, 3, 2, 1, 0, 5, 3, 1, 0, 1, 7, 11, 1, 7, 97, 100, 100, 95, 111, 110, 101, 0, 0, 10, 11, 1, 9, 1, 1, 127, 32, 0, 65, 1, 106, 11, 0, 40, 4, 110, 97, 109, 101, 1, 10, 1, 0, 7, 97, 100, 100, 95, 111, 110, 101, 2, 21, 1, 0, 2, 0, 1, 120, 1, 13, 117, 110, 117, 110, 115, 101, 100, 95, 108, 111, 99, 97, 108].to_vec();

    let validation_info = wasm::validate(&wasm_bytes).unwrap();

    let mut store = wasm::Store::new(());

    let module = store
        .module_instantiate(&validation_info, Vec::new(), None).unwrap().module_addr;

    let add_one = store
        .instance_export(module, "add_one")
        .unwrap()
        .as_func()
        .unwrap();

    let twelve: i32 = store.invoke_typed_without_fuel(add_one, 11).unwrap();
    assert_eq!(twelve, 12);

    twelve

}
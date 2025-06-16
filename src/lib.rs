// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::{string::String, vec::Vec};

// If we wanted to use the types mentioned above, it would also be
// a good idea to use the allocator we pulled in as a dependency
// in Cargo.toml, like so:
//#[global_allocator]
//static ALLOC: miden_sdk_alloc::BumpAlloc = miden_sdk_alloc::BumpAlloc::new();

// Required for no-std crates
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Compiles to a trap instruction in WebAssembly
    core::arch::wasm32::unreachable()
}

// Marking the function no_mangle ensures that it is exported
// from the compiled binary as `fib`, otherwise it would have
// a mangled name that has no stable form.
//
// You can specify a different name from the library than the
// name in the source code using the `#[export_name = "foo"]`
// attribute, which will make the function callable as `foo`
// externally (in this example)
#[no_mangle]
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// The entry point to the Miden program.
#[no_mangle]
fn entrypoint(n: u32) -> bool {
    is_prime(n)
}
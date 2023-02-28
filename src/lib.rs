// not to link against the standard library, only links against core.
// 'core' makes no assumptions about the system the program will run on,
// it provides APIs for language primitives, like float, strings,
// APIs that expose processor features like atomic operations and SIMD instructions.
// but lack of IO, heap memory allocations.
#![no_std]
// mod math {
//     mod math_js {
//         #[link(wasm_import_module = "Math")]
//         extern "C" {
//             pub fn random() -> f64;
//         }
//     }
//
//     pub fn random() -> f64 {
//         unsafe { math_js::random() }
//     }
// }
// One of the few occastions where we have to use `extern crate`
// even in Rust Edition 2021.
extern crate alloc;
use alloc::vec::Vec;

static PRIMES: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

// const _: () = {
//     #[link_section = "surmsection"]
//     static SECTION_CONTENT: [u8; 11] = *b"hello world";
// };

// #[export_name = "add"]
// pub fn add(left: f64, right: f64) -> f64 {
//     left + right + unsafe { math::random() }
// }

#[no_mangle]
extern "C" fn nth_prime(n: usize) -> usize{
    // if n < 0 || n >= PRIMES.len() { return -1; }
    // PRIMES[n]
    // PRIMES.get(n).copied().unwrap_or(-1)
    // unsafe { *PRIMES.get_unchecked(n) }
    // Please enjoy this horrible implementation of
    // The Sieve of Eratosthenes.
    let mut primes: Vec<usize> = Vec::new();
    let mut current = 2;
    while primes.len() < n {
        if !primes.iter().any(|prime| current % prime == 0) {
            primes.push(current);
        }
        current += 1;
    }
    primes.into_iter().last().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

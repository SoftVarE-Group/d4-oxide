use num_bigint::{BigInt, Sign};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("Adapter.h");

        fn compile_ddnnf(input: String, output: String);
        fn compile_ddnnf_proj(input: String, output: String);
        fn count(input: String) -> Vec<u8>;
        fn count_proj(input: String) -> Vec<u8>;
    }
}

/// Compiles the CNF from the input file into a d-DNNF and saves it in the output file.
///
/// This is equivalent to running `d4 -i input -m ddnnf-compiler --dump-ddnnf output`.
pub fn compile_ddnnf(input: String, output: String) {
    ffi::compile_ddnnf(input, output);
}

/// Compiles the projected CNF from the input file into a d-DNNF and saves it in the output file.
///
/// This is equivalent to running `d4 -i input -m proj-ddnnf-compiler --dump-ddnnf output`.
pub fn compile_ddnnf_proj(input: String, output: String) {
    ffi::compile_ddnnf_proj(input, output);
}

/// Calculates the model count of the given input file without constructing a d-DNNF.
pub fn count(input: String) -> BigInt {
    let raw = ffi::count(input);
    BigInt::from_bytes_be(Sign::Plus, raw.as_slice())
}

/// Calculates the projected model count of the given input file without constructing a d-DNNF.
pub fn count_proj(input: String) -> BigInt {
    let raw = ffi::count_proj(input);
    BigInt::from_bytes_be(Sign::Plus, raw.as_slice())
}

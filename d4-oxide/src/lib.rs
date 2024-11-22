#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("Adapter.h");

        fn compile_ddnnf(input: String, output: String);
        fn compile_ddnnf_proj(input: String, output: String);
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

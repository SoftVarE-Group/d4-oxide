use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {
    // Find d4 sources to build.
    let d4_sources: Vec<PathBuf> = glob("d4/src/**/*.cpp")
        .expect("Failed to create glob pattern for d4 sources.")
        .map(|result| result.expect("Failed to find d4 source file."))
        .filter(|path| !path.ends_with("Main.cpp"))
        .filter(|path| !path.ends_with("DdnnfCompilerRunnable.cpp"))
        .filter(|path| !path.ends_with("WrapperGlucose.cpp"))
        .collect();

    // Setup include paths for compilation.
    let mut includes = vec![
        "include".to_string(),
        "d4/src".to_string(),
        "d4".to_string(),
    ];

    // Optionally add Mt-KaHyPar include path.
    if let Ok(mtkahypar_include) = env::var("DEP_MTKAHYPAR_INCLUDE_DIR") {
        includes.push(mtkahypar_include.clone());
    }

    // Build d4.
    cxx_build::bridge("src/lib.rs")
        .includes(includes)
        .file("src/Adapter.cc")
        .files(d4_sources)
        .define("D4_SOLVER", "minisat")
        .define("D4_PREPORC_SOLVER", "minisat")
        .compile("d4");

    // FIXME: Currently, `gmp-mpfr-sys` does not build the C++ interface. Once it does, GMPXX could be used from it.
    // println!("cargo:rustc-link-search={}", env::var("DEP_GMP_OUT_DIR").expect("GMP library directory not passed."));

    // Find and link the C++ interface of GMP.
    let gmpxx_library = pkg_config::Config::new()
        .probe("gmpxx")
        .expect("Failed to find GMP C++ library.");

    for path in gmpxx_library.link_paths {
        println!("cargo:rustc-link-search={}", path.display());
    }

    println!("cargo:rustc-link-lib=dylib=gmpxx");

    // Link Mt-KaHyPar.
    println!("cargo:rustc-link-lib=dylib=mtkahypar");
}

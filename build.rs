use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {
    // Find and link GMP.
    let gmp_library = pkg_config::Config::new()
        .probe("gmp")
        .expect("Failed to find GMP library.");

    for path in gmp_library.link_paths {
        println!("cargo:rustc-link-search={}", path.display());
    }

    // Find d4 sources to build.
    let d4_sources: Vec<PathBuf> = glob("d4/src/**/*.cpp")
        .expect("Failed to create glob pattern for d4 sources.")
        .map(|result| result.expect("Failed to find d4 source file."))
        .filter(|path| !path.ends_with("Main.cpp"))
        .filter(|path| !path.ends_with("ConfigConverter.cpp"))
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

    println!("cargo:rustc-link-lib=dylib=gmp");

    // Link Mt-KaHyPar.
    println!("cargo:rustc-link-lib=dylib=mtkahypar");
}

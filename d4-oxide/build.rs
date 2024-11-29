use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {
    // Find GMP as the C++ bindings are not built by the corresponding crate yet.
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
        "d4".to_string(),
        "d4/3rdParty/glucose-3.0".to_string(),
        "d4/src".to_string(),
        env::var("DEP_ARJUN_INCLUDE").unwrap(),
        env::var("DEP_CRYPTOMINISAT5_INCLUDE").unwrap(),
        env::var("DEP_GPMC_INCLUDE").unwrap(),
        env::var("DEP_GMP_INCLUDE_DIR").unwrap(),
    ];

    // Collect include and lib dirs of Mt-KaHyPar.
    if let Ok(root) = env::var("MTKAHYPAR_ROOT") {
        includes.push(format!("{}/include", root.clone()));
        println!("cargo::rustc-link-search=native={root}/lib");
        println!("cargo::rustc-link-search=native={root}/lib64");
    }

    if let Ok(include_dir) = env::var("MTKAHYPAR_INCLUDE") {
        includes.push(include_dir.clone());
    }

    if let Ok(lib_dir) = env::var("MTKAHYPAR_LIB") {
        println!("cargo::rustc-link-search=native={lib_dir}");
    }

    // Build d4.
    cxx_build::bridge("src/lib.rs")
        .std("c++20")
        .includes(includes)
        .file("src/Adapter.cc")
        .files(d4_sources)
        .define("D4_SOLVER", "minisat")
        .define("D4_PREPORC_SOLVER", "minisat")
        .compile("d4");

    // Link Mt-KaHyPar.
    println!("cargo::rustc-link-lib=dylib=mtkahypar");

    // Link all other dependencies statically.
    println!("cargo::rustc-link-lib=static=arjun");
    println!("cargo::rustc-link-lib=static=cadiback");
    println!("cargo::rustc-link-lib=static=cadical");
    println!("cargo::rustc-link-lib=static=cryptominisat5");
    println!("cargo::rustc-link-lib=static=glucose");
    println!("cargo::rustc-link-lib=static=gmpxx");
    println!("cargo::rustc-link-lib=static=gpmc");
    println!("cargo::rustc-link-lib=static=sbva");
}

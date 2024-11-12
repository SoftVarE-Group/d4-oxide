use glob::glob;
use std::env;
use std::path::PathBuf;

static DYNAMIC_DEPENDENCIES: &'static [&str] = &["mtkahypar"];

static STATIC_DEPENDENCIES: &'static [&str] = &[
    "arjun",
    "cadiback",
    "cadical",
    "cryptominisat5",
    "glucose",
    "gmp",
    "gmpxx",
    "gpmc",
    "sbva",
];

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
        "d4".to_string(),
        "d4/3rdParty/glucose-3.0".to_string(),
        "d4/src".to_string(),
    ];

    // Collect include and lib dirs of dependencies.
    DYNAMIC_DEPENDENCIES
        .iter()
        .chain(STATIC_DEPENDENCIES.iter())
        .for_each(|dependency| {
            if let Ok(include_dir) = env::var(format!("{}_INCLUDE_DIR", dependency.to_uppercase()))
            {
                includes.push(include_dir.clone());
            }

            if let Ok(lib_dir) = env::var(format!("{}_LIB_DIR", dependency.to_uppercase()))
            {
                println!("cargo::rustc-link-search=native={lib_dir}");
            }
        });

    // Build d4.
    cxx_build::bridge("src/lib.rs")
        .std("c++20")
        .includes(includes)
        .file("src/Adapter.cc")
        .files(d4_sources)
        .define("D4_SOLVER", "minisat")
        .define("D4_PREPORC_SOLVER", "minisat")
        .compile("d4");

    // Link all dependencies.
    DYNAMIC_DEPENDENCIES
        .iter()
        .for_each(|dependency| println!("cargo::rustc-link-lib=dylib={dependency}"));

    STATIC_DEPENDENCIES
        .iter()
        .for_each(|dependency| println!("cargo::rustc-link-lib=static={dependency}"));
}

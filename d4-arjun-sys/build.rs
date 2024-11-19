use std::env;

fn main() {
    let build = cmake::Config::new("arjun")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("MPFR_ROOT", env::var("DEP_GMP_OUT_DIR").unwrap())
        .define(
            "CMAKE_PREFIX_PATH",
            env::var("DEP_CRYPTOMINISAT5_ROOT").unwrap(),
        )
        .build();

    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
    println!("cargo::rustc-link-search=native={}/lib64", build.display());
}

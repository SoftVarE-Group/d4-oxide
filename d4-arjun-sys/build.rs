use std::env;

fn main() {
    let build = cmake::Config::new("arjun")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("MPFR_ROOT", env::var("DEP_GMP_OUT_DIR").unwrap())
        .define(
            "cryptominisat5_DIR",
            env::var("DEP_CRYPTOMINISAT5_CMAKE").unwrap(),
        )
        .define("sbva_DIR", env::var("DEP_SBVA_CMAKE").unwrap())
        .build();

    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!("cargo::metadata=CMAKE={}/lib/cmake/arjun", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
    println!("cargo::rustc-link-search=native={}/lib64", build.display());
}

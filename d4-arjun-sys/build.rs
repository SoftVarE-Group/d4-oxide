use std::env;

fn main() {
    let build = cmake::Config::new("arjun")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BINARY", "OFF")
        .define(
            "cryptominisat5_DIR",
            env::var("DEP_CRYPTOMINISAT5_CMAKE").unwrap(),
        )
        .define("sbva_DIR", env::var("DEP_SBVA_CMAKE").unwrap())
        .cxxflag(format!("-isystem {}", env::var("DEP_GMP_INCLUDE").unwrap()))
        .build();

    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!("cargo::metadata=CMAKE={}/lib/cmake/arjun", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
    println!("cargo::rustc-link-search=native={}/lib64", build.display());
}

use std::env;

fn main() {
    let build = cmake::Config::new("d4/3rdParty/GPMC")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ZLIB_ROOT", env::var("DEP_Z_ROOT").unwrap())
        .define("arjun_DIR", env::var("DEP_ARJUN_CMAKE").unwrap())
        .define("CMAKE_PREFIX_PATH", env::var("DEP_GMP_OUT_DIR").unwrap())
        .cxxflag(format!(
            "-isystem {}",
            env::var("DEP_ARJUN_INCLUDE").unwrap()
        ))
        .cxxflag(format!(
            "-isystem {}",
            env::var("DEP_CRYPTOMINISAT5_INCLUDE").unwrap()
        ))
        .cxxflag(format!(
            "-isystem {}",
            env::var("DEP_GMP_INCLUDE_DIR").unwrap()
        ))
        .build();

    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
    println!("cargo::rustc-link-search=native={}/lib64", build.display());
}

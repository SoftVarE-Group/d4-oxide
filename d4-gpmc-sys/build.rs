use std::env;

fn main() {
    let build = cmake::Config::new("d4/3rdParty/GPMC")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ZLIB_ROOT", env::var("DEP_Z_ROOT").unwrap())
        .define("arjun_DIR", env::var("DEP_ARJUN_CMAKE").unwrap())
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
            env::var("DEP_MPFR_INCLUDE").unwrap()
        ))
        .build();

    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
    println!("cargo::rustc-link-search=native={}/lib64", build.display());
}

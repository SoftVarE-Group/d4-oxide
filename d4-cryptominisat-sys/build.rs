use std::env;

fn main() {
    let build = cmake::Config::new("cryptominisat")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BINARY", "OFF")
        .env(
            "CADIBACK_LIB_DIR",
            format!("{}/lib", env::var("DEP_CADIBACK_ROOT").unwrap()),
        )
        .env(
            "CADICAL_LIB_DIR",
            format!("{}/lib", env::var("DEP_CADICAL_ROOT").unwrap()),
        )
        .define(
            "CMAKE_INCLUDE_PATH",
            env::var("DEP_CADIBACK_INCLUDE").unwrap(),
        )
        .define(
            "CMAKE_INCLUDE_PATH",
            env::var("DEP_CADICAL_INCLUDE").unwrap(),
        )
        .cxxflag(format!(
            "-isystem {}",
            env::var("DEP_CADIBACK_INCLUDE").unwrap()
        ))
        .cxxflag(format!(
            "-isystem {}",
            env::var("DEP_CADICAL_INCLUDE").unwrap()
        ))
        .build();

    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!(
        "cargo::metadata=CMAKE={}/lib/cmake/cryptominisat5",
        build.display()
    );
    println!("cargo::rustc-link-search=native={}/lib", build.display());
    println!("cargo::rustc-link-search=native={}/lib64", build.display());
}

use std::env;

fn main() {
    let build = cmake::Config::new("cadiback")
        .define("CMAKE_PREFIX_PATH", env::var("DEP_CADICAL_ROOT").unwrap())
        .define(
            "CMAKE_INCLUDE_PATH",
            env::var("DEP_CADICAL_INCLUDE").unwrap(),
        )
        .cxxflag(format!(
            "-isystem {}",
            env::var("DEP_CADICAL_INCLUDE").unwrap()
        ))
        .build();

    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

use std::env;

fn main() {
    let build = cmake::Config::new("d4/3rdParty/glucose-3.0")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ZLIB_ROOT", env::var("DEP_Z_ROOT").unwrap())
        .build();

    println!("cargo:include={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

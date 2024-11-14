fn main() {
    let build = cmake::Config::new("sbva")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BINARY", "OFF")
        .build();

    println!("cargo:include={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
    println!("cargo::rustc-link-search=native={}/lib64", build.display());
}
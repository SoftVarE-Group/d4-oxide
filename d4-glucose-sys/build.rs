fn main() {
    let build = cmake::Config::new("glucose")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!("cargo:include={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

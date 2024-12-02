use autotools::Config;
use std::env;

fn main() {
    let build = Config::new("mpfr")
        .with("gmp", Some(&env::var("DEP_GMP_ROOT").unwrap()))
        .build();

    println!("cargo:include={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

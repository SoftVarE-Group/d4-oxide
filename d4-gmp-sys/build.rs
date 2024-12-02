use autotools::Config;

fn main() {
    let build = Config::new("gmp").enable("cxx", None).build();
    println!("cargo:include={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

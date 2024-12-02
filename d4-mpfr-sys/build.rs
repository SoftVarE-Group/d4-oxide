fn main() {
    let build = autotools::build("mpfr");
    println!("cargo:include={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

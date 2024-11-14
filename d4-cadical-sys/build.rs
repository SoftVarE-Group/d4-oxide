fn main() {
    let build = cmake::build("cadical");
    println!("cargo:include={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

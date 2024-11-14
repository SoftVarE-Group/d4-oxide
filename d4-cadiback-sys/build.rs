fn main() {
    let build = cmake::build("cadiback");
    println!("cargo::metadata=INCLUDE={}/include", build.display());
    println!("cargo::rustc-link-search=native={}/lib", build.display());
}

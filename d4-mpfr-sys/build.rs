fn main() {
    let gmp = pkg_config::probe_library("mpfr").unwrap();

    for path in gmp.include_paths {
        println!("cargo::metadata=INCLUDE={}", path.display());
    }

    for path in gmp.link_paths {
        println!("cargo::rustc-link-search=native={}", path.display());
    }
}

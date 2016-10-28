To compile with openssl and libz on windows you need to add a build.rs file in your project.

``` rust
fn main() {
    if cfg!(any(windows)) {
        println!("cargo:rustc-link-search=native=C:/OpenSSL-Win64/lib");
        println!("cargo:rustc-link-search=native=C:/Program Files/zlib/lib");
        println!("cargo:rustc-link-lib=static=ssleay32");
        println!("cargo:rustc-link-lib=static=libeay32");
        println!("cargo:rustc-link-lib=static=zlibd")
    }
}
```

add to your Cargo.toml
``` rust
[package]
build = "build.rs"
```

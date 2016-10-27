extern crate cmake;
extern crate git2 as git;


fn main() {
    let dir = std::env::var("OUT_DIR").unwrap() + "/librtmp";
    let url = "https://github.com/Stargateur/librtmp";
    let _ = git::Repository::clone(url, &dir);
    let dst = cmake::Config::new(&dir).build_target("rtmp").build();

    if cfg!(any(windows)) {
        println!("cargo:rustc-link-search=native={}/build/Release", dst.display())
    }
    else {
        println!("cargo:rustc-link-search=native={}/build", dst.display())
    }
    println!("cargo:rustc-link-lib=static=rtmp")
}

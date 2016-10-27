extern crate cmake;

fn main() {
    let dst = cmake::Config::new("librtmp").build_target("rtmp").build();

    if cfg!(any(windows)) {
        println!("cargo:rustc-link-search=native={}/build/Release", dst.display());
    }
    else {
        println!("cargo:rustc-link-search=native={}/build", dst.display());
    }
    println!("cargo:rustc-link-lib=static=rtmp");
}

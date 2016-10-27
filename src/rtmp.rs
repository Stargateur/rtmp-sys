use libc;

extern "C" {
    pub fn RTMP_LibVersion() -> libc::c_int;
}

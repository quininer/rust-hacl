/* automatically generated by rust-bindgen */

pub type __uint8_t = ::libc::c_uchar;
pub type __uint32_t = ::libc::c_uint;
pub type Hacl_Ed25519_uint8_p = *mut u8;
pub type Hacl_Ed25519_hint8_p = *mut u8;
extern "C" {
    pub fn Hacl_Ed25519_sign(signature: *mut u8, secret: *mut u8, msg: *mut u8, len1: u32);
}
extern "C" {
    pub fn Hacl_Ed25519_verify(
        public: *mut u8,
        msg: *mut u8,
        len1: u32,
        signature: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_Ed25519_secret_to_public(out: *mut u8, secret: *mut u8);
}

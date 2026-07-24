#![feature(custom_inner_attributes)]

mod a;

pub use a::as_u16;

#[test]
fn checked_truncation() {
    let _ = as_u16(65536);
}

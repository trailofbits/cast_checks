#![allow(clippy::cast_possible_truncation)]
#![allow(dead_code)]

#[test]
fn unchecked() {
    let x: u64 = 256;
    let _: u8 = x as u8;
}

#[test]
#[cast_checks::enable]
fn checked_ok() {
    let x: u64 = 255;
    let _: u8 = x as u8;
}

#[test]
#[cast_checks::enable]
#[cfg_attr(not(feature = "__no_should_panic"), should_panic)]
#[allow(clippy::cast_sign_loss)]
fn checked_sign_loss() {
    let y: i8 = -1;
    let _ = y as u128; // will return 18446744073709551615
}

#[test]
#[cast_checks::enable]
#[cfg_attr(not(feature = "__no_should_panic"), should_panic)]
fn checked_truncation() {
    let x: u64 = 256;
    let _ = x as u8;
}

#[test]
#[cast_checks::enable]
#[cfg_attr(not(feature = "__no_should_panic"), should_panic)]
fn checked_unsafe() {
    static mut X: u8 = 0;
    let x: u64 = 256;
    unsafe {
        X = x as u8;
    }
}

#[test]
#[cast_checks::enable]
#[cfg_attr(not(feature = "__no_should_panic"), should_panic)]
#[allow(clippy::cast_possible_wrap)]
fn checked_wrap() {
    let _ = u32::MAX as i32; // will yield a value of `-1`
}

#[test]
#[cast_checks::enable]
fn checked_pointer() {
    let a = 300 as *const char; // `a` is a pointer to location 300.
    let _ = a as u32;
}

#[test]
#[cast_checks::enable]
fn checked_float() {
    let _ = 1f32 as i32;
    let _ = 1f64 as i32;
}

#[cast_checks::enable]
fn inferred_type(x: f64) -> i32 {
    x as _
}

#[allow(clippy::cast_lossless)]
#[cast_checks::enable]
fn block_expression(x: u32) -> f64 {
    x as f64 / 2.0
}

#[allow(arithmetic_overflow, clippy::cast_lossless, clippy::neg_multiply)]
#[cast_checks::enable]
#[deny(unused_parens, clippy::double_parens)]
fn unused_parens(use_parens: bool) -> i64 {
    if use_parens {
        (-1 * i32::MIN) as i64
    } else {
        -1 * i32::MIN as i64
    }
}

#[cast_checks::enable]
fn boxed_trait_object() -> Box<dyn std::error::Error> {
    let error = String::new();
    Box::new(std::io::Error::new(std::io::ErrorKind::Other, error)) as Box<dyn std::error::Error>
}

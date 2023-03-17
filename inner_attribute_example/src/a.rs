#![cast_checks::enable]

pub use b::as_u16;

mod b {
    pub fn as_u16(x: u64) -> u16 {
        x as u16
    }
}

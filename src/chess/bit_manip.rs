#![allow(unused_assignments)]

pub fn set_bit(n: u64, bit: u8) -> u64 {
    n | (1 << bit)
}

pub fn clear_bit(n: u64, bit: u8) -> u64 {
    n & !(1 << bit)
}

pub fn get_bit(n: u64, bit: u8) -> bool {
    ((n & (1 << bit)) >> bit) != 0
}

use std::ops::{Add, Mul};

pub trait Unsigned<T>: Copy + From<u8> + Add<Output = T> + Mul<Output = T> {
    const TEN: T;
}

impl Unsigned<u32> for u32 {
    const TEN: u32 = 10;
}

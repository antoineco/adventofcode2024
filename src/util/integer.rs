use std::ops::{Add, Mul};

pub trait Unsigned<T>: Copy + From<u8> + Add<Output = T> + Mul<Output = T> {
    const TEN: T;
}

macro_rules! unsigned {
    ($($t:ty)*) => ($(
        impl Unsigned<$t> for $t {
            const TEN: $t = 10;
        }
    )*)
}

unsigned!(u32 u64);

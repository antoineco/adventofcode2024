use crate::util::integer::Unsigned;
use std::marker::PhantomData;
use std::str::Bytes;

pub trait ParseByte {
    fn to_decimal(self) -> u8;
}

impl ParseByte for u8 {
    #[inline]
    fn to_decimal(self) -> u8 {
        self.wrapping_sub(b'0')
    }
}

pub struct ParseUnsigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<T>,
}

pub trait ParseOps {
    fn unsigned<T: Unsigned<T>>(&self) -> T;
    fn iter_unsigned<T: Unsigned<T>>(&self) -> ParseUnsigned<'_, T>;
}

impl ParseOps for &str {
    fn unsigned<T: Unsigned<T>>(&self) -> T {
        match try_unsigned(&mut self.bytes()) {
            Some(t) => t,
            None => panic!("Unable to parse \"{self}\""),
        }
    }

    fn iter_unsigned<T: Unsigned<T>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned {
            bytes: self.bytes(),
            phantom: PhantomData,
        }
    }
}

impl<T: Unsigned<T>> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.bytes.size_hint();
        (lower / 3, upper.map(|u| u / 3))
    }

    fn next(&mut self) -> Option<Self::Item> {
        try_unsigned(&mut self.bytes)
    }
}

fn try_unsigned<T: Unsigned<T>>(bytes: &mut Bytes<'_>) -> Option<T> {
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = byte.to_decimal();

        if digit < 10 {
            break T::from(digit);
        }
    };

    loop {
        let Some(byte) = bytes.next() else {
            break Some(n);
        };
        let digit = byte.to_decimal();

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break Some(n);
        }
    }
}

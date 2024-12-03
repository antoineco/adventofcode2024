use std::str::Bytes;

pub fn parse(input: &str) -> Vec<Mul> {
    input.iter_mul().collect()
}

pub fn part1(muls: &[Mul]) -> u32 {
    muls.iter().map(|m| m.0 * m.1).sum()
}

pub fn part2(_: &[Mul]) -> u32 {
    0
}

pub struct Mul(u32, u32);

impl From<Vec<u32>> for Mul {
    fn from(value: Vec<u32>) -> Self {
        Self(*value.first().unwrap(), *value.last().unwrap())
    }
}

trait ParseOps {
    fn iter_mul(&self) -> ParseMul;
}

impl ParseOps for &str {
    fn iter_mul(&self) -> ParseMul {
        ParseMul {
            bytes: self.bytes(),
        }
    }
}

struct ParseMul<'a> {
    bytes: Bytes<'a>,
}

impl Iterator for ParseMul<'_> {
    type Item = Mul;

    fn next(&mut self) -> Option<Self::Item> {
        try_mul(&mut self.bytes)
    }
}

fn try_mul(bytes: &mut Bytes) -> Option<Mul> {
    let mut last_three = [0_u8; 3];
    loop {
        let byte = bytes.next()?;
        // reached "mul("
        if byte == b'(' && last_three.iter().eq([b'm', b'u', b'l'].iter()) {
            (last_three[0], last_three[1], last_three[2]) = (last_three[1], last_three[2], byte);
            let mut numbers: Vec<u32> = Vec::with_capacity(2);
            let mut digits = String::with_capacity(3);
            loop {
                let byte = bytes.next()?;
                (last_three[0], last_three[1], last_three[2]) = (last_three[1], last_three[2], byte);
                match byte {
                    d @ b'0'..=b'9' => {
                        digits.push(d.into());
                    }
                    b',' => {
                        // reached "mul(nnn,"
                        if numbers.is_empty() {
                            // safe because input does not contain any "mul(,"
                            numbers.push(digits.parse().unwrap());
                            digits.clear();
                        // reached "mul(nnn,nnn,"
                        } else {
                            break;
                        }
                    }
                    b')' => {
                        // reached "mul(nnn,nnn)", but not "mul(nnn)"
                        if !numbers.is_empty() {
                            // safe because input does not contain any "mul(nnn,)"
                            numbers.push(digits.parse().unwrap());
                        }
                        break;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if numbers.len() == 2 {
                return Some(numbers.into());
            }
            continue;
        }
        (last_three[0], last_three[1], last_three[2]) = (last_three[1], last_three[2], byte);
    }
}

#[test]
fn sample_input() {
    let input = "\
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n\
        ";
    let res = parse(input);
    assert_eq!(part1(&res), 161);
}

#[test]
fn mul_in_mul() {
    let input = "\
        xmul(2,mul(8,5)\n\
        xmul(mul(8,5)\n\
        ";
    let res = parse(input);
    assert_eq!(part1(&res), 80);
}

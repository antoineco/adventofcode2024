use std::str::Bytes;

pub fn parse(input: &str) -> Vec<Mul> {
    input.iter_mul().collect()
}

pub fn part1(muls: &[Mul]) -> u32 {
    muls.iter().map(|m| m.0 * m.1).sum()
}

pub fn part2(muls: &[Mul]) -> u32 {
    let mut r#do = true;
    muls.iter()
        .filter_map(|m| match m {
            // Rogue values
            Mul(0, b) => {
                r#do = *b == 1;
                None
            }
            Mul(a, b) if r#do => Some(a * b),
            _ => None,
        })
        .sum()
}

pub struct Mul(u32, u32);

// Rogue values.
// 0 product to avoid skewing results.
const DO: Mul = Mul(0, 1);
const DONT: Mul = Mul(0, 0);

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
        // reached "do()"
        if byte == b')' && last_three == [b'd', b'o', b'('] {
            return Some(DO);
        }
        // reached "'t()" - for "don't()"
        // Hacky but works on the puzzle's input.
        else if byte == b')' && last_three == [b'\'', b't', b'('] {
            return Some(DONT);
        }
        // reached "mul("
        else if byte == b'(' && last_three == [b'm', b'u', b'l'] {
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
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n\
        ";
    let res = parse(input);
    assert_eq!(part1(&res), 161);
    assert_eq!(part2(&res), 48);
}

#[test]
fn mul_in_mul() {
    let input = "\
        xmul(2,mul(8,5)\n\
        xmul(mul(8,5)\n\
        ";
    let res = parse(input);
    assert_eq!(part1(&res), 80);
    assert_eq!(part2(&res), 80);
}

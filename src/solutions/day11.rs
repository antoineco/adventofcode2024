use crate::util::parse::ParseOps;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(stones: &[u64]) -> u64 {
    const BLINKS: u8 = 25;
    stones
        .iter()
        .map(|&stone| {
            let mut total = 0;

            let mut stack = VecDeque::new();
            stack.push_back((stone, 0));

            while let Some((stone, blink)) = stack.pop_back() {
                if blink < BLINKS {
                    if stone == 0 {
                        stack.push_back((1, blink + 1))
                    } else {
                        let d = num_digits(&stone);
                        if d % 2 == 0 {
                            let div = 10_u64.pow(d / 2);
                            stack.push_back(((stone / div), blink + 1));
                            stack.push_back(((stone % div), blink + 1));
                        } else {
                            stack.push_back(((stone * 2024), blink + 1))
                        }
                    }
                } else {
                    total += 1;
                }
            }

            total
        })
        .sum()
}

pub fn part2(_: &[u64]) -> usize {
    0
}

fn num_digits(n: &u64) -> u32 {
    n.ilog10() + 1
}

#[test]
fn sample_input_1() {
    let input = "\
        125 17\n\
        ";
    let stones = parse(input);
    assert_eq!(part1(&stones), 55312);
}

#[test]
fn sample_input_2() {
    let input = "\
        0 1 10 99 999\n\
        ";
    let stones = parse(input);
    assert_eq!(part1(&stones), 125681);
}

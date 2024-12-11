use crate::util::parse::ParseOps;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(stones: &[u64]) -> usize {
    let mut q: VecDeque<_> = stones.iter().cloned().collect();
    for _ in 0..25 {
        for _ in 0..q.len() {
            match q.pop_front().unwrap() {
                0 => vec![1],
                s if num_digits(&s) % 2 == 0 => {
                    let div = 10_u64.pow(num_digits(&s) / 2);
                    vec![s / div, s % div]
                }
                s => vec![s * 2024],
            }
            .iter()
            .for_each(|s| q.push_back(*s));
        }
    }
    q.len()
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

use crate::util::parse::ParseOps;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.iter_unsigned();
            (iter.next().unwrap(), iter.collect())
        })
        .collect()
}

pub fn part1(eqs: &[(u64, Vec<u64>)]) -> u64 {
    eqs.par_iter()
        .map(|eq| {
            let (tval, numbers) = eq;
            let mut summaries =
                VecDeque::with_capacity(2_u32.pow(numbers.len() as u32 - 1) as usize);
            let mut iter = numbers.iter();
            summaries.push_back(*iter.next().unwrap());
            iter.for_each(|n| {
                for _ in 0..summaries.len() {
                    let s = summaries.pop_front().unwrap();
                    summaries.push_back(s + n);
                    summaries.push_back(s * n);
                }
            });
            if summaries.contains(tval) {
                *tval
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(_: &[(u64, Vec<u64>)]) -> u64 {
    0
}

#[test]
fn sample_input() {
    let input = "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20\n\
        ";
    let eqs = parse(input);
    assert_eq!(part1(&eqs), 3749);
}

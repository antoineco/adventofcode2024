use crate::util::parse::ParseOps;
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
    eqs.iter()
        .map(|eq| {
            let (tval, numbers) = eq;
            let mut summaries = VecDeque::new();
            summaries.push_back(*tval);
            for n in numbers[1..].iter().rev() {
                // No need to break early if summaries is empty, this case never occurs on the
                // given input.
                for _ in 0..summaries.len() {
                    let s = summaries.pop_front().unwrap();
                    if *n < s {
                        summaries.push_back(s - n);
                    }
                    if s % n == 0 {
                        summaries.push_back(s / n);
                    }
                }
            }
            if summaries.contains(&numbers[0]) {
                *tval
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(eqs: &[(u64, Vec<u64>)]) -> u64 {
    eqs.iter()
        .map(|eq| {
            let (tval, numbers) = eq;
            let mut summaries = VecDeque::new();
            summaries.push_back(*tval);
            for n in numbers[1..].iter().rev() {
                // No need to break early if summaries is empty, this case never occurs on the
                // given input.
                for _ in 0..summaries.len() {
                    let s = summaries.pop_front().unwrap();
                    if *n < s {
                        summaries.push_back(s - n);
                    }
                    if s % n == 0 {
                        summaries.push_back(s / n);
                    }
                    let div = 10_u64.pow(num_digits(n));
                    if s % div == *n {
                        summaries.push_back(s / div);
                    }
                }
            }
            if summaries.contains(&numbers[0]) {
                *tval
            } else {
                0
            }
        })
        .sum()
}

fn num_digits(n: &u64) -> u32 {
    n.ilog10() + 1
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
    assert_eq!(part2(&eqs), 11387);
}

#[test]
fn extract_from_real_input() {
    let input = "\
        127536599: 49 4 21 65 99\n\
        7: 15 15 15\n\
        ";
    let eqs = parse(input);
    assert_eq!(part1(&eqs), 0);
    /* 49 * 4 || 21 x 65 || 99
     *    196 || 21 x 65 || 99
     *        19621 x 65 || 99
     *           1275365 || 99
     *               127536599
     */
    assert_eq!(part2(&eqs), 127536599);
}

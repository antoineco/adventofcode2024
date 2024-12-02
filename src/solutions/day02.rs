use crate::util::parse::ParseOps;
use std::cmp::Ordering::{Equal, Greater, Less};

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut reports = Vec::new();
    input
        .lines()
        .for_each(|l| reports.push(l.iter_unsigned().collect()));
    reports
}

pub fn part1(reports: &[Vec<u32>]) -> u32 {
    reports
        .iter()
        .filter(|levels| {
            let mut lvls_iter = levels.iter().peekable();
            let first_lvl = lvls_iter.next().unwrap();
            let second_lvl = lvls_iter.peek().unwrap();
            if first_lvl.abs_diff(**second_lvl) > 3 {
                return false;
            }
            let is_sorted = match first_lvl.cmp(second_lvl) {
                Less => |a, b| a < b && b - a <= 3,
                Greater => |a, b| b < a && a - b <= 3,
                Equal => |_: &u32, _: &u32| false,
            };
            lvls_iter.is_sorted_by(|a: &&u32, b: &&u32| is_sorted(a, b))
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(_: &[Vec<u32>]) -> u32 {
    0
}

#[test]
fn sample_input() {
    let input = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 2);
}

#[test]
fn first_two_levels_are_equal() {
    let input = "\
        3 3 6 7 9\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 0);
}

#[test]
fn first_two_levels_are_far_apart() {
    let input = "\
        1 5 6 7 9\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 0);
}

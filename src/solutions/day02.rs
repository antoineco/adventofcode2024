use crate::util::parse::ParseOps;
use std::cmp::Ordering;
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
            let is_sorted = is_sorted_fn(first_lvl.cmp(second_lvl));
            lvls_iter.is_sorted_by(|a: &&u32, b: &&u32| is_sorted(a, b))
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(reports: &[Vec<u32>]) -> u32 {
    reports
        .iter()
        .filter(|levels| {
            let direction = direction(levels[..3].try_into().unwrap(), *levels.last().unwrap());
            if direction == Equal {
                return false;
            };

            let is_sorted = is_sorted_fn(direction);
            if levels
                .iter()
                .is_sorted_by(|a: &&u32, b: &&u32| is_sorted(a, b))
            {
                return true;
            }

            let mut levels = (*levels).clone();
            for i in 0..levels.len() {
                let l = levels.remove(i);
                if levels
                    .iter()
                    .is_sorted_by(|a: &&u32, b: &&u32| is_sorted(a, b))
                {
                    return true;
                }
                levels.insert(i, l);
            }
            false
        })
        .count()
        .try_into()
        .unwrap()
}

fn is_sorted_fn(ord: Ordering) -> fn(a: &u32, b: &u32) -> bool {
    match ord {
        Less => |a, b| a < b && b - a <= 3,
        Greater => |a, b| b < a && a - b <= 3,
        Equal => |_, _| false,
    }
}

fn direction(beginning: [u32; 3], end: u32) -> Ordering {
    // Possible scenarios:
    // - "|3 2 1| ... 1" is Greater (decreasing)
    // - "|1 2 3| ... 3" is Less (increasing)
    // - "|2 3 4| ... 3" is Equal, which is a good indicator that the series has multiple direction
    //   changes and therefore isn't "safe" in the first place.
    beginning
        .iter()
        .map(|lvl| match lvl.cmp(&end) {
            Less => -1,
            Equal => 0,
            Greater => 1,
        })
        .sum::<i32>()
        .cmp(&0)
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
    assert_eq!(part2(&locs), 4);
}

#[test]
fn first_two_levels_are_equal() {
    let input = "\
        3 3 6 7 9\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 0);
    assert_eq!(part2(&locs), 1);
}

#[test]
fn first_two_levels_are_far_apart() {
    let input = "\
        1 5 6 7 9\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 0);
    assert_eq!(part2(&locs), 1);
}

#[test]
fn increases_direction_changes_after_first_level() {
    let input = "\
        9 3 6 7 8\n\
        1 7 5 3 2\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 0);
    assert_eq!(part2(&locs), 2);
}

#[test]
fn increase_direction_changes_after_second_level() {
    let input = "\
        6 8 4 3 1\n\
        4 2 6 7 9\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 0);
    assert_eq!(part2(&locs), 2);
}

#[test]
fn increase_direction_unclear() {
    let input = "\
        3 1 2 2\n\
        1 2 3 2\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 0);
    assert_eq!(part2(&locs), 0);
}

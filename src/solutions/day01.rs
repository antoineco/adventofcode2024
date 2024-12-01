use crate::util::parse::ParseOps;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Locations {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Locations {
    fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }
}

pub fn parse(input: &str) -> Locations {
    let mut locs = Locations::new();
    input.lines().for_each(|l| {
        let mut loc_iter = l.iter_unsigned();
        locs.left.push(loc_iter.next().unwrap());
        locs.right.push(loc_iter.next().unwrap());
    });
    locs
}

pub fn part1(locs: &Locations) -> u32 {
    let mut locs = locs.clone();
    locs.sort();
    locs.left
        .into_iter()
        .zip(locs.right)
        .map(|lr| lr.0.abs_diff(lr.1))
        .sum()
}

pub fn part2(locs: &Locations) -> u32 {
    let mut right_cnt = HashMap::new();
    locs.right
        .iter()
        .for_each(|l| *right_cnt.entry(l).or_insert(0u32) += 1);
    locs.left
        .iter()
        .filter_map(|l| right_cnt.get(l).map(|cnt| l * cnt))
        .sum()
}

#[test]
fn sample_input() {
    let input = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3\n\
        ";
    let locs = parse(input);
    assert_eq!(part1(&locs), 11);
    assert_eq!(part2(&locs), 31);
}

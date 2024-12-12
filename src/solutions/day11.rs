use crate::util::parse::ParseOps;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn parse(input: &str) -> Vec<u64> {
    input.iter_unsigned().collect()
}

pub fn part1(stones: &[u64]) -> u64 {
    let mut cache = Cache::new();
    stones
        .iter()
        .map(|stone| count(*stone, 25, &mut cache))
        .sum()
}

pub fn part2(stones: &[u64]) -> u64 {
    let mut cache = Cache::new();
    stones
        .iter()
        .map(|stone| count(*stone, 75, &mut cache))
        .sum()
}

type Cache = HashMap<(u64, u8), u64>; // (stone, blinks): count

fn count(source_stone: u64, total_blinks: u8, cache: &mut Cache) -> u64 {
    let mut stack = VecDeque::new();

    let key_source = (source_stone, 0);
    stack.push_back(key_source);

    while let Some(key) = stack.pop_back() {
        if cache.contains_key(&key) {
            continue;
        }

        let (stone, blinks) = key;
        if blinks == total_blinks {
            cache.insert(key, 1);
            continue;
        }

        if stone == 0 {
            let key_next = (1, blinks + 1);
            if let Some(count) = cache.get(&key_next) {
                cache.insert(key, *count);
            } else {
                stack.push_back(key);
                stack.push_back(key_next);
            }
        } else {
            let d = num_digits(&stone);
            if d % 2 == 0 {
                let div = 10_u64.pow(d / 2);
                let key_next1 = (stone / div, blinks + 1);
                let key_next2 = (stone % div, blinks + 1);

                let count_opt1 = cache.get(&key_next1);
                let count_opt2 = cache.get(&key_next2);
                if let (Some(count1), Some(count2)) = (count_opt1, count_opt2) {
                    cache.insert(key, count1 + count2);
                } else {
                    stack.push_back(key);
                    if count_opt1.is_none() {
                        stack.push_back(key_next1);
                    }
                    if count_opt2.is_none() {
                        stack.push_back(key_next2);
                    }
                }
            } else {
                let key_next = (stone * 2024, blinks + 1);
                if let Some(count) = cache.get(&key_next) {
                    cache.insert(key, *count);
                } else {
                    stack.push_back(key);
                    stack.push_back(key_next);
                }
            }
        }
    }

    *cache.get(&key_source).unwrap()
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
    assert_eq!(part2(&stones), 65_601_038_650_482);
}

#[test]
fn sample_input_2() {
    let input = "\
        0 1 10 99 999\n\
        ";
    let stones = parse(input);
    assert_eq!(part1(&stones), 125681);
    assert_eq!(part2(&stones), 149_161_030_616_311);
}

#[test]
fn cache_check() {
    let input = "\
        0 0 0 0 0 0 0 0 0 0\n\
        ";
    let stones = parse(input);
    assert_eq!(part1(&stones), 19778 * 10);
    assert_eq!(part2(&stones), 22938365706844 * 10);
}

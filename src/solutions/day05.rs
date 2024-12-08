use crate::util::parse::ParseOps;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> (Updates, OrderingRules) {
    let (section_one, section_two) = input.split_once("\n\n").unwrap();

    let (mut updates, mut ordering_rules) = (Updates::new(), OrderingRules::new());

    for line in section_one.lines() {
        let mut pages_iter = line.iter_unsigned();
        let (before, after) = (pages_iter.next().unwrap(), pages_iter.next().unwrap());
        ordering_rules.entry(before).or_default().insert(after);
    }

    for l in section_two.lines() {
        updates.push(l.iter_unsigned().collect());
    }

    (updates, ordering_rules)
}

pub fn part1(sections: &(Updates, OrderingRules)) -> u32 {
    let (updates, ordering_rules) = sections;
    updates
        .iter()
        .filter_map(|upd| {
            // We cannot pre-sort the pages from the input and simply compare upd to (pages ∪ upd),
            // because the ordering rules contain a loop by design, so sort_by() panics ("sort
            // comparison does not implement a total order"). By chance, the provided updates
            // purposely avoid combinations of pages which would contain multiple of the pages
            // involved in the loop, so sorting here works.
            //
            // See test 'contains_loop' for an input which allows reproducing the panic.
            if ordering_rules.sorted(upd) {
                Some(upd[upd.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(sections: &(Updates, OrderingRules)) -> u32 {
    let (updates, ordering_rules) = sections;
    updates
        .iter()
        .filter_map(|upd| {
            if !ordering_rules.sorted(upd) {
                Some(ordering_rules.sort(upd)[upd.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

type Updates = Vec<Vec<u32>>;

pub struct OrderingRules {
    r: HashMap<u32, HashSet<u32>>,
}

impl OrderingRules {
    fn new() -> Self {
        Self { r: HashMap::new() }
    }

    fn entry(&mut self, key: u32) -> Entry<u32, HashSet<u32>> {
        self.r.entry(key)
    }

    fn sorted(&self, pages: &[u32]) -> bool {
        pages.is_sorted_by(|a, b| self.r.get(a).is_some_and(|rules| rules.contains(b)))
    }

    fn sort(&self, pages: &[u32]) -> Vec<u32> {
        let mut pages_ordered = pages.to_owned();
        pages_ordered.sort_by(|a, b| {
            if self.r.get(a).is_some_and(|rules| rules.contains(b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        pages_ordered
    }
}

#[test]
fn sample_input() {
    let input = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n\
        ";
    let out = parse(input);
    assert_eq!(part1(&out), 143);
    assert_eq!(part2(&out), 123);
}

#[test]
// Fails about 90% of the time when the sorting of pages occurs inside parse(), on the complete set
// of pages.
fn contains_loop() {
    let input = "\
        11|22\n\
        11|29\n\
        11|31\n\
        11|33\n\
        11|34\n\
        11|38\n\
        11|41\n\
        11|43\n\
        11|49\n\
        11|53\n\
        11|55\n\
        11|56\n\
        11|59\n\
        14|11\n\
        14|22\n\
        14|29\n\
        14|31\n\
        14|33\n\
        14|34\n\
        14|38\n\
        14|41\n\
        14|43\n\
        14|49\n\
        14|53\n\
        14|55\n\
        14|56\n\
        14|59\n\
        19|11\n\
        19|14\n\
        19|23\n\
        19|24\n\
        19|35\n\
        19|36\n\
        19|37\n\
        19|39\n\
        19|42\n\
        19|45\n\
        19|47\n\
        19|57\n\
        22|19\n\
        22|23\n\
        22|24\n\
        22|35\n\
        22|36\n\
        22|37\n\
        22|39\n\
        22|42\n\
        22|45\n\
        22|47\n\
        22|57\n\
        23|11\n\
        23|14\n\
        23|24\n\
        23|35\n\
        23|36\n\
        23|37\n\
        23|39\n\
        23|42\n\
        23|45\n\
        23|47\n\
        23|57\n\
        24|11\n\
        24|14\n\
        24|29\n\
        24|34\n\
        24|35\n\
        24|36\n\
        24|38\n\
        24|39\n\
        24|42\n\
        24|43\n\
        24|49\n\
        24|53\n\
        24|57\n\
        29|19\n\
        29|22\n\
        29|23\n\
        29|31\n\
        29|33\n\
        29|41\n\
        29|45\n\
        29|49\n\
        29|53\n\
        29|55\n\
        29|56\n\
        29|59\n\
        31|19\n\
        31|22\n\
        31|23\n\
        31|24\n\
        31|37\n\
        31|41\n\
        31|45\n\
        31|47\n\
        31|55\n\
        31|59\n\
        33|19\n\
        33|22\n\
        33|23\n\
        33|24\n\
        33|31\n\
        33|37\n\
        33|41\n\
        33|45\n\
        33|47\n\
        33|55\n\
        33|59\n\
        34|19\n\
        34|22\n\
        34|23\n\
        34|29\n\
        34|31\n\
        34|33\n\
        34|38\n\
        34|41\n\
        34|43\n\
        34|49\n\
        34|53\n\
        34|55\n\
        34|56\n\
        34|59\n\
        35|11\n\
        35|14\n\
        35|29\n\
        35|31\n\
        35|33\n\
        35|34\n\
        35|36\n\
        35|38\n\
        35|39\n\
        35|42\n\
        35|43\n\
        35|49\n\
        35|53\n\
        35|56\n\
        35|57\n\
        36|11\n\
        36|14\n\
        36|29\n\
        36|31\n\
        36|33\n\
        36|34\n\
        36|38\n\
        36|39\n\
        36|42\n\
        36|43\n\
        36|49\n\
        36|53\n\
        36|55\n\
        36|56\n\
        36|57\n\
        37|11\n\
        37|14\n\
        37|24\n\
        37|29\n\
        37|34\n\
        37|35\n\
        37|36\n\
        37|38\n\
        37|39\n\
        37|42\n\
        37|43\n\
        37|49\n\
        37|57\n\
        38|19\n\
        38|22\n\
        38|23\n\
        38|29\n\
        38|31\n\
        38|33\n\
        38|41\n\
        38|43\n\
        38|49\n\
        38|53\n\
        38|55\n\
        38|56\n\
        38|59\n\
        39|11\n\
        39|14\n\
        39|29\n\
        39|31\n\
        39|33\n\
        39|34\n\
        39|38\n\
        39|42\n\
        39|43\n\
        39|49\n\
        39|53\n\
        39|55\n\
        39|56\n\
        39|57\n\
        39|59\n\
        41|19\n\
        41|22\n\
        41|23\n\
        41|24\n\
        41|35\n\
        41|36\n\
        41|37\n\
        41|39\n\
        41|45\n\
        41|47\n\
        42|11\n\
        42|14\n\
        42|29\n\
        42|31\n\
        42|33\n\
        42|34\n\
        42|38\n\
        42|41\n\
        42|43\n\
        42|49\n\
        42|53\n\
        42|55\n\
        42|56\n\
        42|59\n\
        43|19\n\
        43|22\n\
        43|23\n\
        43|29\n\
        43|31\n\
        43|33\n\
        43|41\n\
        43|45\n\
        43|49\n\
        43|53\n\
        43|55\n\
        43|56\n\
        43|59\n\
        45|11\n\
        45|14\n\
        45|24\n\
        45|34\n\
        45|35\n\
        45|36\n\
        45|37\n\
        45|38\n\
        45|39\n\
        45|42\n\
        45|47\n\
        45|57\n\
        47|11\n\
        47|14\n\
        47|24\n\
        47|29\n\
        47|34\n\
        47|35\n\
        47|36\n\
        47|37\n\
        47|38\n\
        47|39\n\
        47|42\n\
        47|43\n\
        47|57\n\
        49|19\n\
        49|22\n\
        49|23\n\
        49|31\n\
        49|33\n\
        49|41\n\
        49|45\n\
        49|47\n\
        49|53\n\
        49|55\n\
        49|56\n\
        49|59\n\
        53|19\n\
        53|22\n\
        53|23\n\
        53|31\n\
        53|33\n\
        53|37\n\
        53|41\n\
        53|45\n\
        53|47\n\
        53|55\n\
        53|56\n\
        53|59\n\
        55|19\n\
        55|22\n\
        55|23\n\
        55|24\n\
        55|35\n\
        55|37\n\
        55|41\n\
        55|45\n\
        55|47\n\
        55|59\n\
        56|19\n\
        56|22\n\
        56|23\n\
        56|24\n\
        56|31\n\
        56|33\n\
        56|37\n\
        56|41\n\
        56|45\n\
        56|47\n\
        56|55\n\
        56|59\n\
        57|11\n\
        57|14\n\
        57|29\n\
        57|31\n\
        57|33\n\
        57|34\n\
        57|38\n\
        57|41\n\
        57|42\n\
        57|43\n\
        57|49\n\
        57|53\n\
        57|55\n\
        57|56\n\
        57|59\n\
        59|19\n\
        59|22\n\
        59|23\n\
        59|24\n\
        59|35\n\
        59|36\n\
        59|37\n\
        59|41\n\
        59|45\n\
        59|47\n\
        \n\
        11\n\
        ";
    let out = parse(input);
    assert_eq!(part1(&out), 11);
    assert_eq!(part2(&out), 0);
}

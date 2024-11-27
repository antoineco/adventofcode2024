#[allow(unused_imports)]
use aoc2024::solutions;
use aoc2024::util::parse::ParseOps;
use std::env::args;
use std::fs::read_to_string;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};

fn main() {
    let day = match args().nth(1) {
        Some(day) => day.as_str().iter_unsigned().next(),
        None => None,
    };

    let solutions = solutions()
        .into_iter()
        .filter(|solution| day.is_none_or(|day: u32| day == solution.day));

    for Solution { day, path, wrapper } in solutions {
        if let Ok(data) = read_to_string(&path) {
            let (part1, part2) = wrapper(data);

            println!("Day {day:02}");
            println!("    Part 1: {part1}");
            println!("    Part 2: {part2}");
        } else {
            eprintln!("Day {day:02}");
            eprintln!("    Missing input in {}", path.display());
        }
    }
}

struct Solution {
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

#[allow(unused_macros)]
macro_rules! solution {
    ($day:tt) => {{
        let day = stringify!($day);
        let path = Path::new("input").join(day).with_extension("txt");

        let wrapper = |data: String| {
            use solutions::$day::{parse, part1, part2};

            let input = parse(&data);
            let part1 = part1(&input);
            let part2 = part2(&input);

            (part1.to_string(), part2.to_string())
        };

        Solution {
            day: day.unsigned(),
            path,
            wrapper,
        }
    }};
}

fn solutions() -> Vec<Solution> {
    vec![]
}

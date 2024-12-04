use std::collections::HashSet;

pub fn parse(input: &str) -> LetterLocs {
    let mut lls = LetterLocs::new();
    input.lines().enumerate().for_each(|(j, line)| {
        lls.size += 1;
        line.chars().enumerate().for_each(|(i, c)| match c {
            'X' => {
                lls.x.push((i, j));
            }
            'M' => {
                lls.m.insert((i, j));
            }
            'A' => {
                lls.a.insert((i, j));
            }
            'S' => {
                lls.s.insert((i, j));
            }
            _ => (),
        })
    });
    lls
}

pub fn part1(lls: &LetterLocs) -> u32 {
    let dist_s_from_x = ("XMAS".len() - 1) as i32;
    let dist_a_from_x = dist_s_from_x - 1;
    let dist_m_from_x = dist_a_from_x - 1;
    let upper_bound = lls.size as i32 - 1;

    lls.x
        .iter()
        .map(|(x, y)| {
            let mut occurences: u32 = 0;

            // ↖ ← ↙ ↑ . ↓ ↗ → ↘
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let s_loc = (*x as i32 + i * dist_s_from_x, *y as i32 + j * dist_s_from_x);
                    if s_loc.0.is_negative()
                        || s_loc.1.is_negative()
                        || (upper_bound - s_loc.0).is_negative()
                        || (upper_bound - s_loc.1).is_negative()
                    {
                        continue;
                    }

                    let s_loc: (usize, usize) = (s_loc.0 as usize, s_loc.1 as usize);
                    let a_loc: (usize, usize) = (
                        (*x as i32 + i * dist_a_from_x) as usize,
                        (*y as i32 + j * dist_a_from_x) as usize,
                    );
                    let m_loc: (usize, usize) = (
                        (*x as i32 + i * dist_m_from_x) as usize,
                        (*y as i32 + j * dist_m_from_x) as usize,
                    );

                    if lls.is_s(s_loc) && lls.is_a(a_loc) && lls.is_m(m_loc) {
                        occurences += 1
                    }
                }
            }

            occurences
        })
        .sum()
}

pub fn part2(lls: &LetterLocs) -> u32 {
    let dist_s_from_m = ("MAS".len() - 1) as i32;
    let dist_a_from_m = dist_s_from_m - 1;
    let upper_bound = lls.size as i32 - 1;

    // 'x' shaped crosses
    let mut x_centers = HashSet::new();

    lls.m
        .iter()
        .map(|(x, y)| {
            let mut occurences: u32 = 0;

            // ↖ ↙ ↗ ↘
            // '+' shaped crosses do not count
            for i in [-1, 1] {
                for j in [-1, 1] {
                    let s_loc = (*x as i32 + i * dist_s_from_m, *y as i32 + j * dist_s_from_m);
                    if s_loc.0.is_negative()
                        || s_loc.1.is_negative()
                        || (upper_bound - s_loc.0).is_negative()
                        || (upper_bound - s_loc.1).is_negative()
                    {
                        continue;
                    }

                    let s_loc: (usize, usize) = (s_loc.0 as usize, s_loc.1 as usize);
                    let a_loc: (usize, usize) = (
                        (*x as i32 + i * dist_a_from_m) as usize,
                        (*y as i32 + j * dist_a_from_m) as usize,
                    );

                    if lls.is_s(s_loc) && lls.is_a(a_loc) && !x_centers.insert(a_loc) {
                        occurences += 1
                    }
                }
            }

            occurences
        })
        .sum()
}

pub struct LetterLocs {
    size: usize,
    x: Vec<(usize, usize)>, // x,y
    m: HashSet<(usize, usize)>,
    a: HashSet<(usize, usize)>,
    s: HashSet<(usize, usize)>,
}

impl LetterLocs {
    fn new() -> Self {
        LetterLocs {
            size: 0,
            x: Vec::new(),
            m: HashSet::new(),
            a: HashSet::new(),
            s: HashSet::new(),
        }
    }

    fn is_m(&self, coord: (usize, usize)) -> bool {
        self.m.contains(&coord)
    }

    fn is_a(&self, coord: (usize, usize)) -> bool {
        self.a.contains(&coord)
    }

    fn is_s(&self, coord: (usize, usize)) -> bool {
        self.s.contains(&coord)
    }
}

#[test]
fn sample_input() {
    let input = "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX\n\
        ";
    let lls = parse(input);
    assert_eq!(part1(&lls), 18);
    assert_eq!(part2(&lls), 9);
}

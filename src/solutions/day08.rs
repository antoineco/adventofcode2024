use std::collections::{HashMap, HashSet};

pub fn parse(input: &str) -> Grid {
    let input_unwrapped = input.replace('\n', "");
    let size = (input_unwrapped.len() as f32).sqrt() as usize;

    let mut antennas_by_freq: HashMap<char, Vec<Point>> = HashMap::new();
    input_unwrapped.chars().enumerate().for_each(|(pos, c)| {
        if c.is_ascii_alphanumeric() {
            antennas_by_freq
                .entry(c)
                .or_default()
                .push(Point((pos % size) as i32, (pos / size) as i32))
        }
    });
    let antennas = antennas_by_freq.into_values().collect();

    Grid { size, antennas }
}

pub fn part1(g: &Grid) -> usize {
    let mut antinodes = HashSet::new();
    g.antennas.iter().for_each(|antennas| {
        antennas.iter().enumerate().for_each(|(i, antenna)| {
            let rotation_points = antennas[..i].iter().chain(antennas[(i + 1)..].iter());
            rotation_points.for_each(|rp| {
                let a = antenna.rotate_180(rp);
                if g.contains(&a) {
                    antinodes.insert(a);
                }
            });
        });
    });
    antinodes.len()
}

pub fn part2(g: &Grid) -> usize {
    let mut antinodes = HashSet::new();
    g.antennas.iter().for_each(|antennas| {
        antennas.iter().enumerate().for_each(|(i, antenna)| {
            antinodes.insert(*antenna);
            let rotation_points = antennas[..i].iter().chain(antennas[(i + 1)..].iter());
            rotation_points.for_each(|rp| {
                let mut antenna = *antenna;
                let mut rp = *rp;
                loop {
                    let a = antenna.rotate_180(&rp);
                    if !g.contains(&a) {
                        break;
                    }
                    antinodes.insert(a);
                    (antenna, rp) = (rp, a);
                }
            });
        });
    });
    antinodes.len()
}

pub struct Grid {
    size: usize,
    antennas: Vec<Vec<Point>>,
}

impl Grid {
    fn contains(&self, p: &Point) -> bool {
        let upper_bound = self.size as i32;
        p.0 >= 0 && p.0 < upper_bound && p.1 >= 0 && p.1 < upper_bound
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn rotate_180(&self, rp: &Point) -> Point {
        Point(rp.0 + (rp.0 - self.0), rp.1 + (rp.1 - self.1))
    }
}

#[test]
fn sample_input() {
    let input = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............\n\
        ";
    let out = parse(input);
    assert_eq!(part1(&out), 14);
    assert_eq!(part2(&out), 34);
}

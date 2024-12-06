use std::collections::HashSet;

pub fn parse(input: &str) -> (Grid, usize) {
    let input_unwrapped: &str = &input.replace('\n', "");

    let grid = Grid {
        size: (input_unwrapped.len() as f32).sqrt() as usize,
        obstacles: input_unwrapped
            .match_indices('#')
            .map(|(pos, _)| pos)
            .collect(),
    };

    let start = input_unwrapped.find('^').unwrap();

    (grid, start)
}

pub fn part1(r#in: &(Grid, usize)) -> u32 {
    let (grid, start) = r#in;

    let mut visits = HashSet::new();
    visits.insert(*start);

    let mut cur_pos = *start;
    let mut cur_direction = Direction::Up;
    while let Some((pos, d)) = grid.step(&cur_pos, &cur_direction) {
        (cur_pos, cur_direction) = (pos, d);
        visits.insert(cur_pos);
    }

    visits.len() as u32
}

pub fn part2(_: &(Grid, usize)) -> u32 {
    0
}

pub struct Grid {
    size: usize,
    obstacles: HashSet<usize>,
}

impl Grid {
    fn step(&self, cur: &usize, d: &Direction) -> Option<(usize, Direction)> {
        let (cur_x, cur_y) = self.coordinates(cur);
        let nxt = match d {
            Direction::Up => {
                if cur_y == 0 {
                    return None;
                }
                self.position((cur_x, cur_y - 1))
            }
            Direction::Right => {
                if cur_x == self.size - 1 {
                    return None;
                }
                self.position((cur_x + 1, cur_y))
            }
            Direction::Down => {
                if cur_y == self.size - 1 {
                    return None;
                }
                self.position((cur_x, cur_y + 1))
            }
            Direction::Left => {
                if cur_x == 0 {
                    return None;
                }
                self.position((cur_x - 1, cur_y))
            }
        };

        if self.obstacles.contains(&nxt) {
            self.step(cur, &d.next())
        } else {
            Some((nxt, d.clone()))
        }
    }

    // position -> (x,y) coordinates
    fn coordinates(&self, pos: &usize) -> (usize, usize) {
        (pos % self.size, pos / self.size)
    }

    // (x,y) coordinates -> position
    fn position(&self, coord: (usize, usize)) -> usize {
        coord.0 + coord.1 * self.size
    }
}

#[derive(Clone)]
#[repr(u8)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Self {
        ((self.clone() as u8 + 1_u8) % 4).try_into().unwrap()
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    // https://doc.rust-lang.org/reference/items/enumerations.html#accessing-discriminant
    // https://internals.rust-lang.org/t/pre-rfc-enum-from-integer/6348/21
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Direction::Up as u8 => Ok(Direction::Up),
            v if v == Direction::Right as u8 => Ok(Direction::Right),
            v if v == Direction::Down as u8 => Ok(Direction::Down),
            v if v == Direction::Left as u8 => Ok(Direction::Left),
            _ => todo!(),
        }
    }
}

#[test]
fn sample_input() {
    let input = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...\n\
        ";
    let out = parse(input);
    assert_eq!(part1(&out), 41);
}

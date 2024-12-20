use petgraph::visit::{Dfs, Visitable};
use petgraph::{Directed, Graph};
use std::collections::VecDeque;

pub fn parse(input: &str) -> Topography {
    let map_locs: Vec<u8> = input.replace('\n', "").bytes().collect();
    let size = (map_locs.len() as f32).sqrt() as usize;

    let mut trailheads = Vec::new();

    let map = Graph::from_edges(
        map_locs
            .iter()
            .enumerate()
            .filter(|(_, height)| **height != b'.') // test inputs
            .flat_map(|(i, height)| {
                if *height == b'0' {
                    trailheads.push(i);
                }

                let mut edges = Vec::new();
                // ← → ↑ ↓
                if i % size != 0 && map_locs[i - 1] == height + 1 {
                    edges.push((i, i - 1));
                }
                if (i + 1) % size != 0 && map_locs[i + 1] == height + 1 {
                    edges.push((i, i + 1));
                }
                if i >= size && map_locs[i - size] == height + 1 {
                    edges.push((i, i - size));
                }
                if i + size <= size * size - 1 && map_locs[i + size] == height + 1 {
                    edges.push((i, i + size));
                }
                edges
            }),
    );

    Topography { map, trailheads }
}

pub fn part1(topo: &Topography) -> u32 {
    let Topography { map, trailheads } = topo;
    trailheads
        .iter()
        .map(|th| {
            let mut score = 0;

            let mut depth_queue = VecDeque::new();
            depth_queue.push_front(0);

            let mut dfs = Dfs::new(map, (*th).into());
            while dfs.next(map).is_some() {
                let node_depth = depth_queue.pop_front().unwrap();
                if node_depth == 9 {
                    score += 1;
                }
                for _ in depth_queue.len()..dfs.stack.len() {
                    depth_queue.push_front(node_depth + 1);
                }
            }
            score
        })
        .sum()
}

pub fn part2(topo: &Topography) -> u32 {
    let Topography { map, trailheads } = topo;
    trailheads
        .iter()
        .map(|th| {
            let mut score = 0;

            let mut depth_queue = VecDeque::new();
            depth_queue.push_front(0);

            let mut dfs = Dfs::new(map, (*th).into());
            while dfs.next(map).is_some() {
                let node_depth = depth_queue.pop_front().unwrap();
                if node_depth == 9 {
                    score += 1;
                    // Reset discovered nodes but, unlike dfs.reset(), keep the stack of nodes to visit.
                    //
                    // This allows the DFS to re-enter a branch that had previously been visited
                    // via another ancestor.
                    // Whenever an ancestor is visited, it is popped from the visit stack, which
                    // then only ever contains its successor(s). As a result, we never visit the
                    // exact same trail more than once, only portions of it.
                    map.reset_map(&mut dfs.discovered);
                }
                for _ in depth_queue.len()..dfs.stack.len() {
                    depth_queue.push_front(node_depth + 1);
                }
            }
            score
        })
        .sum()
}

pub struct Topography {
    map: Graph<(), usize, Directed, usize>,
    trailheads: Vec<usize>,
}

#[test]
fn sample_input_1() {
    let input = "\
        ...0...\n\
        ...1...\n\
        ...2...\n\
        6543456\n\
        7.....7\n\
        8.....8\n\
        9.....9\n\
        ";
    let topo = parse(input);
    assert_eq!(part1(&topo), 2);
}

#[test]
fn sample_input_2() {
    let input = "\
        ..90..9\n\
        ...1.98\n\
        ...2..7\n\
        6543456\n\
        765.987\n\
        876....\n\
        987....\n\
        ";
    let topo = parse(input);
    assert_eq!(part1(&topo), 4);
    assert_eq!(part2(&topo), 13);
}

#[test]
fn sample_input_3() {
    let input = "\
        10..9..\n\
        2...8..\n\
        3...7..\n\
        4567654\n\
        ...8..3\n\
        ...9..2\n\
        .....01\n\
        ";
    let topo = parse(input);
    assert_eq!(part1(&topo), 3);
}

#[test]
fn sample_input_4() {
    let input = "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732\n\
        ";
    let topo = parse(input);
    assert_eq!(part1(&topo), 36);
    assert_eq!(part2(&topo), 81);
}

#[test]
fn sample_input_5() {
    let input = "\
        .....0.\n\
        ..4321.\n\
        ..5..2.\n\
        ..6543.\n\
        ..7..4.\n\
        ..8765.\n\
        ..9....\n\
        ";
    let topo = parse(input);
    assert_eq!(part2(&topo), 3);
}

#[test]
fn sample_input_6() {
    let input = "\
        012345\n\
        123456\n\
        234567\n\
        345678\n\
        4.6789\n\
        56789.\n\
        ";
    let topo = parse(input);
    assert_eq!(part2(&topo), 227);
}

use std::cmp::Ordering;

pub fn parse(input: &str) -> Vec<Block> {
    input
        .bytes()
        .enumerate()
        .filter_map(|(i, c)| match c {
            b'1'..=b'9' => {
                let len = (c - b'0').into();
                Some(if i % 2 == 0 {
                    Block::File { id: i / 2, len }
                } else {
                    Block::Free(len)
                })
            }
            b'0' | b'\n' => None,
            _ => unimplemented!(),
        })
        .collect()
}

pub fn part1(diskmap: &[Block]) -> usize {
    let mut compacted = diskmap.to_vec();

    let mut i = 1;
    let mut j = compacted.len() - 1;
    while i <= j {
        let Block::Free(free_len) = compacted[i] else {
            i += 1;
            continue;
        };
        let Block::File {
            id: file_id,
            len: file_len,
        } = compacted[j]
        else {
            compacted.truncate(compacted.len() - 1);
            j -= 1;
            continue;
        };

        match file_len.cmp(&free_len) {
            Ordering::Equal => {
                compacted.swap_remove(i);
                i += 1;
                j -= 1;
            }
            Ordering::Greater => {
                compacted[i] = Block::File {
                    id: file_id,
                    len: free_len,
                };
                i += 1;
                compacted[j] = Block::File {
                    id: file_id,
                    len: file_len - free_len,
                };
            }
            Ordering::Less => {
                compacted.insert(i + 1, Block::Free(free_len - file_len));
                compacted.swap_remove(i);
                i += 1;
            }
        }
    }

    compacted
        .iter()
        .fold((0, 0), |(acc, offset), f| {
            let Block::File { id, len } = f else {
                unimplemented!()
            };
            (
                acc + (offset..(offset + len)).fold(0, |acc, mul| acc + (id * mul)),
                offset + len,
            )
        })
        .0
}

pub fn part2(_: &[Block]) -> usize {
    0
}

#[derive(Clone)]
pub enum Block {
    File { id: usize, len: usize },
    Free(usize),
}

#[test]
fn sample_input() {
    let input = "\
        2333133121414131402\n\
        ";
    let diskmap = parse(input);
    assert_eq!(part1(&diskmap), 1928);
}

#[test]
fn trivial() {
    let input = "\
        12345\n\
        ";
    let diskmap = parse(input);
    assert_eq!(part1(&diskmap), 60);
}

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

pub fn part2(diskmap: &[Block]) -> usize {
    let mut compacted = diskmap.to_vec();

    // A possible optimization here would be to keep track of all free blocks indexed by length,
    // and to check them off as we swap them with files. This would help reducing the number of
    // iterations even further, based on the length of the file being evaluated.
    let mut first_free = 1;

    let mut j = compacted.len() - 1;
    while j > first_free {
        let Block::File { len: file_len, .. } = compacted[j] else {
            if j == compacted.len() - 1 {
                compacted.truncate(compacted.len() - 1);
            }
            j -= 1;
            continue;
        };

        let mut first_free_seen = None;
        let mut moved = false;
        let mut i = first_free;
        while i < j {
            let Block::Free(free_len) = compacted[i] else {
                i += 1;
                continue;
            };
            if first_free_seen.is_none() {
                first_free_seen = Some(i);
            }

            match file_len.cmp(&free_len) {
                Ordering::Equal => {
                    compacted.swap(i, j);
                    moved = true;
                    break;
                }
                Ordering::Less => {
                    compacted.insert(i + 1, Block::Free(free_len - file_len));
                    compacted[i] = Block::Free(file_len);
                    j += 1;
                    compacted.swap(i, j);
                    moved = true;
                    break;
                }
                Ordering::Greater => i += 1,
            }
        }

        let Some(first_free_seen) = first_free_seen else {
            break;
        };
        if first_free_seen > first_free {
            first_free = first_free_seen;
        }

        if !moved {
            j -= 1;
        }
    }

    compacted
        .iter()
        .fold((0, 0), |(acc, offset), f| match f {
            Block::File { id, len } => (
                acc + (offset..(offset + len)).fold(0, |acc, mul| acc + (id * mul)),
                offset + len,
            ),
            Block::Free(len) => (acc, offset + len),
        })
        .0
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
    assert_eq!(part2(&diskmap), 2858);
}

#[test]
fn trivial() {
    let input = "\
        12345\n\
        ";
    let diskmap = parse(input);
    assert_eq!(part1(&diskmap), 60);
    assert_eq!(part2(&diskmap), 132);
}

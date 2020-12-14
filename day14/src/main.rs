use std::collections::HashMap;
type Bitmask = Vec<Option<u8>>;
type Memory = HashMap<usize, Vec<u8>>;
#[derive(Debug, PartialEq, Eq)]
struct MaskOp {
    mask: Bitmask,
    ops: Vec<(usize, u32)>,
}

#[derive(Debug, PartialEq, Eq)]
enum Changes {
    Bit(u8),
    Floating,
}
impl MaskOp {
    fn apply(&self, mut inp: Memory) -> Memory {
        for (mempos, val) in &self.ops {
            let number_binary = format!("{:036b}", val);
            number_binary
                .chars()
                .zip(&self.mask)
                .enumerate()
                .for_each(|(pos, (n, mask))| {
                    let to_insert = match mask {
                        None => n.to_digit(10).unwrap() as u8,
                        Some(n) => *n,
                    };
                    if let Some(v) = inp.get_mut(mempos) {
                        (*v)[pos] = to_insert;
                    } else {
                        inp.insert(*mempos, vec![0; 36]);
                        (*inp.get_mut(mempos).unwrap())[pos] = to_insert;
                    }
                });
        }

        inp
    }
    fn apply_2(&self, mut inp: Memory) -> Memory {
        for (mempos, val) in &self.ops {
            let memory_binary = format!("{:036b}", mempos);
            let value_binary = format!("{:036b}", val);
            let result: Vec<Changes> = memory_binary
                .chars()
                .zip(&self.mask)
                .map(|(n, mask)| match mask {
                    Some(0) => Changes::Bit(n.to_digit(10).unwrap() as u8),
                    Some(1) => Changes::Bit(1),
                    None => Changes::Floating,
                    _ => panic!(),
                })
                .collect();

            // Expand the result to all the addresses can write to
            let addresses_to_write: Vec<usize> = expand_addresses(result);
            // Write to all of those
            addresses_to_write.iter().for_each(|addr| {
                inp.insert(
                    *addr,
                    value_binary
                        .chars()
                        .map(|v| v.to_digit(10).unwrap() as u8)
                        .collect::<Vec<u8>>(),
                );
            });
        }
        inp
    }
}
fn expand_addresses(changes: Vec<Changes>) -> Vec<usize> {
    let mut base: Vec<Vec<u8>> = vec![vec![]];
    for val in changes {
        match val {
            Changes::Bit(n) => base.iter_mut().for_each(|v| v.push(n)),
            Changes::Floating => {
                let mut base1 = base.clone();
                base.iter_mut().for_each(|v| v.push(0));
                base1.iter_mut().for_each(|v| v.push(1));
                base.extend(base1.clone());
            }
        }
    }

    let res = base.iter().map(|v| to_u64(v)).map(|v| v as usize).collect();
    res
}
fn to_u64(slice: &[u8]) -> u64 {
    slice.iter().fold(0, |acc, &b| acc * 2 + (b as u64))
}
fn memory_sum(mem: Memory) -> u64 {
    mem.values()
        .map(|memory_address| to_u64(memory_address))
        .sum()
}
fn parse(inp: &str) -> Vec<MaskOp> {
    let splitted_by_mask = inp.split("mask = ");
    splitted_by_mask.skip(1).map(parse_one).collect()
}
fn parse_one(inp: &str) -> MaskOp {
    let mut lin = inp.lines();
    let mask: Vec<Option<u8>> = lin
        .next()
        .unwrap()
        .chars()
        .map(|v| match v {
            'X' => None,
            n => Some(n.to_digit(10).unwrap() as u8),
        })
        .collect();
    let ops = lin
        .map(|m| {
            let mempos = m
                .split('[')
                .nth(1)
                .unwrap()
                .split(']')
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();
            let memval = m.split(" = ").last().unwrap().parse().unwrap();
            (mempos, memval)
        })
        .collect();

    MaskOp { mask, ops }
}
fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
fn part1(inp: &Vec<MaskOp>) -> u64 {
    let mut mem: Memory = HashMap::new();
    for op in inp {
        mem = op.apply(mem);
    }
    // Need to sum the values in memory
    memory_sum(mem)
}
fn part2(inp: &Vec<MaskOp>) -> u64 {
    let mut mem: Memory = HashMap::new();
    for op in inp {
        mem = op.apply_2(mem);
    }
    // Need to sum the values in memory
    memory_sum(mem)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test2() {
        let input = parse(include_str!("../test2.txt"));

        let mut mem: Memory = HashMap::new();
        let mem58_expect: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 0, 0, 1, 0, 0,
        ];
        for op in input {
            mem = op.apply_2(mem);
        }

        assert_eq!(*mem.get(&59).unwrap(), mem58_expect);
        assert_eq!(memory_sum(mem), 208);
    }
    #[test]
    fn test1() {
        let input = parse(include_str!("../test1.txt"));
        let input_one = &input[0];
        assert_eq!(
            input_one,
            &MaskOp {
                mask: vec![
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(1),
                    None,
                    None,
                    None,
                    None,
                    Some(0),
                    None
                ],
                ops: vec![(8, 11), (7, 101), (8, 0)]
            }
        );
        let mem: Memory = HashMap::new();
        let mem8: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0,
        ];
        let mem = input_one.apply(mem);
        assert_eq!(*mem.get(&8).unwrap(), mem8);
        assert_eq!(memory_sum(mem), 165);
    }
}

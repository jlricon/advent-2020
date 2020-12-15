use std::collections::HashMap;
type Input = Vec<u64>;
type Solution = u64;
fn parse(inp: &str) -> Input {
    inp.split(",").map(|n| n.parse().unwrap()).collect()
}
fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("Part 1: {}", part1(&input, 2020));
    println!("Part 2: {}", part1(&input, 30000000));
}
fn part1(inp: &Input, n: usize) -> Solution {
    let mut nums_to_turn: HashMap<u64, Vec<usize>> = HashMap::new();
    inp.iter().enumerate().for_each(|(pos, v)| {
        nums_to_turn.insert(*v, vec![pos + 1]);
    });
    let mut last_n = *inp.last().unwrap();
    let mut prev_first_time_spoken = true;
    for turn in inp.len() + 1..(n + 1) {
        if prev_first_time_spoken {
            last_n = 0;
        } else {
            let prev_turns = nums_to_turn
                .get(&last_n)
                .unwrap()
                .iter()
                .nth_back(1)
                .unwrap();
            let turn_diff = turn - 1 - prev_turns;
            last_n = turn_diff as u64;
        };
        prev_first_time_spoken = !nums_to_turn.contains_key(&last_n);
        if let Some(entry) = nums_to_turn.get_mut(&last_n) {
            (*entry).push(turn);
        } else {
            nums_to_turn.insert(last_n, vec![turn]);
        }
    }

    last_n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = parse(include_str!("../test1.txt"));
        let res = part1(&input, 2020);
        assert_eq!(res, 436);
    }
}

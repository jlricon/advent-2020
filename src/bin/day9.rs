use itertools::{Itertools, MinMaxResult};

fn is_sum_of(v: &Vec<&u64>, target_n: u64) -> bool {
    let combis = v.iter().combinations(2);

    combis
        .into_iter()
        .map(|v| (v.into_iter().unique().map(|v| *v).sum::<u64>()) == target_n)
        .any(|v| v)
}
fn main() {
    let input: Vec<u64> = include_str!("../../input/day9.txt")
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let mut preamble: Vec<&u64> = input.iter().take(25).collect();
    let mut nums_to_validate = input.iter().skip(25);
    // Part 1
    let invalid = loop {
        let to_validate = nums_to_validate.next().unwrap();
        let is_valid = is_sum_of(&preamble, *to_validate);
        if !is_valid {
            println!("Invalid number: {}", to_validate);
            break to_validate;
        }
        preamble = preamble.iter().skip(1).copied().collect();
        preamble.push(to_validate);
    };
    // Part 2
    let (from, to) = input
        .iter()
        .enumerate()
        .map(|(pos, _)| {
            input
                .iter()
                .skip(pos)
                .scan(0, |acc, b| {
                    *acc = *acc + b;
                    Some(*acc)
                })
                .enumerate()
                .filter(|(_, v)| v == invalid)
                .map(|v| v.0)
                .nth(0)
                .map(|p| (pos, p))
        })
        .filter(|x| !x.is_none())
        .nth(0)
        .unwrap()
        .unwrap();
    if let MinMaxResult::MinMax(min, max) = input.iter().skip(from).take(to).minmax() {
        println!("Part 2: {}", max + min);
    } else {
        panic!()
    }
}

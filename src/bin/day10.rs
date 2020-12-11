use std::collections::HashMap;

fn main() {
    let mut input: Vec<u64> = include_str!("../../input/day10.txt")
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let rated_for: u64 = input.iter().max().unwrap() + 3;
    input.push(0);
    input.push(rated_for);
    input.sort();

    let diffs: Vec<u64> = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();
    assert_eq!(diffs.iter().filter(|v| **v > 3).count(), 0);
    let one = diffs.iter().filter(|n| **n == 1).count();
    let three = diffs.iter().filter(|n| **n == 3).count();
    let mut ways: HashMap<u64, u64> = HashMap::new();
    ways.insert(0, 1);
    for i in input.iter().skip(1) {
        let other_ways = *ways.entry(i.saturating_sub(1)).or_default()
            + *ways.entry(i.saturating_sub(2)).or_default()
            + *ways.entry(i.saturating_sub(3)).or_default();
        ways.insert(*i, other_ways);
    }
    let last = *input.iter().last().unwrap();
    println!("Part 1 {}*{}={}", one, three, one * three);
    println!("Part 2 {}", ways.get(&last).unwrap());
}

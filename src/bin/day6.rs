use itertools::Itertools;
use std::collections::HashMap;
fn main() {
    let input = include_str!("../../input/day6.txt");
    let part1: usize = input
        .split("\n\n")
        .map(|l| l.chars().filter(|c| *c != '\n').unique().count())
        .sum();
    println!("{}", part1);

    let part2: usize = input
        .split("\n\n")
        .map(|l| {
            let mut counts: HashMap<char, usize> = HashMap::new();
            let people = l.chars().filter(|c| *c == '\n').count() + 1;
            l.chars()
                .filter(|c| *c != '\n')
                .for_each(|c| *counts.entry(c).or_insert(0) += 1);
            counts.values().filter(|val| **val == people).count()
        })
        .sum();
    println!("{}", part2);
}

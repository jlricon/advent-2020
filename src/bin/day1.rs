use itertools::Itertools;
use std::collections::HashMap;
const SUM_UP_TO: i32 = 2020;

fn part2_fast(nums: &Vec<i32>) {
    let mut hash = HashMap::<i32, (&i32, &i32)>::new();
    nums.into_iter()
        .tuple_combinations()
        .for_each(|(val1, val2)| {
            hash.entry(val1 + val2).or_insert((val1, val2));
        });
    nums.iter().for_each(|val| {
        let remainder = SUM_UP_TO - val;
        if let Some(ent) = hash.get(&remainder) {
            println!("{}", ent.0 * ent.1 * val);
            return;
        }
    })
}

fn main() {
    let nums: Vec<i32> = include_str!("../../input/day1.txt")
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    part1(&nums);
    part2(&nums);
    part2_fast(&nums);
}
fn part1(nums: &Vec<i32>) {
    nums.into_iter()
        .tuple_combinations()
        .for_each(|(val1, val2)| {
            if val1 + val2 == SUM_UP_TO {
                println!("{}", val1 * val2);
                return;
            }
        })
}
fn part2(nums: &Vec<i32>) {
    nums.into_iter()
        .tuple_combinations()
        .for_each(|(val1, val2, val3)| {
            if val1 + val2 + val3 == SUM_UP_TO {
                println!("{}", val1 * val2 * val3);
                return;
            }
        })
}

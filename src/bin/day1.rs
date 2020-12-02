use std::collections::HashMap;

const SUM_UP_TO: i32 = 2020;
fn part1(nums: &Vec<i32>) {
    for (first, val1) in nums.iter().enumerate() {
        for (_, val2) in nums.iter().skip(first).enumerate() {
            if val1 + val2 == SUM_UP_TO {
                println!("{}", val1 * val2);
                return;
            }
        }
    }
}
fn part2(nums: &Vec<i32>) {
    let mut hash = HashMap::<i32, (&i32, &i32)>::new();
    for (first, val1) in nums.iter().enumerate() {
        for (_, val2) in nums.iter().skip(first).enumerate() {
            hash.entry(val1 + val2).or_insert((val1, val2));
        }
    }
    for val in nums.iter() {
        let remainder = SUM_UP_TO - val;
        if let Some(ent) = hash.get(&remainder) {
            println!("{}", ent.0 * ent.1 * val);
            return;
        }
    }
}
fn main() {
    let nums: Vec<i32> = include_str!("../../input/day1.txt")
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    part1(&nums);
    part2(&nums)
}

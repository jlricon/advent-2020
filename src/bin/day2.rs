use std::convert::TryInto;

type Password<'a> = &'a str;

fn part1() {
    let valid_pass: i32 = include_str!("../../input/day2.txt")
        .split("\n")
        .map(|line| {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            let password: Password = splitted[2];
            let policy_letter: char = splitted[1].chars().next().unwrap();
            let splitted_from_to: Vec<i32> = splitted[0]
                .split("-")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            let from = splitted_from_to[0];
            let to = splitted_from_to[1];
            let n_letter_in_password =
                password.chars().filter(|c| *c == policy_letter).count() as i32;
            if n_letter_in_password >= from && n_letter_in_password <= to {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();
    println!("{}", valid_pass)
}
fn part2() {
    let valid_pass: i32 = include_str!("../../input/day2.txt")
        .split("\n")
        .map(|line| {
            let splitted: Vec<&str> = line.split_whitespace().collect();
            let password: Password = splitted[2];
            let policy_letter: char = splitted[1].chars().next().unwrap();

            let splitted_from_to: Vec<i32> = splitted[0]
                .split("-")
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            fn split_by_pos(
                pos: usize,
                password: &Password,
                splitted_from_to: &Vec<i32>,
                policy_letter: char,
            ) -> bool {
                password
                    .chars()
                    .skip((splitted_from_to[pos] - 1).try_into().unwrap())
                    .next()
                    .map_or(false, |val| val == policy_letter)
            };
            let first_pos_matches = split_by_pos(0, &password, &splitted_from_to, policy_letter);
            let second_pos_matches = split_by_pos(1, &password, &splitted_from_to, policy_letter);
            if first_pos_matches ^ second_pos_matches {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{}", valid_pass)
}
fn main() {
    part1();
    part2();
}

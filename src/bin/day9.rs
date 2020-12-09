use im_rc::Vector;
use itertools::Itertools;
fn is_sum_of(v: &Vector<u64>, target_n: u64) -> bool {
    let combis = v.iter().combinations(2);

    combis
        .into_iter()
        .map(|v| (v.into_iter().unique().sum::<u64>()) == target_n)
        .any(|v| v)
}
fn main() {
    let input: Vector<u64> = include_str!("../../input/day9.txt")
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let mut preamble = input.take(25);
    let mut nums_to_validate = input.skip(25);
    // Part 1
    let invalid = loop {
        let to_validate = nums_to_validate.pop_front().unwrap();
        let is_valid = is_sum_of(&preamble, to_validate);
        if !is_valid {
            println!("Invalid number: {}", to_validate);
            break to_validate;
        }
        preamble.pop_front();
        preamble.push_back(to_validate);
    };
    // Part 2

    let (from, to) = input
        .iter()
        .enumerate()
        .map(|(pos, _)| {
            let post2 = input
                .iter()
                .skip(pos)
                .scan(0, |acc, b| {
                    *acc = *acc + b;
                    Some(*acc)
                })
                .enumerate()
                .filter(|(_, v)| *v == invalid)
                .map(|v| v.0)
                .nth(0);
            if let Some(p) = post2 {
                Some((pos, p))
            } else {
                None
            }
        })
        .filter(|x| !x.is_none())
        .nth(0)
        .unwrap()
        .unwrap();
    let range = input.skip(from).take(to);
    println!(
        "Part 2: {}",
        range.iter().max().unwrap() + range.iter().min().unwrap()
    );
}

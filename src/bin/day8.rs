use std::collections::HashSet;

fn main() {
    // Part 1 and 2 together. Being sneaky and manually changing the relevant input value :P
    let input: Vec<(&str, i32)> = include_str!("../../input/day8.txt")
        .lines()
        .map(|l| {
            let mut splitted = l.split_whitespace();
            let inst = splitted.nth(0).unwrap();
            let val: i32 = splitted.nth(0).unwrap().parse().unwrap();
            (inst, val)
        })
        .collect();
    let mut acc: i32 = 0;
    let mut executed_so_far: HashSet<i32> = HashSet::new();
    let mut icounter: i32 = 0;
    let mut prev_icounter: i32 = 0;
    loop {
        if executed_so_far.contains(&icounter) {
            println!("value {} at line {} from {}", acc, icounter, prev_icounter);
            break;
        }

        let &(instruction, val) = input
            .get(icounter as usize)
            .expect(&format!("Reached the end. Final value is {}", acc));

        executed_so_far.insert(icounter);
        prev_icounter = icounter;
        icounter += match instruction {
            "nop" => 1,
            "acc" => {
                acc += val;
                1
            }
            "jmp" => val,
            _ => panic!(),
        };
    }
}

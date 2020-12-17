use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
type Solution = u64;
#[derive(Debug)]
struct Input<'a> {
    rules: AllRules<'a>,
    my_ticket: Vec<u64>,
    other_tickets: Vec<Vec<u64>>,
}
#[derive(Debug)]
struct Rule {
    first_range: [u64; 2],
    second_range: [u64; 2],
}
impl Rule {
    fn meets_rule(&self, x: u64) -> bool {
        if (x >= self.first_range[0] && x <= self.first_range[1])
            || (x >= self.second_range[0] && x <= self.second_range[1])
        {
            true
        } else {
            false
        }
    }
}
type AllRules<'a> = HashMap<&'a str, Rule>;
fn parse(inp: &str) -> Input {
    let mut inputs = inp.split("\n\n");
    let rules = inputs
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut name_and_rest = line.split(": ");
            let rulename = name_and_rest.next().unwrap();
            let mut rest = name_and_rest.last().unwrap().split(" or ");
            let mut first_range = [0, 0];
            let mut second_range = [0, 0];
            rest.next()
                .unwrap()
                .split("-")
                .enumerate()
                .for_each(|(pos, v)| first_range[pos] = v.parse().unwrap());
            rest.next()
                .unwrap()
                .split("-")
                .enumerate()
                .for_each(|(pos, v)| second_range[pos] = v.parse().unwrap());
            (
                rulename,
                Rule {
                    first_range,
                    second_range,
                },
            )
        })
        .collect();
    let my_ticket: Vec<u64> = inputs
        .next()
        .unwrap()
        .split(":\n")
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    let other_tickets: Vec<Vec<u64>> = inputs
        .next()
        .unwrap()
        .split(":\n")
        .last()
        .unwrap()
        .lines()
        .map(|line| line.split(",").map(|c| c.parse().unwrap()).collect())
        .collect();
    Input {
        rules,
        my_ticket,
        other_tickets,
    }
}
fn main() {
    let input = parse(include_str!("../input.txt"));
    //println!("{:?}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
fn part1(inp: &Input) -> Solution {
    // For each nearby ticket, for each value, check if it meets any rule

    inp.other_tickets
        .iter()
        .map(|one_ticket| {
            one_ticket
                .iter()
                .filter_map(|one_field| {
                    // This particular field meets at least one of the rules
                    let is_field_valid_by_any_rule = inp
                        .rules
                        .values()
                        .map(|rule| rule.meets_rule(*one_field))
                        .any(|v| v);
                    if is_field_valid_by_any_rule {
                        None
                    } else {
                        Some(*one_field)
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}
fn part2(inp: &Input) -> Solution {
    // For each nearby ticket, for each value, check if it meets any rule
    let valid_tickets: Vec<&Vec<u64>> = inp
        .other_tickets
        .iter()
        .filter_map(|one_ticket| {
            let is_ticket_valid = one_ticket
                .iter()
                .map(|one_field| {
                    // This particular field meets at all rules
                    inp.rules
                        .values()
                        .map(|rule| rule.meets_rule(*one_field))
                        .any(|v| v)
                })
                .all(|v| v);
            if is_ticket_valid {
                Some(one_ticket)
            } else {
                None
            }
        })
        .collect();
    // Which rules are matched by each position?
    let mut rulename_to_position: HashMap<&str, HashSet<usize>> = inp
        .rules
        .iter()
        .map(|(rulename, rule)| {
            let ticket_to_positions_that_match_rule: Vec<HashSet<usize>> = valid_tickets
                .iter()
                // Positions that  comply with the rule
                .map(|ticket| {
                    ticket
                        .iter()
                        .enumerate()
                        .filter_map(|(pos, val)| match rule.meets_rule(*val) {
                            true => Some(pos),
                            false => None,
                        })
                        .collect::<HashSet<usize>>()
                })
                .collect();

            let pre_pos = ticket_to_positions_that_match_rule
                .into_iter()
                .fold1(|a, b| {
                    let new_set = a.intersection(&b);
                    // dbg!(&new_set);
                    new_set.copied().collect::<HashSet<usize>>()
                })
                .unwrap();
            // Make sure there is at least some stuff in pre_pos, otherwise there are no candidate positions for that entry!
            assert_ne!(pre_pos.len(), 0);
            (*rulename, pre_pos)
        })
        .collect();
    let mut positions_taken: HashSet<usize> = HashSet::new();
    // Need to dedupe now
    while rulename_to_position
        .values()
        .map(|l| l.len() != 1)
        .any(|v| v)
    {
        rulename_to_position = rulename_to_position
            .into_iter()
            .map(|(k, v)| {
                assert_ne!(v.len(), 0);
                if v.len() == 1 {
                    positions_taken.insert(*v.iter().nth(0).unwrap());
                    (k, v)
                } else {
                    // We can probably intersect
                    let inters: HashSet<usize> = v.difference(&positions_taken).copied().collect();
                    //   let definite_position = *inters.iter().nth(0).unwrap();
                    (k, inters)
                }
            })
            .collect();
        // Print avg len for debugging
        let m: usize = rulename_to_position
            .values()
            .map(|v| v.len())
            .sum::<usize>()
            / rulename_to_position.len();
        println!("Average len {}", m);
    }

    rulename_to_position
        .iter()
        .filter_map(|(l, v)| {
            if l.starts_with("departure") {
                let pos = *v.iter().nth(0).unwrap();
                Some(inp.my_ticket[pos] as u64)
            } else {
                None
            }
        })
        .product::<u64>()
}

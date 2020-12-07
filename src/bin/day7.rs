use std::{collections::HashMap, time::Instant};
const GOLD: &str = "shiny gold";
fn does_bag_contain_shiny<'a>(
    bag_color: &'a str,
    bag_to_bag_to_n: &HashMap<&str, HashMap<&'a str, u32>>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    // This memoisation makes it around 10x faster
    if let Some(res) = cache.get(bag_color) {
        return *res;
    }
    let res = match bag_to_bag_to_n.get(bag_color) {
        None => false,
        Some(contained_bags_to_n) if contained_bags_to_n.contains_key(GOLD) => true,
        Some(contained_bags_to_n) => contained_bags_to_n
            .keys()
            .map(|k| does_bag_contain_shiny(k, bag_to_bag_to_n, cache))
            .any(|v| v == true),
    };
    cache.insert(bag_color, res);
    res
}

fn count_bags_inside_bag(color: &str, bag_to_bag_to_n: &HashMap<&str, HashMap<&str, u32>>) -> u32 {
    match bag_to_bag_to_n.get(color) {
        None => 0,
        Some(contained_bags) => {
            let n_bags_inside_this: u32 = contained_bags.values().sum();
            let bags_inside_bags: u32 = contained_bags
                .iter()
                .map(|(k, v)| v * count_bags_inside_bag(k, bag_to_bag_to_n))
                .sum();
            n_bags_inside_this + bags_inside_bags
        }
    }
}
fn main() {
    let start = Instant::now();
    let input = include_str!("../../input/day7.txt");
    let clean_input = input
        .replace(".", "")
        .replace(" bags", "")
        .replace(" bag", "");
    let mut bag_to_bag_to_n: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    clean_input.lines().for_each(|lin| {
        let mut two_parts = lin.split(" contain ");
        let bag_name = two_parts.next().unwrap();
        let mut bag_to_n: HashMap<&str, u32> = HashMap::new();
        two_parts.next().unwrap().split(", ").for_each(|other_bag| {
            if other_bag != "no other" {
                let n = other_bag.chars().nth(0).unwrap().to_digit(10).unwrap();
                let inner_bag_name: &str = &other_bag[2..];
                bag_to_n.insert(inner_bag_name, n);
            }
        });
        if !bag_to_n.is_empty() {
            bag_to_bag_to_n.insert(bag_name, bag_to_n);
        }
    });
    // Part 1
    let mut cache = HashMap::new();
    let n_contain_shiny = bag_to_bag_to_n
        .keys()
        .map(|k| does_bag_contain_shiny(k, &bag_to_bag_to_n, &mut cache))
        .filter(|v| *v == true)
        .count();
    println!("{}", n_contain_shiny);
    // Part 2
    let counted_bags = count_bags_inside_bag(GOLD, &bag_to_bag_to_n);
    println!("{}", counted_bags);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

use im_rc::{HashMap, HashSet};

type Allergen = String;
type Ingredient = String;
use itertools::Itertools;
struct Input {
    inp: Vec<(HashSet<Ingredient>, HashSet<Allergen>)>,
    all_allergens: HashSet<Allergen>,
    all_ingredients: HashSet<Ingredient>,
}
fn solve(inp: Input) -> (usize, String) {
    // For each allergen, find
    let mut food_to_potential_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    for (ing, al) in inp.inp.iter() {
        for ingredient in ing {
            for allergen in al {
                if let Some(val) = food_to_potential_allergen.get_mut(ingredient) {
                    val.insert(allergen.to_owned());
                } else {
                    let mut set = HashSet::new();
                    set.insert(allergen.clone());
                    food_to_potential_allergen.insert(ingredient.clone(), set);
                };
            }
        }
    }
    let mut food_to_allergen = HashMap::new();
    // For each allergen, find a
    for _ in 0..inp.all_allergens.len() {
        for allergen in &inp.all_allergens {
            let equations_for_allergen: Vec<HashSet<Ingredient>> = inp
                .inp
                .clone()
                .into_iter()
                .filter_map(|(ing, al)| {
                    if al.contains(allergen) {
                        Some(ing)
                    } else {
                        None
                    }
                })
                .collect();
            // Merge sets
            let common_ingredients = equations_for_allergen
                .into_iter()
                .fold1(|x, y| x.intersection(y))
                .unwrap();
            let common_ingredients_not_assigned_already: Vec<_> = common_ingredients
                .into_iter()
                .filter(|v| !food_to_allergen.contains_key(v))
                .collect();
            // If there is just 1 it means we kn
            if common_ingredients_not_assigned_already.len() == 1 {
                food_to_allergen
                    .insert(common_ingredients_not_assigned_already[0].clone(), allergen);
            }
        }
    }
    // Now that we know for sure the ingredient->allergen mapping, we need to see for which ones we are sure
    let other_ingredients: HashSet<String> = inp
        .all_ingredients
        .clone()
        .into_iter()
        .filter(|i| !food_to_allergen.contains_key(i))
        .map(|v| v.clone())
        .collect();
    // How many times do these appear in the recipes?
    let total_appearences: usize = inp
        .inp
        .clone()
        .into_iter()
        .map(|(ingred, _)| {
            let inters = ingred.intersection(other_ingredients.clone());
            inters.len()
        })
        .sum();
    let sorted_foods = food_to_allergen
        .iter()
        .sorted_by_key(|k| k.1)
        .map(|v| v.0)
        .join(",");

    (total_appearences, sorted_foods)
}
fn main() {
    let input: Vec<(HashSet<String>, HashSet<Allergen>)> = include_str!("../../input/day21.txt")
        .lines()
        .map(|l| {
            let mut spl = l.split(" (contains ");
            let ingredients: HashSet<String> = spl
                .next()
                .unwrap()
                .split(" ")
                .map(|v| v.to_owned())
                .collect();
            let allergens: HashSet<String> = spl
                .nth(0)
                .unwrap()
                .replace(")", "")
                .split(", ")
                .map(|v| v.to_owned())
                .collect();
            (ingredients, allergens)
        })
        .collect();
    let mut all_allergens: HashSet<String> = HashSet::new();
    input.iter().for_each(|v| {
        v.1.iter().for_each(|v| {
            all_allergens.insert((*v).to_string());
        })
    });
    let mut all_ingredients = HashSet::new();
    input.iter().for_each(|v| {
        v.0.iter().for_each(|v| {
            all_ingredients.insert((*v).to_string());
        })
    });
    let input2 = Input {
        inp: input,
        all_allergens,
        all_ingredients,
    };
    let sol = solve(input2);
    println!("Part 1 and 2: {:?}", sol);
}

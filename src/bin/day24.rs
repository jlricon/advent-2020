use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}
fn main() {
    use Direction::*;
    let inp: Vec<Vec<Direction>> = include_str!("../../input/day24.txt")
        .lines()
        .map(|l| {
            let mut result = vec![];
            let mut crs = l.chars();
            while let Some(direction) = crs.next() {
                // println!("Parting {} {}", curr, c);
                let to_push = match direction {
                    'w' => Direction::West,
                    'e' => Direction::East,
                    _ => {
                        let direction2 = crs.next().unwrap();
                        match (direction, direction2) {
                            ('n', 'w') => Direction::Northwest,
                            ('n', 'e') => Direction::Northeast,
                            ('s', 'e') => Direction::Southeast,
                            ('s', 'w') => Direction::Southwest,
                            _ => panic!(),
                        }
                    }
                };
                result.push(to_push);
            }
            result
        })
        .collect();
    // We start at false, which represents white
    let mut coords: HashMap<(i32, i32), bool> = HashMap::new();
    // Fill time!
    for (i, j) in (-200..200).cartesian_product(-200..200) {
        coords.insert((i, j), false);
    }
    for line in inp {
        let mut pos = (0, 0);
        for dir in line {
            let next_pos = match dir {
                East => (1, 0),
                Southeast => (0, 1),
                Southwest => (-1, 1),
                West => (-1, 0),
                Northwest => (0, -1),
                Northeast => (1, -1),
            };
            pos.0 += next_pos.0;
            pos.1 += next_pos.1;
        }
        // If we haven't seen it before, it was white, flip to black
        if let Some(entry) = coords.get_mut(&pos) {
            *entry = !*entry;
        } else {
            coords.insert(pos, true);
        }
    }

    let nblack = coords.iter().filter(|(k, v)| **v == true).count();
    println!("Part 1: {}", nblack);

    // For part 2
    for _ in 1..=100 {
        let mut proposed = vec![];
        for (pos, val) in coords.iter() {
            // If val is white
            let adj = count_adjacent(*pos, &coords);
            if !val && adj == 2 {
                proposed.push((*pos, !val));
            } else if *val && (adj == 0 || adj > 2) {
                proposed.push((*pos, !val));
            }
        }
        proposed.iter().for_each(|(k, v)| {
            coords.insert(*k, *v);
        });
    }
    let nblack = coords.iter().filter(|(k, v)| **v == true).count();
    println!("Part 2: {}", nblack);
}

fn count_adjacent(pos: (i32, i32), hmap: &HashMap<(i32, i32), bool>) -> usize {
    let mut two = 0;
    for candidate in [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)].iter() {
        if let Some(v) = hmap.get(&(pos.0 + candidate.0, pos.1 + candidate.1)) {
            if *v == true {
                two += 1;
            }
        }
    }
    two
}

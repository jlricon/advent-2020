use itertools::Itertools;
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Tile {
    Occupied,
    Empty,
    Floor,
}
fn count_adjacent_seats(map: &Vec<Vec<Tile>>, coord: (usize, usize)) -> u32 {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(a, b)| !(*a == 0 && *b == 0))
        .map(|(a, b): (i32, i32)| {
            let maybe_coords = (coord.0 as i32 + a, coord.1 as i32 + b);
            match maybe_coords {
                (a, b) if a < 0 || b < 0 => None,
                (a, b) => map.get(a as usize).map_or(None, |v| v.get(b as usize)),
            }
        })
        .map(|maybe_tile| match maybe_tile {
            Some(Tile::Occupied) => 1,
            _ => 0,
        })
        .sum()
}
fn count_far_seats(map: &Vec<Vec<Tile>>, coord: (usize, usize)) -> u32 {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(a, b)| !(*a == 0 && *b == 0))
        .map(|(a, b): (i32, i32)| {
            let mut maybe_coords = (coord.0 as i32 + a, coord.1 as i32 + b);
            loop {
                let maybe_tile = match maybe_coords {
                    (a, b) if a < 0 || b < 0 => None,
                    (a, b) => map.get(a as usize).map_or(None, |v| v.get(b as usize)),
                };
                match maybe_tile {
                    None => break 0,
                    Some(Tile::Occupied) => break 1,
                    Some(Tile::Empty) => break 0,
                    Some(Tile::Floor) => (),
                };
                maybe_coords = (maybe_coords.0 + a, maybe_coords.1 + b);
            }
        })
        .sum()
}
fn count_occupied_seats(map: &Vec<Vec<Tile>>) -> u32 {
    map.iter()
        .map(|v| v.iter().filter(|t| **t == Tile::Occupied).count() as u32)
        .sum()
}
fn change_seat(
    map: &Vec<Vec<Tile>>,
    x: usize,
    y: usize,
    n_for_change: u32,
    seat_count_fn: fn(&Vec<Vec<Tile>>, (usize, usize)) -> u32,
) -> Tile {
    let this_tile = map[x][y];
    match this_tile {
        Tile::Empty if seat_count_fn(map, (x, y)) == 0 => Tile::Occupied,
        Tile::Occupied if seat_count_fn(map, (x, y)) >= n_for_change => Tile::Empty,
        _ => this_tile,
    }
}

fn step_map(
    map: &Vec<Vec<Tile>>,
    n_for_change: u32,
    seat_count_fn: fn(&Vec<Vec<Tile>>, (usize, usize)) -> u32,
) -> (Vec<Vec<Tile>>, bool) {
    let mut are_equal = true;
    let new_map = map
        .iter()
        .enumerate()
        .map(|(row, inner)| {
            inner
                .iter()
                .enumerate()
                .map(|(col, this_tile)| {
                    let new_tile = change_seat(map, row, col, n_for_change, seat_count_fn);

                    if *this_tile != new_tile {
                        are_equal = false;
                    }
                    new_tile
                })
                .collect()
        })
        .collect();
    (new_map, are_equal)
}
fn main() {
    let og_map: Vec<Vec<Tile>> = include_str!("../../input/day11.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Tile::Empty,
                    '.' => Tile::Floor,
                    '#' => Tile::Occupied,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let mut part1_map = og_map.clone();
    let nseats = loop {
        let map_and_eq = step_map(&part1_map, 4, count_adjacent_seats);
        if map_and_eq.1 {
            break count_occupied_seats(&map_and_eq.0);
        }
        part1_map = map_and_eq.0;
    };
    println!("Part 1: {}", nseats);
    let mut part2_map = og_map.clone();
    let nseats = loop {
        let map_and_eq = step_map(&part2_map, 5, count_far_seats);
        if map_and_eq.1 {
            break count_occupied_seats(&map_and_eq.0);
        }
        part2_map = map_and_eq.0;
    };
    println!("Part 2: {}", nseats);
}

// #[cfg(test)]
// mod test {
//     use crate::Tile;

//     //use super::change_seat;
//     use super::count_far_seats;
//     #[test]
//     fn test_change_seat() {
//         let mut nrow = 0;
//         let mut ncol = 0;
//         let mut map = super::HashMap::new();
//         r#".............
// .L.L.#.#.#.#.
// ............."#
//             .lines()
//             .enumerate()
//             .for_each(|(row, line)| {
//                 line.chars().enumerate().for_each(|(col, c)| {
//                     let tile = match c {
//                         'L' => Tile::Empty,
//                         '.' => Tile::Floor,
//                         '#' => Tile::Occupied,
//                         _ => panic!(),
//                     };
//                     map.insert((row, col), tile);
//                     nrow = nrow.max(row);
//                     ncol = ncol.max(col);
//                 })
//             });
//         let fs = count_far_seats(&map, (1, 3));
//         assert_eq!(fs, 1);
//     }
// }

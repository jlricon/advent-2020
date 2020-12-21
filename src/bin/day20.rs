use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};
type InnerTile = Vec<Vec<bool>>;
use rand::prelude::*;
#[derive(Eq, PartialEq, Clone, Debug)]
struct Tile {
    // Stored as rows
    tile: InnerTile,
    up: Option<usize>,
    below: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    rotated: bool,
}
#[derive(Debug, PartialEq, Eq)]
enum TilePos {
    Up,
    Below,
    Left,
    Right,
}
enum Rotation {
    // Multiples of 90 degrees. 360 is the same so no.
    One,
    Two,
    Three,
}
impl Tile {
    fn stack_horizontally_with(&self, t: &Tile) -> Tile {
        let new_tile = self
            .tile
            .iter()
            .zip(t.tile.iter())
            .map(|(a, b)| a.into_iter().chain(b.into_iter()).map(|v| *v).collect())
            .collect();
        Tile::new(new_tile)
    }
    fn stack_vertically_with(&self, t: &Tile) -> Tile {
        let mut new_tile = self.tile.clone();
        new_tile.extend(t.tile.clone());

        Tile::new(new_tile)
    }
    fn set_rotated(&mut self) {
        self.rotated = true;
    }
    fn matches_with_any_other_orientation(&self, t: &Tile) -> Option<(TilePos, InnerTile)> {
        if !t.rotated {
            let flips = [
                |v: Tile| v.vertical_flip(),
                |v: Tile| v.horizontal_flip(),
                |v: Tile| v,
            ];
            let rotations = [
                |v: Tile| v,
                |v: Tile| v.rotate(Rotation::One),
                |v: Tile| v.rotate(Rotation::Two),
                |v: Tile| v.rotate(Rotation::Three),
            ];
            // If we do all the flips and rotations, can we match the tile?

            let mut res = None;
            for (flip, rotation) in flips.iter().cartesian_product(rotations.iter()) {
                let new_self: Tile = t.clone();
                let mut repositioned_self = flip(rotation(new_self));
                if let Some(n) = self.is_located_relative_to(&repositioned_self) {
                    repositioned_self.set_rotated();
                    res = Some((n, repositioned_self.tile));
                    break;
                }
            }
            res
        } else {
            if let Some(n) = self.is_located_relative_to(&t) {
                Some((n, t.clone().tile))
            } else {
                None
            }
        }
    }
    fn is_located_relative_to(&self, t: &Tile) -> Option<TilePos> {
        // Check if this tile shares borders with another tile
        // Returns the position of the other tile wrt this one
        // Self upper and t lower
        if self.up == None && t.below == None && self.tile.first() == t.tile.iter().last() {
            Some(TilePos::Up)
        }
        // Self lower and t upper
        else if self.below == None && t.up == None && t.tile.first() == self.tile.iter().last() {
            Some(TilePos::Below)
        }
        // Self right and t left
        else if self.right == None
            && t.left == None
            && self
                .tile
                .iter()
                .map(|v| *v.last().unwrap())
                .collect::<Vec<bool>>()
                == t.tile
                    .iter()
                    .map(|v| *v.first().unwrap())
                    .collect::<Vec<bool>>()
        {
            Some(TilePos::Right)
        }
        // Self left and t right
        else if self.left == None
            && t.right == None
            && self
                .tile
                .iter()
                .map(|v| v.first())
                .collect::<Vec<Option<&bool>>>()
                == t.tile
                    .iter()
                    .map(|v| v.last())
                    .collect::<Vec<Option<&bool>>>()
        {
            Some(TilePos::Left)
        } else {
            None
        }
    }
    fn new(tile: Vec<Vec<bool>>) -> Tile {
        Tile {
            tile,
            up: None,
            below: None,
            left: None,
            right: None,
            rotated: false,
        }
    }
    fn transpose(&self) -> Tile {
        let tile = (0..self.tile[0].len())
            .map(|i| {
                self.tile
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<bool>>()
            })
            .collect();
        Tile::new(tile)
    }
    fn rotate(&self, angle: Rotation) -> Tile {
        let transposed = self.transpose();
        match angle {
            Rotation::One => transposed.horizontal_flip(),
            Rotation::Two => self.rotate(Rotation::One).rotate(Rotation::One),
            Rotation::Three => self
                .rotate(Rotation::One)
                .rotate(Rotation::One)
                .rotate(Rotation::One),
        }
    }
    fn vertical_flip(&self) -> Tile {
        let mut tile = self.tile.clone();
        tile.reverse();
        Tile::new(tile)
    }
    fn horizontal_flip(&self) -> Tile {
        let mut tile = self.tile.clone();
        tile.iter_mut().for_each(|v| v.reverse());
        Tile::new(tile)
    }
    fn trim_border(&self) -> Tile {
        let l = self.tile.len() - 1;
        let tile = self.tile[1..l]
            .into_iter()
            .map(|v| v[1..l].into_iter().copied().collect())
            .collect();
        let mut new_tile = self.clone();
        new_tile.tile = tile;
        new_tile
    }
}
type Input = HashMap<usize, RefCell<Tile>>;
type Solution = (usize, Input);

fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|tile| {
            let mut inp = tile.split(":");
            let tile_n: usize = inp
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let tile = inp
                .next()
                .unwrap()
                .trim()
                .lines()
                .map(|ch| {
                    ch.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!(),
                        })
                        .collect::<Vec<bool>>()
                })
                .collect();
            (tile_n, RefCell::from(Tile::new(tile)))
        })
        .collect()
}
fn part1(inp: Input) -> Solution {
    let mut rng = rand::thread_rng();
    let base_inp = loop {
        let base_inp = inp.clone();
        let mut vals = base_inp.keys().collect_vec();
        vals.shuffle(&mut rng);
        for val1 in vals.iter() {
            let tile = base_inp.get(val1).unwrap();
            for (val, other_tile) in base_inp.iter() {
                if *val1 == val {
                    continue;
                }
                let maybe_tile_pos = tile
                    .borrow()
                    .matches_with_any_other_orientation(&other_tile.borrow());
                // Ok if this tile is to be to the right of something it can't already be on the right
                if let Some((ref v, _)) = maybe_tile_pos {
                    if base_inp
                        .values()
                        .map(|tile| match v {
                            TilePos::Right => tile.borrow().right == Some(*val),
                            TilePos::Left => tile.borrow().left == Some(*val),
                            TilePos::Up => tile.borrow().up == Some(*val),
                            TilePos::Below => tile.borrow().below == Some(*val),
                        })
                        .any(|v| v)
                    {
                        continue;
                    }
                }
                if let Some((tile_pos, new_tile)) = maybe_tile_pos {
                    match tile_pos {
                        TilePos::Up => {
                            tile.borrow_mut().up = Some(*val);
                            other_tile.borrow_mut().tile = new_tile;
                            other_tile.borrow_mut().below = Some(**val1);
                        }
                        TilePos::Below => {
                            tile.borrow_mut().below = Some(*val);
                            other_tile.borrow_mut().tile = new_tile;
                            other_tile.borrow_mut().up = Some(**val1)
                        }
                        TilePos::Right => {
                            tile.borrow_mut().right = Some(*val);
                            other_tile.borrow_mut().tile = new_tile;
                            other_tile.borrow_mut().left = Some(**val1)
                        }
                        TilePos::Left => {
                            tile.borrow_mut().left = Some(*val);
                            other_tile.borrow_mut().tile = new_tile;
                            other_tile.borrow_mut().right = Some(**val1)
                        }
                    }
                }
            }
        }

        // Do corners exist?
        let corners = base_inp
            .iter()
            .filter_map(|(pos, tile)| {
                match (
                    tile.borrow().up,
                    tile.borrow().below,
                    tile.borrow().left,
                    tile.borrow().right,
                ) {
                    (None, Some(_), None, Some(_)) => Some(*pos),
                    (None, Some(_), Some(_), None) => Some(*pos),
                    (Some(_), None, Some(_), None) => Some(*pos),
                    (Some(_), None, None, Some(_)) => Some(*pos),
                    _ => None,
                }
            })
            .count();
        if corners == 4 {
            break base_inp;
        }
    };

    // Let's find the borders of the square!
    // let top_right
    base_inp.iter().for_each(|(pos, tile)| {
        println!(
            "{}: R:{:?} L:{:?} U: {:?} B {:?}",
            pos,
            tile.borrow().right,
            tile.borrow().left,
            tile.borrow().up,
            tile.borrow().below
        );
    });
    // let a = base_inp[&1427].borrow();
    // println!("---");
    // println!(
    //     "{}: R:{:?} L:{:?} U: {:?} B {:?}",
    //     1427, a.right, a.left, a.up, a.below
    // );
    // Locate top right
    let top_right = base_inp
        .iter()
        .filter_map(|(pos, tile)| {
            match (
                tile.borrow().up,
                tile.borrow().below,
                tile.borrow().left,
                tile.borrow().right,
            ) {
                (None, Some(_), None, Some(_)) => Some(*pos),
                (None, Some(_), Some(_), None) => Some(*pos),
                (Some(_), None, Some(_), None) => Some(*pos),
                (Some(_), None, None, Some(_)) => Some(*pos),
                _ => None,
            }
        })
        .product::<usize>();
    assert_eq!(top_right, 20899048083289);
    (top_right, base_inp)
}
fn part2(inp: Input) -> usize {
    // Stack in order
    let top_left_pos = inp
        .iter()
        .filter_map(|(pos, tile)| {
            match (
                tile.borrow().up,
                tile.borrow().below,
                tile.borrow().left,
                tile.borrow().right,
            ) {
                (None, Some(_), None, Some(_)) => Some(*pos),

                _ => None,
            }
        })
        .nth(0)
        .unwrap();
    let trimmed_inp: Input = inp
        .iter()
        .map(|(l, v)| (*l, RefCell::new(v.borrow().trim_border())))
        .collect();

    let mut row = top_left_pos;
    let mut curr = top_left_pos;
    let mut vec_row = vec![];
    while let Some(tile) = inp.get(&curr) {
        vec_row.push(tile.borrow().tile.clone());
        // If there is right, right
        if let Some(r) = tile.borrow().right {
            curr = r;
        } else if let Some(b) = inp.get(&row).unwrap().borrow().below {
            row = b;
            curr = b;
        } else {
            curr = 0;
        }
        // If we can't go, then back and down
    }
    let len_of_one_block = vec_row[0].len();
    let nblocks = vec_row.len();
    // Here we would stack horizontally and vertically
    // Then do a convolution and try various orientations for the Monster, but no time

    1
}
fn main() {
    let input = parse(include_str!("../../input/day20.txt"));
    let solution = part1(input);
    println!("Part 1: {}", solution.0);
    let solution2 = part2(solution.1);
    println!("Part 2: {}", solution2);
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let input = parse(include_str!("../../input/day20.txt"));
        let to_compare = input[&2311].borrow();
        assert_eq!(to_compare.tile[0][0], false);
    }
    #[test]
    fn test_transpose() {
        let tile = Tile::new(vec![vec![true, true], vec![false, false]]);
        assert_eq!(
            tile.transpose().tile,
            vec![vec![true, false], vec![true, false]]
        );
    }
    #[test]
    fn test_rotate() {
        let tile = Tile::new(vec![vec![true, true], vec![false, false]]);
        assert_eq!(
            tile.rotate(Rotation::One).tile,
            vec![vec![false, true], vec![false, true]]
        );
        assert_eq!(
            tile.rotate(Rotation::Two).tile,
            vec![vec![false, false], vec![true, true]]
        );
        assert_eq!(
            tile.rotate(Rotation::Three).tile,
            vec![vec![true, false], vec![true, false]]
        );
    }
    #[test]
    fn test_flip_horizontal() {
        let tile = Tile::new(vec![vec![true, false], vec![true, false]]);
        assert_eq!(
            tile.horizontal_flip().tile,
            vec![vec![false, true], vec![false, true]]
        );
    }
    #[test]
    fn test_located() {
        let tile = Tile::new(vec![vec![true, true], vec![false, false]]);
        let res = tile.is_located_relative_to(&tile);
        assert_eq!(res.unwrap(), TilePos::Right);
        let other = tile.vertical_flip();
        let res = tile.is_located_relative_to(&other);
        assert_eq!(res.unwrap(), TilePos::Up);
    }
    #[test]
    fn test_trim() {
        let tile = Tile::new(vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![false, true, false],
        ]);
        let trimmed = tile.trim_border();
        assert_eq!(trimmed.tile, vec![vec![false]])
    }
}

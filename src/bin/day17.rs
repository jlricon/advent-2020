use itertools::iproduct;
use rayon::prelude::*;
use std::collections::HashMap;
const PADDING: i32 = 14;
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}
trait Neighbors {
    type Dimension;
    fn get_neighbors(&self) -> Vec<Self::Dimension>;
    fn new(x: i32, y: i32) -> Self;
    fn pad_input(map: Map<Self::Dimension>) -> Map<Self::Dimension>;
}

impl Neighbors for Point3 {
    type Dimension = Point3;
    fn new(x: i32, y: i32) -> Self {
        Point3 { x, y, z: 0 }
    }
    fn pad_input(mut map: Map<Self>) -> Map<Self> {
        for (x, y, z) in iproduct!(-PADDING..=PADDING, -PADDING..=PADDING, -PADDING..=PADDING) {
            if !map.contains_key(&Point3 { x, y, z }) {
                map.insert(Point3 { x, y, z }, CubeState::Inactive);
            }
        }
        map
    }
    fn get_neighbors(&self) -> Vec<Point3> {
        let mut res = Vec::new();
        for (i, j, k) in iproduct!(-1..=1, -1..=1, -1..=1) {
            if !(i == 0 && j == 0 && k == 0) {
                res.push(Point3 {
                    x: self.x + i,
                    y: self.y + j,
                    z: self.z + k,
                })
            }
        }
        res
    }
}

impl Neighbors for Point4 {
    type Dimension = Point4;
    fn new(x: i32, y: i32) -> Self {
        Point4 { x, y, z: 0, w: 0 }
    }
    fn get_neighbors(&self) -> Vec<Point4> {
        let mut res = Vec::new();
        for (i, j, k, l) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
            if !(i == 0 && j == 0 && k == 0 && l == 0) {
                res.push(Point4 {
                    x: self.x + i,
                    y: self.y + j,
                    z: self.z + k,
                    w: self.w + l,
                })
            }
        }
        res
    }
    fn pad_input(mut map: Map<Self>) -> Map<Self> {
        for (x, y, z, w) in iproduct!(
            -PADDING..=PADDING,
            -PADDING..=PADDING,
            -PADDING..=PADDING,
            -PADDING..=PADDING
        ) {
            if !map.contains_key(&Point4 { x, y, z, w }) {
                map.insert(Point4 { x, y, z, w }, CubeState::Inactive);
            }
        }
        map
    }
}
type Map<T> = HashMap<T, CubeState>;
fn change_cube_state<T: Neighbors<Dimension = T> + Eq + std::hash::Hash + Sync>(
    point: &T,
    state: CubeState,
    map: &Map<T>,
) -> CubeState {
    let nei = point.get_neighbors();
    let active_neigh = nei
        .par_iter()
        .filter_map(|p| match map.get(p) {
            Some(CubeState::Inactive) => None,
            Some(CubeState::Active) => Some(1),
            None => None,
        })
        .count();
    match (state, active_neigh) {
        (CubeState::Active, n) if n == 2 || n == 3 => CubeState::Active,
        (CubeState::Active, _) => CubeState::Inactive,
        (CubeState::Inactive, 3) => CubeState::Active,
        (CubeState::Inactive, _) => CubeState::Inactive,
    }
}
#[derive(Clone, Copy, Eq, PartialEq)]
enum CubeState {
    Active,
    Inactive,
}

fn part<T: Neighbors<Dimension = T> + std::hash::Hash + Eq + Copy + Sync + Send>(
    mut input: Map<T>,
) -> usize {
    input = T::pad_input(input);
    for _ in 0..6 {
        // let mut new_states = vec![];
        // Update all the points
        let new_states: Vec<(T, CubeState)> = input
            .iter()
            .par_bridge()
            .map(|(point, state)| {
                let point_state = change_cube_state(point, *state, &input);
                (*point, point_state)
            })
            .collect();
        new_states.iter().for_each(|(k, v)| {
            input.insert(*k, *v);
        });
    }

    // Let's see how it's going
    input.values().filter(|c| **c == CubeState::Active).count()
}
fn parse<T: Neighbors + std::hash::Hash + Eq>() -> Map<T> {
    include_str!("../../input/day17.txt")
        .lines()
        .enumerate()
        .map(|(row, v)| {
            v.char_indices().map(move |(col, c)| {
                (
                    T::new(col as i32, row as i32),
                    match c {
                        '.' => CubeState::Inactive,
                        '#' => CubeState::Active,
                        _ => panic!(),
                    },
                )
            })
        })
        .flatten()
        .collect()
}
fn main() {
    let input = parse::<Point3>();
    println!("Part 1: {}", part(input));
    let input = parse::<Point4>();
    println!("Part 2: {}", part(input));
}

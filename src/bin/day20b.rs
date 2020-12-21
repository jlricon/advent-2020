#[derive(Clone)]
pub struct Orientation {
    pub reflect: bool,
    pub rotate: u8,
}

impl Iterator for Orientation {
    type Item = Orientation;

    fn next(&mut self) -> Option<Orientation> {
        let result = self.clone();
        self.rotate = (self.rotate + 1) % 4;
        if self.rotate == 0 {
            self.reflect = !self.reflect
        }
        Some(result)
    }
}

impl Orientation {
    pub fn new() -> Orientation {
        Orientation {
            reflect: false,
            rotate: 0,
        }
    }

    pub fn compose(&self, other: &Orientation) -> Orientation {
        Orientation {
            reflect: self.reflect ^ other.reflect,
            rotate: (if other.reflect {
                4 - self.rotate
            } else {
                self.rotate
            } + other.rotate)
                % 4,
        }
    }

    pub fn transform(&self, (mut x, mut y): (usize, usize), size: usize) -> (usize, usize) {
        if self.reflect {
            x = size - x - 1
        }
        for _ in 0..self.rotate {
            std::mem::swap(&mut x, &mut y);
            x = size - x - 1
        }
        (x, y)
    }
}

use std::collections::HashMap;

fn main() {
    let image = build_image(
        include_str!("../../input/day20.txt")
            .split("\n\n")
            .map(parse_tile)
            .collect::<HashMap<_, _>>(),
    );

    let &(min_x, min_y) = image.keys().min().unwrap();
    let &(max_x, max_y) = image.keys().max().unwrap();
    println!(
        "{}",
        image.get(&(min_x, min_y)).unwrap().0
            * image.get(&(min_x, max_y)).unwrap().0
            * image.get(&(max_x, min_y)).unwrap().0
            * image.get(&(max_x, max_y)).unwrap().0
    );

    const MONSTER_WIDTH: usize = 20;
    const MONSTER_HEIGHT: usize = 3;
    const MONSTER: &[(usize, usize)] = &[
        (18, 0),
        (0, 1),
        (5, 1),
        (6, 1),
        (11, 1),
        (12, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (1, 2),
        (4, 2),
        (7, 2),
        (10, 2),
        (13, 2),
        (16, 2),
    ];

    let size_tile = image.values().next().unwrap().1.len() - 2;
    let size_image = size_tile * (1 + max_x - min_x) as usize;
    let monsters = Orientation::new()
        .take(8)
        .map(|o| {
            (0..=1 + size_image - MONSTER_WIDTH)
                .map(|x| {
                    (0..=1 + size_image - MONSTER_HEIGHT)
                        .filter(|y| {
                            MONSTER.iter().all(|&(ox, oy)| {
                                let (x, y) = o.transform((x + ox, y + oy), size_image);
                                let (_, t, o) = image
                                    .get(&(
                                        min_x + (x / size_tile) as i32,
                                        min_y + (y / size_tile) as i32,
                                    ))
                                    .unwrap();
                                let (x, y) = o.transform(
                                    (1 + x % size_tile, 1 + y % size_tile),
                                    size_tile + 2,
                                );

                                t[y][x]
                            })
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .max()
        .unwrap();

    println!(
        "{}",
        image
            .values()
            .flat_map(|(_, t, _)| t[1..=size_tile].iter())
            .flat_map(|r| r[1..=size_tile].iter())
            .filter(|&&x| x)
            .count()
            - monsters * MONSTER.len()
    );
}

type Tile = Vec<Vec<bool>>;

fn parse_tile(s: &str) -> (u64, Tile) {
    let mut it = s.lines();

    (
        it.next()
            .and_then(|l| l[5..l.len() - 1].parse().ok())
            .unwrap(),
        it.map(|l| l.bytes().map(|b| b == b'#').collect()).collect(),
    )
}

fn build_image(mut tiles: HashMap<u64, Tile>) -> HashMap<(i32, i32), (u64, Tile, Orientation)> {
    let mut image = HashMap::new();
    let mut queue = vec![(0, 0)];

    while let Some((x, y)) = queue.pop() {
        if image.contains_key(&(x, y)) {
            continue;
        }

        if let Some((id, o)) = tiles.iter().find_map(|(&id, t1)| {
            Orientation::new()
                .take(8)
                .find(|o1| {
                    image
                        .get(&(x - 1, y))
                        .map(|(_, t2, o2)| match_left_right(t2, o2, t1, o1))
                        .unwrap_or(true)
                        && image
                            .get(&(x + 1, y))
                            .map(|(_, t2, o2)| match_left_right(t1, o1, t2, o2))
                            .unwrap_or(true)
                        && image
                            .get(&(x, y - 1))
                            .map(|(_, t2, o2)| match_above_below(t2, o2, t1, o1))
                            .unwrap_or(true)
                        && image
                            .get(&(x, y + 1))
                            .map(|(_, t2, o2)| match_above_below(t1, o1, t2, o2))
                            .unwrap_or(true)
                })
                .map(|o| (id, o))
        }) {
            if let Some(t) = tiles.remove(&id) {
                image.insert((x, y), (id, t, o));
                if tiles.is_empty() {
                    break;
                }
                queue.extend(vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)])
            }
        }
    }

    image
}

fn match_first_row(t1: &Tile, o1: &Orientation, t2: &Tile, o2: &Orientation) -> bool {
    let size = t1.len();

    (0..size).all(|x| {
        let (x1, y1) = o1.transform((x, 0), size);
        let (x2, y2) = o2.transform((x, 0), size);

        t1[y1][x1] == t2[y2][x2]
    })
}

fn match_above_below(t1: &Tile, o1: &Orientation, t2: &Tile, o2: &Orientation) -> bool {
    match_first_row(
        t1,
        &Orientation {
            reflect: true,
            rotate: 2,
        }
        .compose(o1),
        t2,
        o2,
    )
}

fn match_left_right(t1: &Tile, o1: &Orientation, t2: &Tile, o2: &Orientation) -> bool {
    match_first_row(
        t1,
        &Orientation {
            reflect: false,
            rotate: 1,
        }
        .compose(o1),
        t2,
        &Orientation {
            reflect: true,
            rotate: 3,
        }
        .compose(o2),
    )
}

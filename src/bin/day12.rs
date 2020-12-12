const DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];
const DIRECTIONS_BACK: [char; 4] = ['N', 'W', 'S', 'E'];
fn relative_advance(facing: char, degrees: i64, counter_clock: bool) -> char {
    // Maps a facing, a rotation direction
    let angle: i64 = (degrees % 360) / 90;
    *if !counter_clock {
        DIRECTIONS.iter()
    } else {
        DIRECTIONS_BACK.iter()
    }
    .cycle()
    .skip_while(|e| **e != facing)
    .skip((angle) as usize)
    .nth(0)
    .unwrap()
}
fn rotate_waypoint_around_ship(
    ship_pos: (i64, i64),
    waypoint: (i64, i64),
    inst: i64,
    clockwise: bool,
) -> (i64, i64) {
    let mut angle = inst as f32;
    angle = angle.to_radians();
    if clockwise {
        angle = -angle;
    }

    let s = angle.sin() as i64;
    let c = angle.cos() as i64;
    let px = c * (waypoint.0 - ship_pos.0) - s * (waypoint.1 - ship_pos.1) + ship_pos.0;
    let py = s * (waypoint.0 - ship_pos.0) + c * (waypoint.1 - ship_pos.1) + ship_pos.1;

    (px, py)
}
fn relative_move(facing: char, much: i64, position: (i64, i64)) -> (i64, i64) {
    let p = match facing {
        'N' => (0, much),
        'S' => (0, -much),
        'E' => (much, 0),
        'W' => (-much, 0),
        _ => panic!(),
    };
    (p.0 + position.0, p.1 + position.1)
}
fn relative_move_towards_waypoint(waypoint: (i64, i64), much: i64, boat: (i64, i64)) -> (i64, i64) {
    ((waypoint.0 - boat.0) * much, (waypoint.1 - boat.1) * much)
}
fn manhattan(position: (i64, i64)) -> i64 {
    position.0.abs() + position.1.abs()
}
fn part1(input: &Vec<(char, i64)>) {
    let mut position = (0, 0);
    let mut facing = 'E';
    for instru in input {
        match instru {
            ('N', n) => position = (position.0, position.1 + n),
            ('S', n) => position = (position.0, position.1 - n),
            ('E', n) => position = (position.0 + n, position.1),
            ('W', n) => position = (position.0 - n, position.1),
            ('L', n) => facing = relative_advance(facing, *n, true),
            ('R', n) => facing = relative_advance(facing, *n, false),
            ('F', n) => position = relative_move(facing, *n, position),
            _ => panic!(),
        }
    }
    assert_eq!(manhattan(position), 2458);
    println!("Part 1: {}", manhattan(position));
}
fn part2(input: &Vec<(char, i64)>) {
    let mut boat_position = (0, 0);
    let mut position = (10, 1);
    // let mut facing = 'E';
    for instru in input {
        match instru {
            ('N', n) => position = (position.0, position.1 + n),
            ('S', n) => position = (position.0, position.1 - n),
            ('E', n) => position = (position.0 + n, position.1),
            ('W', n) => position = (position.0 - n, position.1),
            ('L', n) => position = rotate_waypoint_around_ship(boat_position, position, *n, false),
            ('R', n) => position = rotate_waypoint_around_ship(boat_position, position, *n, true),
            ('F', n) => {
                let diff = relative_move_towards_waypoint(position, *n, boat_position);
                boat_position = (boat_position.0 + diff.0, boat_position.1 + diff.1);
                position = (position.0 + diff.0, position.1 + diff.1);
            }
            _ => panic!(),
        }
    }
    assert_eq!(manhattan(boat_position), 145117);
    println!("Part 2: {}", manhattan(boat_position));
}

fn main() {
    let input: Vec<(char, i64)> = include_str!("../../input/day12.txt")
        .lines()
        .map(|m| {
            let mut c = m.chars();
            let letter = c.next().unwrap();
            let number: i64 = c.collect::<String>().parse().unwrap();
            (letter, number)
        })
        .collect();
    part1(&input);
    part2(&input);
}

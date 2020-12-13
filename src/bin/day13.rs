#[derive(Debug, Eq, PartialEq)]
enum BusState {
    OutOfService,
    Bus(u64),
}
fn main() {
    let mut input = include_str!("../../input/day13.txt").lines();
    let earliest: u64 = input.next().unwrap().parse().unwrap();
    let notes: Vec<BusState> = input
        .next()
        .unwrap()
        .split(',')
        .map(|c| match c {
            "x" => BusState::OutOfService,
            n => BusState::Bus(n.parse().unwrap()),
        })
        .collect();
    let available_buses: Vec<(u64, u64)> = notes
        .iter()
        .filter_map(|n| match n {
            BusState::OutOfService => None,
            BusState::Bus(v) => Some((*v, v * (earliest).div_euclid(*v) + v)),
        })
        .collect();
    let part1 = available_buses
        .iter()
        .min_by_key(|c| c.1)
        .map(|v| (v.1 - earliest) * v.0)
        .unwrap();
    assert_eq!(part1, 3215);
    println!("Part 1: {}", part1);
    // Need to check that at one timestamp the offsets are right
    let prepared: Vec<(usize, u64)> = notes
        .iter()
        .enumerate()
        .filter_map(|(pos, n)| match n {
            BusState::OutOfService => None,
            BusState::Bus(v) => Some((pos, *v)),
        })
        .collect();

    let times_for_t = |t: u64| -> bool {
        prepared
            .iter()
            .map(|(pos, v)| (t + *pos as u64) % v == 0)
            .all(|v| v)
    };

    let candidate_n = prepared.iter().max_by_key(|v| v.1).unwrap();

    let mut t = 134589502019;
    loop {
        let try_n = candidate_n.1.checked_mul(t).unwrap() - candidate_n.0 as u64;
        if times_for_t(try_n) {
            println!("Part 2: {}", try_n);
            break;
        }

        t += 1;
    }
}

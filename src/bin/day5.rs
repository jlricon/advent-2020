use itertools::Itertools;

fn seat_id(row: u32, col: u32) -> u32 {
    row * 8 + col
}
// Binary 101
fn main() {
    let input = include_str!("../../input/day5.txt");
    let all_ids: Vec<u32> = input
        .lines()
        .map(|l| {
            let row_strng: String = l
                .chars()
                .take(7)
                .map(|c| match c {
                    'F' => '0',
                    'B' => '1',
                    _ => panic!(),
                })
                .collect();
            let row = u32::from_str_radix(&row_strng, 2).unwrap();
            let col_string: String = l
                .chars()
                .skip(7)
                .map(|c| match c {
                    'L' => '0',
                    'R' => '1',
                    _ => panic!(),
                })
                .collect();
            let col = u32::from_str_radix(&col_string, 2).unwrap();
            seat_id(row, col)
        })
        .collect();
    let part1 = all_ids.iter().max().unwrap();

    let mut sorted_ids = all_ids.iter().collect::<Vec<&u32>>();
    sorted_ids.sort();
    let part2_pos: usize = sorted_ids
        .iter()
        .tuple_windows()
        .map(|(prev, next)| **next as i32 - **prev as i32)
        .enumerate()
        .filter(|(_, n)| *n != 1)
        .map(|(pos, _n)| pos as usize)
        .next()
        .unwrap();

    let part2 = sorted_ids[part2_pos] + 1;

    println!("{}", part1);
    println!("{:?}", &part2);
}

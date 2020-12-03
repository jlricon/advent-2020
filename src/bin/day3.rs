use std::collections::HashMap;
#[derive(Debug)]
enum GridType {
    Tree,
    Open,
}
impl From<char> for GridType {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Tree,
            '.' => Self::Open,
            _ => panic!(),
        }
    }
}
type Coords = (usize, usize);
fn slide_down_with_wraparound(coord: Coords, ncols: usize, coord_shift: Coords) -> Coords {
    (coord.0 + coord_shift.0, (coord.1 + coord_shift.1) % ncols)
}

fn count_trees_for_slope(map: &HashMap<Coords, GridType>, ncols: usize, slope: Coords) -> u64 {
    let mut current_pos = (0, 0);
    let mut ntrees = 0;
    while let Some(grid_type) = map.get(&current_pos) {
        current_pos = slide_down_with_wraparound(current_pos, ncols, slope);
        match grid_type {
            GridType::Tree => ntrees += 1,
            _ => (),
        }
    }
    ntrees
}
fn part1(map: &HashMap<Coords, GridType>, ncols: usize) {
    let ntrees = count_trees_for_slope(map, ncols, (1, 3));
    println!("{}", ntrees);
}
fn part2(map: &HashMap<Coords, GridType>, ncols: usize) {
    let slopes_to_test: [Coords; 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let tree_prod: u64 = slopes_to_test
        .iter()
        .map(|slopes| count_trees_for_slope(map, ncols, *slopes))
        .product();
    println!("{}", tree_prod);
}
fn main() {
    let mut map: HashMap<Coords, GridType> = HashMap::new();
    let mut nrows = 0;
    let input = include_str!("../../input/day3.txt");
    input.lines().enumerate().for_each(|(row, ch)| {
        ch.chars().enumerate().for_each(|(col, val)| {
            map.insert((row, col), val.into());
        });
        nrows += 1;
    });
    let ncols = input.lines().next().unwrap().len();

    part1(&map, ncols);
    part2(&map, ncols);
}
#[cfg(test)]
mod test {
    use super::slide_down_with_wraparound;
    #[test]
    fn test_slide_down() {
        let slide = (1, 3);
        assert_eq!(slide_down_with_wraparound((0, 0), 1, slide), (1, 0));
        assert_eq!(slide_down_with_wraparound((0, 0), 2, slide), (1, 1));
        assert_eq!(slide_down_with_wraparound((0, 0), 3, slide), (1, 0));
        assert_eq!(slide_down_with_wraparound((0, 0), 4, slide), (1, 3));
    }
}

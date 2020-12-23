use std::collections::VecDeque;

fn main() {
    //let mut inp: VecDeque<usize> = [3, 8, 9, 1, 2, 5, 4, 6, 7].iter().copied().collect();
    let mut inp: Vec<usize> = [4, 7, 6, 1, 3, 8, 2, 5, 9].iter().copied().collect();
    let og_input = inp.clone();
    let max = *inp.iter().max().unwrap();
    let og_len = inp.len();
    let mut popped = [0, 0, 0];
    // Let's add a million here

    inp.extend(max..1_000_000);

    let min = *inp.iter().min().unwrap();
    let max = *inp.iter().max().unwrap();
    let mut current_cup: usize = inp[0];
    for n in 1..=10_000_000 {
        if n % 100_000 == 0 {
            println!("Done {}", n);
        }
        // println!("-- move {} --", n);
        //  println!("cups: {:?}, current {}", inp, current_cup);
        //  assert_eq!(inp.len(), 1_000_000);
        // Find current cup, take 3 clockwise
        inp.iter()
            .cycle()
            .skip_while(|v| **v != current_cup)
            .skip(1)
            .take(3)
            .enumerate()
            .for_each(|(pos, v)| popped[pos] = *v);

        let mut before_current: VecDeque<usize> = inp
            .iter()
            .cycle()
            .take_while(|v| **v != current_cup)
            .copied()
            .collect();
        // dbg!(&before_current);
        let remaining_elems: usize = {
            let p = inp.len() as i32 - (3 + before_current.len()) as i32 - 1;
            if p >= 0 {
                p as usize
            } else {
                (0..p.abs()).for_each(|_| {
                    before_current.pop_front();
                });
                0
            }
        };
        //
        //   dbg!(remaining_elems);
        let after_current = inp
            .iter()
            .cycle()
            .skip_while(|v| **v != current_cup)
            .skip(4)
            .take(remaining_elems);

        // dbg!(&popped, &before_current, &after_current);

        let mut candidate = current_cup - 1;
        let destination_cup = loop {
            if candidate < min {
                candidate = max;
            }
            if !(popped[0] == candidate || popped[1] == candidate || popped[2] == candidate) {
                break candidate;
            } else {
                candidate -= 1;
            }
        };
        // dbg!(candidate);

        let mut new_input: VecDeque<usize> = before_current
            .iter()
            .chain([current_cup].iter())
            .chain(after_current)
            .copied()
            .collect();

        //   dbg!(&new_input, &destination_cup);
        let destination_position = new_input
            .iter()
            .enumerate()
            .skip_while(|(_, v)| **v != destination_cup)
            .take(1)
            .map(|(p, _)| p)
            .nth(0)
            .unwrap();
        //dbg!(destination_position);

        popped
            .iter()
            .enumerate()
            .for_each(|(pos, v)| new_input.insert(destination_position + pos + 1, *v));
        //  dbg!(&new_input);
        inp = new_input.into();

        current_cup = *inp
            .iter()
            .cycle()
            .skip_while(|c| **c != current_cup)
            .skip(1)
            .take(1)
            .nth(0)
            .unwrap();
    }
    // Let's do the order
    let part1 = inp
        .iter()
        .cycle()
        .skip_while(|v| **v != 1)
        .skip(1)
        .take(og_len - 1)
        .enumerate()
        .map(|(_, val)| format!("{}", val))
        .collect::<String>();
    println!("Part 1: {}", part1);
    let part2: usize = inp
        .iter()
        .cycle()
        .skip_while(|v| **v != 1)
        .skip(1)
        .take(2)
        .product();
    println!("Part 2: {}", part2);
}

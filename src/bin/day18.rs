#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Sum,
    Prod,
    Val(u64),
    RightParen,
    LeftParen,
}
impl Op {
    fn operate(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Sum => a + b,
            Op::Prod => a * b,
            _ => panic!(),
        }
    }
}
fn compute(ops: &Vec<Op>) -> u64 {
    use Op::*;
    let mut acc = 0;
    let mut clos = Op::Sum;
    let mut skip_pos: usize = 0;
    for (pos, op) in ops.iter().enumerate() {
        if (pos <= skip_pos) && skip_pos > 0 {
            continue;
        }

        match op {
            Sum => clos = Sum,
            Prod => clos = Prod,
            Val(n) => acc = clos.operate(acc, *n),
            LeftParen => {
                let inner_op = &ops
                    .iter()
                    .skip(pos + 1)
                    // Let's count parens and stop when they are balanced
                    .scan(vec![1, 0], |state, value| {
                        match value {
                            LeftParen => state[0] += 1,
                            RightParen => state[1] += 1,
                            _ => (),
                        };
                        Some((state.clone(), *value))
                    })
                    .take_while(|(paren_count, _)| paren_count[0] != paren_count[1])
                    .map(|(_, value)| value)
                    .collect();
                acc = clos.operate(acc, compute(inner_op));
                skip_pos = pos + inner_op.len() + 1;
            }
            RightParen => panic!(),
        }
    }
    acc
}
fn parse(line: &str) -> Vec<Op> {
    line.replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|c| match c {
            "+" => Op::Sum,
            "*" => Op::Prod,
            "(" => Op::LeftParen,
            ")" => Op::RightParen,
            n => Op::Val(n.parse().unwrap()),
        })
        .collect()
}
fn main() {
    let input: Vec<Vec<Op>> = include_str!("../../input/day18.txt")
        .lines()
        .map(parse)
        .collect();
    let sum: u64 = input.iter().map(|c| compute(c)).sum();
    assert_eq!(sum, 209335026987);
    println!("Part1: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let test = parse("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(compute(&test), 71);
        let test = parse("2 * 3 + (4 * 5)");
        assert_eq!(compute(&test), 26);
        let test = parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(compute(&test), 12240);
        let test = parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(compute(&test), 13632);
        let test = parse("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(compute(&test), 51);
        let test = parse("(((((((1 + (1 + 1) * 1))))))");
        assert_eq!(compute(&test), 3);
        let test=parse("(9 + 3 + 3 + 3) + (2 + (6 * 2 * 3) + 5 * 5) + 3 + (2 * 3 * 6 * 4 * 3 * (7 + 4)) * 6 * 6");
        assert_eq!(compute(&test), 179568);
    }
}

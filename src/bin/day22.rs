use std::{collections::hash_map::DefaultHasher, hash::Hash};
use std::{collections::HashSet, collections::VecDeque, hash::Hasher};

#[derive(Debug, Clone)]
struct Input {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
    // Use a crap hash
    prev_games: HashSet<u64>,
}
impl<'a> Input {
    fn is_over(&self) -> bool {
        if self.player1.len() == 0 || self.player2.len() == 0 {
            true
        } else {
            false
        }
    }
    fn hash_game(&self) -> u64 {
        let mut hash = DefaultHasher::new();
        let mut hash2 = DefaultHasher::new();
        self.player1.hash(&mut hash);
        self.player2.hash(&mut hash2);
        hash.finish() * hash2.finish()
    }
    fn add_self_player(&mut self) {
        self.prev_games.insert(self.hash_game());
    }
    fn get_recursive_deck(&self, p1cards: u8, p2cards: u8) -> Input {
        Input {
            player1: self
                .player1
                .iter()
                .take(p1cards as usize)
                .copied()
                .collect(),
            player2: self
                .player2
                .iter()
                .take(p2cards as usize)
                .copied()
                .collect(),
            prev_games: self.prev_games.clone(),
        }
    }
    fn fancy_sum(player: &VecDeque<u8>) -> u32 {
        player
            .iter()
            .rev()
            .enumerate()
            .map(|(pos, val)| (1 + pos) as u32 * *val as u32)
            .sum::<u32>()
    }
    fn get_winner_score(&self) -> u32 {
        if self.player1.len() == 0 {
            // Player 2 won
            Input::fancy_sum(&self.player2)
        } else {
            Input::fancy_sum(&self.player1)
        }
    }
    fn been_here_before(&self) -> bool {
        if self.prev_games.contains(&self.hash_game()) {
            true
        } else {
            false
        }
    }
    fn construct_player<T: Iterator<Item = &'a str>>(mut inp: T) -> VecDeque<u8> {
        inp.nth(0)
            .unwrap()
            .split('\n')
            .skip(1)
            .map(|l| l.trim().parse::<u8>().unwrap())
            .collect()
    }
}
fn part1(mut input: Input) {
    while !input.is_over() {
        let p1card = input.player1.pop_front().unwrap();
        let p2card = input.player2.pop_front().unwrap();
        // println!("Player 1 plays: {}| Player 2 plays: {}", p1card, p2card);
        if p1card > p2card {
            input.player1.push_back(p1card);
            input.player1.push_back(p2card);
        } else {
            input.player2.push_back(p2card);
            input.player2.push_back(p1card);
        }
    }

    println!("Part 1: {}", input.get_winner_score());
}
type PlayerOneWon = bool;

fn play_round(mut input: Input) -> (PlayerOneWon, Input) {
    loop {
        if input.been_here_before() {
            break (true, input);
        }
        // Add this round to the previous rounds
        input.add_self_player();
        let (p1card, p2card) = match (input.player1.iter().nth(0), input.player2.iter().nth(0)) {
            (None, Some(_)) => break (false, input),
            (Some(_), None) => break (true, input),
            (Some(u1), Some(u2)) => (*u1, *u2),
            (None, None) => panic!(),
        };
        input.player1.pop_front();
        input.player2.pop_front();
        // Determine who won the match
        let player_one_won =
            if input.player1.len() >= p1card as usize && input.player2.len() >= p2card as usize {
                // Recursion game
                let (player_one_won, _) = play_round(input.get_recursive_deck(p1card, p2card));
                player_one_won
            } else {
                if p1card > p2card {
                    true
                } else {
                    false
                }
            };
        // Put back the cards after the match
        if player_one_won {
            input.player1.push_back(p1card);
            input.player1.push_back(p2card);
        } else {
            input.player2.push_back(p2card);
            input.player2.push_back(p1card);
        }
    }
}
fn part2(input: Input) {
    let res = play_round(input);
    let summed = res.1.get_winner_score();
    println!("Part 2: {}", summed);
}
fn main() {
    let mut inp = include_str!("../../input/day22.txt").split("\n\n");
    let player1: VecDeque<u8> = Input::construct_player(&mut inp);
    let player2: VecDeque<u8> = Input::construct_player(&mut inp);
    let og_game = HashSet::new();

    let input = Input {
        player1: player1,
        player2: player2,
        prev_games: og_game,
    };
    part1(input.clone());
    part2(input);
}

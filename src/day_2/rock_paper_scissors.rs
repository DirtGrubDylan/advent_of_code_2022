#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn new(move_def: char) -> Move {
        match move_def {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("Cannot handle move_def: {}", move_def),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn new(move_def: char) -> Outcome {
        match move_def {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Cannot handle move_def: {}", move_def),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Round {
    opponent_move: Move,
    player_move: Move,
    expected_outcome: Outcome,
}

impl Round {
    pub fn new(move_defs: &str) -> Round {
        let moves: Vec<char> = move_defs
            .split(' ')
            .filter_map(|s| s.chars().nth(0))
            .collect();

        assert_eq!(moves.len(), 2);

        Round {
            opponent_move: Move::new(moves[0]),
            player_move: Move::new(moves[1]),
            expected_outcome: Outcome::new(moves[1]),
        }
    }

    pub fn get_score(&self) -> u32 {
        Round::score_of(self.opponent_move, self.player_move)
    }

    pub fn get_expected_output_score(&self) -> u32 {
        Round::expected_score_of(self.opponent_move, self.expected_outcome)
    }

    fn score_of(opponent_move: Move, player_move: Move) -> u32 {
        let outcome = Round::get_outcome(opponent_move, player_move);

        (player_move as u32) + (outcome as u32)
    }

    fn expected_score_of(opponent_move: Move, expected_outcome: Outcome) -> u32 {
        let player_move = Round::get_player_move(opponent_move, expected_outcome);

        (player_move as u32) + (expected_outcome as u32)
    }

    fn get_outcome(opponent_move: Move, player_move: Move) -> Outcome {
        match (opponent_move, player_move) {
            (Move::Rock, Move::Paper) => Outcome::Win,
            (Move::Paper, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Rock) => Outcome::Win,
            (x, y) if x == y => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    fn get_player_move(opponent_move: Move, expected_outcome: Outcome) -> Move {
        match (opponent_move, expected_outcome) {
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_new() {
        let input = vec!['A', 'B', 'C', 'X', 'Y', 'Z'];

        let expected = vec![
            Move::Rock,
            Move::Paper,
            Move::Scissors,
            Move::Rock,
            Move::Paper,
            Move::Scissors,
        ];

        let result: Vec<Move> = input.into_iter().map(|def| Move::new(def)).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_outcome_new() {
        let input = vec!['X', 'Y', 'Z'];

        let expected = vec![Outcome::Lose, Outcome::Draw, Outcome::Win];

        let result: Vec<Outcome> = input.into_iter().map(|def| Outcome::new(def)).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn round_new() {
        let expected = Round {
            opponent_move: Move::Rock,
            player_move: Move::Paper,
            expected_outcome: Outcome::Draw,
        };

        let result = Round::new("A Y");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_round_get_score() {
        let expected = vec![8, 1, 6];

        let result = vec![
            Round::score_of(Move::Rock, Move::Paper),
            Round::score_of(Move::Paper, Move::Rock),
            Round::score_of(Move::Scissors, Move::Scissors),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_round_get_expected_outcome_score() {
        let expected = vec![4, 1, 7];

        let result = vec![
            Round::expected_score_of(Move::Rock, Outcome::Draw),
            Round::expected_score_of(Move::Paper, Outcome::Lose),
            Round::expected_score_of(Move::Scissors, Outcome::Win),
        ];

        assert_eq!(result, expected);
    }
}

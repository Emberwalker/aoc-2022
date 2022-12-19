use crate::prelude::*;

#[derive(Debug, clap::Args)]
pub struct Args {
    file: std::path::PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Move {
    fn score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }

    fn fight(self, opponent_move: Move) -> Outcome {
        if self == opponent_move {
            Outcome::Draw
        } else if self as u8 == (opponent_move as u8 + 1) % 3 {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

impl TryFrom<char> for Move {
    type Error = Box<dyn std::error::Error>;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => bail!("Invalid action: {}", ch),
        }
    }
}

impl TryFrom<u8> for Move {
    type Error = Box<dyn std::error::Error>;

    fn try_from(ord: u8) -> Result<Self, Self::Error> {
        match ord {
            0 => Ok(Move::Rock),
            1 => Ok(Move::Paper),
            2 => Ok(Move::Scissors),
            _ => bail!("Invalid action: {}", ord),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = Box<dyn std::error::Error>;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => bail!("Invalid action: {}", ch),
        }
    }
}

pub fn run_a(args: Args) -> RootErr {
    run(args, resolve_line_a)
}

pub fn run_b(args: Args) -> RootErr {
    run(args, resolve_line_b)
}

pub fn run(args: Args, resolver: impl Fn(char, char) -> ErrWrapper<u32>) -> RootErr {
    debug!("Args: {:?}", args);
    let line_src = files::read_lines(args.file)?;
    let mut total_score = 0;
    for line in line_src {
        let ln = line?;
        if ln.len() != 3 {
            warn!("Line of wrong length: {}", ln)
        } else {
            total_score += resolver(ln.chars().nth(0).unwrap(), ln.chars().nth(2).unwrap())?
        }
    }

    info!("Total score for strategy guide: {}", total_score);

    Ok(())
}

fn resolve_line_a(opponent: char, player: char) -> ErrWrapper<u32> {
    let player_move = Move::try_from(player)?;
    let opponent_move = Move::try_from(opponent)?;

    let outcome = player_move.fight(opponent_move);
    Ok(player_move.score() + outcome.score())
}

fn resolve_line_b(opponent: char, outcome_raw: char) -> ErrWrapper<u32> {
    let opponent_move = Move::try_from(opponent)?;
    let outcome = Outcome::try_from(outcome_raw)?;

    let mut player_move_ord = (opponent_move as i8 + match Outcome::try_from(outcome)? {
        Outcome::Lose => -1,
        Outcome::Draw => 0,
        Outcome::Win => 1,
    }) % 3;
    if player_move_ord < 0 {
        player_move_ord = 3 + player_move_ord;
    }
    let player_move = Move::try_from(player_move_ord as u8)?;

    Ok(player_move.score() + outcome.score())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fight_draw() {
        assert_eq!(Outcome::Draw, Move::Rock.fight(Move::Rock))
    }

    #[test]
    fn fight_win() {
        assert_eq!(Outcome::Win, Move::Paper.fight(Move::Rock))
    }

    #[test]
    fn fight_lose() {
        assert_eq!(Outcome::Lose, Move::Scissors.fight(Move::Rock))
    }

    #[test]
    fn resolve_line_a1() {
        assert_eq!(8, resolve_line_a('A', 'Y').unwrap())
    }

    #[test]
    fn resolve_line_a2() {
        assert_eq!(1, resolve_line_a('B', 'X').unwrap())
    }

    #[test]
    fn resolve_line_a3() {
        assert_eq!(6, resolve_line_a('C', 'Z').unwrap())
    }

    #[test]
    fn resolve_line_b1() {
        assert_eq!(4, resolve_line_b('A', 'Y').unwrap())
    }

    #[test]
    fn resolve_line_b2() {
        assert_eq!(1, resolve_line_b('B', 'X').unwrap())
    }

    #[test]
    fn resolve_line_b3() {
        assert_eq!(7, resolve_line_b('C', 'Z').unwrap())
    }
}

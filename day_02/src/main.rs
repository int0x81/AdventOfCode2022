use utils::file_io::FileReader;

#[derive(PartialEq)]
enum GameInput {
    Rock,
    Paper,
    Scissors
}

enum GameOutcome {
    Loss,
    Draw,
    Win
}

fn transform_input(code: &char) -> GameInput {

    match code {
        'A' => GameInput::Rock,
        'B' => GameInput::Paper,
        'C' => GameInput::Scissors,
        'X' => GameInput::Rock,
        'Y' => GameInput::Paper,
        'Z' => GameInput::Scissors,
        _ => panic!("Invalid game input")
    }
}

fn transform_outcome(code: &char) -> GameOutcome {

    match code {
        'X' => GameOutcome::Loss,
        'Y' => GameOutcome::Draw,
        'Z' => GameOutcome::Win,
        _ => panic!("Invalid game outcome")
    }
}

fn calculate_game_outcome(opponents_input: &GameInput, own_input: &GameInput) -> GameOutcome {
    
    match opponents_input {
        GameInput::Rock => {
            match own_input {
                GameInput::Rock => { GameOutcome::Draw }
                GameInput::Paper => { GameOutcome::Win }
                GameInput::Scissors => { GameOutcome::Loss }
            }
        },
        GameInput::Paper => {
            match own_input {
                GameInput::Rock => { GameOutcome::Loss }
                GameInput::Paper => { GameOutcome::Draw }
                GameInput::Scissors => { GameOutcome::Win }
            }
        },
        GameInput::Scissors => {
            match own_input {
                GameInput::Rock => { GameOutcome::Win }
                GameInput::Paper => { GameOutcome::Loss }
                GameInput::Scissors => { GameOutcome::Draw }
            }
        }
    }
}

fn calculate_own_input(opponents_input: &GameInput, outcome: &GameOutcome) -> GameInput {

    match opponents_input {
        GameInput::Rock => {
            match outcome {
                GameOutcome::Loss => { GameInput::Scissors }
                GameOutcome::Draw => { GameInput::Rock }
                GameOutcome::Win => { GameInput::Paper }
            }
        }
        GameInput::Paper => {
            match outcome {
                GameOutcome::Loss => { GameInput::Rock }
                GameOutcome::Draw => { GameInput::Paper }
                GameOutcome::Win => { GameInput::Scissors}
            }
        }
        GameInput::Scissors => {
            match outcome {
                GameOutcome::Loss => { GameInput::Paper }
                GameOutcome::Draw => { GameInput::Scissors }
                GameOutcome::Win => { GameInput::Rock }
            }
        }
    }
}

fn calculate_score(own_input: &GameInput, outcome: &GameOutcome) -> usize {
    let input_score: usize = match own_input {
        GameInput::Rock => 1,
        GameInput::Paper => 2,
        GameInput::Scissors => 3
    };
    let outcome_score: usize = match outcome {
        GameOutcome::Loss => 0,
        GameOutcome::Draw => 3,
        GameOutcome::Win => 6
    };

    input_score + outcome_score
}

#[allow(dead_code)] // only used in the first part of the challenge
fn calculate_round_score(line: &str) -> usize {

    let opponents_play: GameInput = transform_input(&line.chars().nth(0).unwrap());
    let own_play: GameInput = transform_input(&line.chars().nth(2).unwrap());

    let outcome: GameOutcome = calculate_game_outcome(&opponents_play, &own_play);

    calculate_score(&own_play, &outcome)
}

fn calculate_round_score_02(line: &str) -> usize {

    let opponents_input: GameInput = transform_input(&line.chars().nth(0).unwrap());
    let outcome: GameOutcome = transform_outcome(&line.chars().nth(2).unwrap());

    let own_input: GameInput = calculate_own_input(&opponents_input, &outcome);

    calculate_score(&own_input, &outcome)
}

#[allow(dead_code)] // only used in the first part of the challenge
fn determine_total_score(file_reader: FileReader) -> usize {
    
    let mut total_score = 0;

    for line in file_reader {
        let round_score = calculate_round_score(&line);
        total_score += round_score;
    }

    total_score
}

fn determine_total_score_02(file_reader: FileReader) -> usize {

    let mut total_score = 0;

    for line in file_reader {
        let round_score = calculate_round_score_02(&line);
        total_score += round_score;
    }

    total_score
}

fn main() {
    let file_reader: FileReader = FileReader::new("day_02/data/puzzle_input.txt");
    let total_score = determine_total_score_02(file_reader);
    println!("{}", total_score);
}

#[cfg(test)]
mod tests {
    use utils::file_io::FileReader;
    use crate::{determine_total_score, determine_total_score_02};

    #[test]
    fn should_determine_correct_total_score() {

        // Arrange
        let file_reader: FileReader = FileReader::new("day_02/data/test.txt");

        // Act
        let total_score = determine_total_score(file_reader);

        // Assert
        assert_eq!(15, total_score);
    }

    #[test]
    fn should_determine_correct_total_score_02() {

        // Arrange
        let file_reader: FileReader = FileReader::new("day_02/data/test.txt");

        // Act
        let total_score = determine_total_score_02(file_reader);

        // Assert
        assert_eq!(12, total_score);
    }
}
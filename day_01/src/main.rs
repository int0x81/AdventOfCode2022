use std::path::{Path, PathBuf};
use std::{fs::File, io::BufReader};
use std::io::prelude::*;

fn get_file_path(file: &str) -> String {
    let current_dir: PathBuf = std::env::current_dir().expect("Unable to get current directory");
    let full_path: PathBuf = current_dir.join(file);
    let full_path: &Path = full_path.as_path();
    full_path.to_str().expect("Unable to convert file path to string").to_string()
}

fn read_in_puzzle_input(file: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::<String>::new();

    let path = get_file_path(file);
    let file: File = File::open(path).expect("Unable to open file");
    let buffer_reader: BufReader<File> = BufReader::new(file);

    for line in buffer_reader.lines() {
        let line: String = line.expect("Unable to read line in file");
        lines.push(line);
    }

    lines
}

fn is_list_seperator(input_string: &str) -> bool {
    input_string == ""
}

fn create_initialized_vector_with_fixed_capacity<T: Default>(capacity: usize) -> Vec<T> {
    let mut vector = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        vector.push(T::default());
    }

    vector
}

fn check_and_set_higher_amount(calories_buffer: &usize, highest_calories_amount: &mut usize) {
    if calories_buffer > highest_calories_amount {
        *highest_calories_amount = *calories_buffer;
    }
}

fn check_and_align_higher_amounts(calories_buffer: &usize, highest_calories_amounts: &mut Vec<usize>) {

    for index in 0..highest_calories_amounts.capacity() {
        let entry = highest_calories_amounts[index];

        if entry < *calories_buffer {
            highest_calories_amounts.pop();
            highest_calories_amounts.insert(index, *calories_buffer);
            return
        }
    }
}

fn determine_highest_single_carried_calories_amount(calories_table: &Vec<String>) -> usize {
    
    let mut calories_buffer: usize = 0;
    let mut highest_calories_amount: usize = 0;

    for amount in calories_table {

        if is_list_seperator(amount) {
            check_and_set_higher_amount(&calories_buffer, &mut highest_calories_amount);
            calories_buffer = 0;
            continue;
        }

        let converted_amount: usize = amount.parse::<usize>().expect("Unable to parse calories amount from file input");
        calories_buffer += converted_amount;
    }

    highest_calories_amount
}

fn determine_highest_amounts_of_carried_calories<const TOP_ELFS_BOUNDARY: usize>(calories_table: &Vec<String>) -> usize {
    
    let mut calories_buffer: usize = 0;
    let mut highest_calories_amounts: Vec<usize> = create_initialized_vector_with_fixed_capacity(TOP_ELFS_BOUNDARY);

    for row in calories_table {

        if !is_list_seperator(row) {
            let converted_amount: usize = row.parse::<usize>().expect("Unable to parse calories amount from file input");
            calories_buffer += converted_amount;
            continue;
        }
        
        check_and_align_higher_amounts(&calories_buffer, &mut highest_calories_amounts);
        calories_buffer = 0;
    }

    check_and_align_higher_amounts(&calories_buffer, &mut highest_calories_amounts);

    highest_calories_amounts.iter().sum()
}

fn main() {
    const PUZZLE_INPUT: &str = "day_01/data/puzzle_input.txt";
    const TOP_ELFS_BOUNDARY: usize = 3;
    let calories_table: Vec<String> = read_in_puzzle_input(PUZZLE_INPUT);

    let carried_amount: usize = determine_highest_amounts_of_carried_calories::<TOP_ELFS_BOUNDARY>(&calories_table);

    println!("{}", carried_amount);
}

#[cfg(test)]
mod tests {
    use crate::{read_in_puzzle_input, determine_highest_single_carried_calories_amount, determine_highest_amounts_of_carried_calories};

    #[test]
    fn should_determine_elf_with_highest_calories() {

        // Assert
        const PUZZLE_INPUT: &str = "day_01/data/test_01.txt";
        let calories_table: Vec<String> = read_in_puzzle_input(PUZZLE_INPUT);

        // Arrange
        let highest_single_carried_amount: usize = determine_highest_single_carried_calories_amount(&calories_table);

        // Assert
        assert_eq!(24000, highest_single_carried_amount);
    }

    #[test]
    fn should_determine_top_three_elfs_with_highest_calories() {

        // Assert
        const PUZZLE_INPUT: &str = "day_01/data/test_01.txt";
        const TOP_ELFS_BOUNDARY: usize = 3;
        let calories_table: Vec<String> = read_in_puzzle_input(PUZZLE_INPUT);

        // Arrange
        let amount_calories: usize = determine_highest_amounts_of_carried_calories::<TOP_ELFS_BOUNDARY>(&calories_table);

        // Assert
        assert_eq!(45000, amount_calories);
    }
}

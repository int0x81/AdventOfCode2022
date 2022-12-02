use utils::{file_io::FileReader, collections::create_initialized_vector_with_fixed_capacity};

fn is_elf_seperator(input_string: &str) -> bool {
    input_string.is_empty()
}

#[allow(dead_code)] // only used in the first part of the challenge
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

fn calculate_sum_of_top_carried_calories(top_carried_calories: &[usize]) -> usize {
    top_carried_calories.iter().sum()
}

#[allow(dead_code)] // only used in the first part of the challenge
fn determine_single_top_carried_calories(file_reader: FileReader) -> usize {
    
    let mut calories_buffer: usize = 0;
    let mut highest_calories_amount: usize = 0;

    for line in file_reader {

        if is_elf_seperator(&line) {
            check_and_set_higher_amount(&calories_buffer, &mut highest_calories_amount);
            calories_buffer = 0;
            continue;
        }

        let converted_amount: usize = line.parse::<usize>().expect("Unable to parse calories amount from file input");
        calories_buffer += converted_amount;
    }

    highest_calories_amount
}

fn determine_top_carried_calories<const TOP_CARRIES_BOUNDARY: usize>(file_reader: FileReader) -> usize {
    
    let mut calories_buffer: usize = 0;
    let mut top_carried_calories: Vec<usize> = create_initialized_vector_with_fixed_capacity(TOP_CARRIES_BOUNDARY);

    for line in file_reader {

        if !is_elf_seperator(&line) {
            let converted_amount: usize = line.parse::<usize>().expect("Unable to parse calories amount from file input");
            calories_buffer += converted_amount;
            continue;
        }
        
        check_and_align_higher_amounts(&calories_buffer, &mut top_carried_calories);
        calories_buffer = 0;
    }

    check_and_align_higher_amounts(&calories_buffer, &mut top_carried_calories);

    calculate_sum_of_top_carried_calories(&top_carried_calories)
}

fn main() {
    const TOP_CARRIES_BOUNDARY: usize = 3;
    let file_reader: FileReader = FileReader::new("day_01/data/puzzle_input.txt");
    let top_carried_calories: usize = determine_top_carried_calories::<TOP_CARRIES_BOUNDARY>(file_reader);
    println!("{}", top_carried_calories);
}

#[cfg(test)]
mod tests {
    use utils::file_io::FileReader;
    use crate::{determine_top_carried_calories, determine_single_top_carried_calories};

    #[test]
    fn should_determine_elf_with_highest_calories() {

        // Arrange
        let file_reader: FileReader = FileReader::new("day_01/data/test.txt");

        // Act
        let highest_single_carried_amount: usize = determine_single_top_carried_calories(file_reader);

        // Assert
        assert_eq!(24000, highest_single_carried_amount);
    }

    #[test]
    fn should_determine_top_three_elfs_with_highest_calories() {

        // Arrange
        const TOP_ELFS_BOUNDARY: usize = 3;
        let file_reader: FileReader = FileReader::new("day_01/data/test.txt");

        // Act
        let amount_calories: usize = determine_top_carried_calories::<TOP_ELFS_BOUNDARY>(file_reader);

        // Assert
        assert_eq!(45000, amount_calories);
    }
}

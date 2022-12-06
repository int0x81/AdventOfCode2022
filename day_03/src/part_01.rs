mod shared;

use shared::get_priority;
use utils::file_io::FileReader;

fn determine_overlap(inventory_list: &str) -> char {
    
    if inventory_list.len() % 2 != 0 {
        panic!("Invalid inventory list")
    }

    let (first_compartment, second_compartment) = inventory_list.split_at(inventory_list.len() / 2);

    for char in first_compartment.chars() {
        if second_compartment.contains(char) {
            return char;
        }
    }

    panic!("No overlap between compartments");
}

fn compute_priority_sum(file_reader: FileReader) -> usize {
    let mut sum: usize = 0;

    for line in file_reader {
        let overlap = determine_overlap(&line);
        sum += get_priority(&overlap);
    }

    sum
}

fn main() {
    let file_reader: FileReader = FileReader::new("day_03/data/puzzle_input.txt");
    let priority_sum = compute_priority_sum(file_reader);
    println!("{}", priority_sum);
}

#[cfg(test)]
mod tests {
    use utils::file_io::FileReader;

    use crate::{determine_overlap, compute_priority_sum};

    #[test]
    fn should_determine_overlap() {

        // Arrange
        let test_string: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";

        // Act
        let overlap: char = determine_overlap(test_string);

        // Assert
        assert_eq!('p', overlap);
    }

    #[test]
    fn should_compute_priority_sum() {

        // Arrange
        let file_reader: FileReader = FileReader::new("day_03/data/test.txt");

        // Act
        let priority_sum: usize = compute_priority_sum(file_reader);

        // Assert
        assert_eq!(157, priority_sum);
    }
}

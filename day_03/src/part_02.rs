mod shared;

use shared::get_priority;
use utils::{file_io::FileReader, collections::initialize_string_array};

fn is_overlapping_char<const GROUP_SIZE: usize>(found_character: &char, badges: &[String; GROUP_SIZE], depth: usize) -> bool {

    if depth == GROUP_SIZE {
        return true;
    }

    let badge = &badges[depth];

    for character in badge.chars() {
        if character == *found_character {
            return is_overlapping_char(found_character, badges, depth + 1)
        }
    }

    return false;
}

fn find_shared_badge<const GROUP_SIZE: usize>(badges: &[String; GROUP_SIZE]) -> char {

    let first_badge = &badges[0];

    for character in first_badge.chars() {
        if is_overlapping_char(&character, badges, 1) {
            return character;
        }
    }

    panic!("Unable to find overlapping character in badge collection")
}

fn compute_priority_sum<const GROUP_SIZE: usize>(file_reader: FileReader) -> usize {
    let mut sum: usize = 0;
    let mut index: usize = 0;
    let mut groups: [String; GROUP_SIZE] = initialize_string_array::<GROUP_SIZE>();

    for line in file_reader.into_iter() {
        groups[index] = line;

        index += 1;

        if index == GROUP_SIZE {
            let overlap = find_shared_badge::<GROUP_SIZE>(&groups);
            sum += get_priority(&overlap);
            index = 0;
        }
    }

    sum
}

fn main() {
    let file_reader: FileReader = FileReader::new("day_03/data/puzzle_input.txt");
    const GROUP_SIZE: usize = 3;
    let priority_sum = compute_priority_sum::<GROUP_SIZE>(file_reader);
    println!("{}", priority_sum);
}

#[cfg(test)]
mod tests {
    use utils::file_io::FileReader;

    use crate::{compute_priority_sum, find_shared_badge};

    #[test]
    fn should_find_shared_badge_01() {

        // Arrange
        const GROUP_SIZE: usize = 3; 
        let badges: [String; 3] = [
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string()
        ];

        // Act
        let overlap: char = find_shared_badge::<GROUP_SIZE>(&badges);

        // Assert
        assert_eq!('r', overlap);
    }

    #[test]
    fn should_find_shared_badge_02() {

        // Arrange
        const GROUP_SIZE: usize = 3; 
        let badges: [String; 3] = [
            "TTLChzhDnjQLTDhTQJrzSbbJHsGrGrGFGb".to_string(),
            "BfvvpflfWVlVsFFvJHcFJFrJrt".to_string(),
            "ZwMBwwZPWMMpffflqlZMRnRNQLCNhPhDDNssnRQD".to_string()
        ];

        // Act
        let overlap: char = find_shared_badge::<GROUP_SIZE>(&badges);

        // Assert
        assert_eq!('s', overlap);
    }

    #[test]
    fn should_compute_priority_sum() {

        // Arrange
        const GROUP_SIZE: usize = 3; 
        let file_reader: FileReader = FileReader::new("day_03/data/test.txt");

        // Act
        let priority_sum: usize = compute_priority_sum::<GROUP_SIZE>(file_reader);

        // Assert
        assert_eq!(70, priority_sum);
    }
}
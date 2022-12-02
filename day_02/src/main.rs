fn determine_total_score(file_reader: FileReader) -> usize {
    todo!()
}

fn main() {
    let file_reader: FileReader = FileReader::new("day_02/data/puzzle_input.txt");
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_determine_correct_total_score() {

        // Arrange
        let file_reader: FileReader = FileReader::new("day_02/data/puzzle_input.txt");

        // Act
        let total_score = determine_total_score(file_reader);

        // Assert
        assert_eq!(15, total_score);
    }
}
use std::{fs::File, io::{BufReader, BufRead}, path::{PathBuf, Path}};

pub struct FileReader {
    buffer_reader: BufReader<File>,
    index: usize
}

impl FileReader {
    fn get_file_path(file: &str) -> String {
        let current_dir: PathBuf = std::env::current_dir().expect("Unable to get current directory");
        let full_path: PathBuf = current_dir.join(file);
        let full_path: &Path = full_path.as_path();
        full_path.to_str().expect("Unable to convert file path to string").to_string()
    }
    
    pub fn new(file_name: &str) -> FileReader {
        let path = FileReader::get_file_path(file_name);
        let file: File = File::open(path).expect("Unable to open file");
        FileReader {
            buffer_reader: BufReader::new(file),
            index: 0
        }
    }
}

impl Iterator for FileReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line_buffer = String::new();
        let read_result = self.buffer_reader.read_line(&mut line_buffer);

        match read_result {
            Ok(code) => {
                if code == 0 {
                    None
                } else {
                    self.index += 1;
                    line_buffer.pop(); //remove new line delimiter
                    Some(line_buffer)
                }
            }
            Err(_) => None
        } 
    }
}
use std::{ io::{ Lines, BufReader, BufRead, Write }, fs::File };

pub static TEST_FILE: &str = "test-input.txt";
pub static FILE: &str = "input.txt";

pub fn get_lines(path: &str) -> Lines<BufReader<File>> {
    let input_file = File::open(path).unwrap();
    BufReader::new(input_file).lines()
}

pub fn create_lines(text: &str, path: &str) -> Lines<BufReader<File>> {
    write_text(text, path);
    get_lines(path)
}

pub fn write_text(text: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(text.as_bytes()).unwrap();
}
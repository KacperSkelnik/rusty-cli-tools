use thiserror::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Error, Debug)]
enum UserInputError {
    #[error("Too many arguments!")]
    TooManyArguments,

    #[error("Too few arguments!")]
    TooFewArguments
}

fn read_file(path: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn process_file(reader: BufReader<File>, find_element: &str) -> std::io::Result<()> {
    for line in reader.lines() {
        line.map(|line_string| if line_string.contains(find_element) { println!("{}", line_string) } )?
    }

    Ok(())
}

fn main() -> Result<(), UserInputError> {
    let input = std::env::args().skip(1).collect::<Vec<_>>();
    if input.len() > 2 { return Err(UserInputError::TooManyArguments) };
    if input.len() < 2 { return Err(UserInputError::TooFewArguments) };

    let find_element = input.get(0).map(|i| i.as_str()).unwrap();
    let path = input.get(1).map(|i| i.as_str()).unwrap();

    read_file(path).and_then(|reader| process_file(reader, find_element)).expect("Unexpected error!");

    Ok(())
}

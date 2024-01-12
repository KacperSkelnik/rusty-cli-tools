use thiserror::Error;

#[derive(Error, Debug)]
enum UserInputError {
    #[error("Too many arguments!")]
    TooManyArguments,

    #[error("Provided directory does not exists!")]
    DirectoryDoesNotExists(#[source] std::io::Error)
}

fn main() -> Result<(), UserInputError> {
    let input = std::env::args().skip(1).collect::<Vec<_>>();
    if input.len() > 1 { return Err(UserInputError::TooManyArguments) };

    let path = input.get(0).map(|i| i.as_str()).unwrap_or(&"./");
    let dir = std::fs::read_dir(path).map_err(UserInputError::DirectoryDoesNotExists)?;

    dir.for_each( |d| {
        d.map(|x| x.file_name().to_ascii_lowercase())
            .map(|x| x.to_str().map(|result| println!("{}", result)))
            .expect("Unexpected error!");
    });

    Ok(())
}

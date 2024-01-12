struct Input {
    file_name: String,
}

fn main() {
    let file_name: String = std::env::args().nth(1).expect("Argument must be provided!");
    let input = Input { file_name };

    for line in std::fs::read_to_string(input.file_name)
        .expect("Specified file does not exists!")
        .lines()
    {
        println!("{}", line)
    }
}

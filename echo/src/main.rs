fn main() {
    let input: String = std::env::args()
        .skip(1)
        .map(|i| i + " ")
        .collect::<String>();
    print!("{}", input)
}

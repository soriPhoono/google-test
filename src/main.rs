use std::fs::read_to_string;

fn main() {
    let file_input = read_to_string("./resources/e.txt")
        .expect("Failed to read input file")
        .split("\n")
        .map(|element| String::from(element))
        .reduce(|acumulator, element| acumulator + element.trim())
        .expect("Failed to reduce the input number to a single line");

    let chars = file_input.chars().collect::<Vec<_>>();
    let possible_primes = chars.chunks(10).filter_map(|input| {
        let mut input_line = String::new();
        for character in input {
            input_line.push(*character);
        }
        input_line.parse::<u64>().ok()
    });

    for possible_prime in possible_primes {
        if primes::is_prime(possible_prime) {
            println!("Found prime: {}", possible_prime);
            break;
        }
    }
}

use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let input: Vec<char> = input.chars().collect();

    let first_with_differing_chars = input
        .windows(4)
        .enumerate()
        .filter(|(_, view)| HashSet::<char>::from_iter(view.iter().cloned()).len() >= 4)
        .next()
        .unwrap()
        .0
        + 4;

    println!(
        "characters processed before first start-of-packet: {}",
        first_with_differing_chars
    );

    let first_with_14_differing_chars = input
        .windows(14)
        .enumerate()
        .filter(|(_, view)| HashSet::<char>::from_iter(view.iter().cloned()).len() >= 14)
        .next()
        .unwrap()
        .0
        + 14;

    println!(
        "characters processed before first start-of-message: {}",
        first_with_14_differing_chars
    );
}

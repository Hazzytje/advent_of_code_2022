use std::collections::HashSet;
use std::fs;

fn char_value(in_char: char) -> u32 {
    match in_char {
        'a'..='z' => in_char as u32 - 'a' as u32 + 1,
        'A'..='Z' => in_char as u32 - 'A' as u32 + 27,
        _ => panic!("unknown char"),
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let input = input.lines();

    let sum_of_priorities: u32 = input
        .clone()
        .map(|line| {
            let first_half: HashSet<char> = HashSet::from_iter(line[0..line.len() / 2].chars());
            let second_half = HashSet::from_iter(line[line.len() / 2..line.len()].chars());
            first_half
                .intersection(&second_half)
                .into_iter()
                .next()
                .unwrap()
                .clone()
        })
        .map(char_value)
        .sum();

    println!("sum of priorities is {}", sum_of_priorities);

    let vec_input: Vec<&str> = input.collect();
    let sum_of_badges: u32 = vec_input
        .chunks(3)
        .map(|arr| {
            let first_part: HashSet<char> = HashSet::from_iter(arr[0].chars());
            let second_part: HashSet<char> = HashSet::from_iter(arr[1].chars());
            let third_part: HashSet<char> = HashSet::from_iter(arr[2].chars());

            let first_combination: HashSet<char> =
                HashSet::from_iter(first_part.intersection(&second_part).map(|c| *c));
            first_combination
                .intersection(&third_part)
                .into_iter()
                .next()
                .unwrap()
                .clone()
        })
        .map(char_value)
        .sum();

    println!("sum of badges is {}", sum_of_badges);
}

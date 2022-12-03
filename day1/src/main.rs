use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let input = input.split("\n\n");

    let mut calories_per_elf : Vec<i32> = input.map(|group|
        group.lines().map(|n| n.parse::<i32>().unwrap()).sum()
    ).collect();

    println!("Max is {}", calories_per_elf.iter().max().unwrap());

    calories_per_elf.sort();
    let max3 : i32 = calories_per_elf.iter().rev().take(3).sum();

    println!("Max3 is {}", max3);
}

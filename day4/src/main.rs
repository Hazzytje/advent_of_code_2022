use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let input = input.lines();

    let fully_contained_count = input
        .clone()
        .map(|line| {
            let mut parts = line.split(&['-', ',']).map(|n| n.parse::<i32>().unwrap());
            let elf1_start = parts.next().unwrap();
            let elf1_end = parts.next().unwrap();
            let elf2_start = parts.next().unwrap();
            let elf2_end = parts.next().unwrap();

            return (elf1_start >= elf2_start && elf1_end <= elf2_end)
                || (elf2_start >= elf1_start && elf2_end <= elf1_end);
        })
        .filter(|b| *b)
        .count();

    println!("number of fully contained: {}", fully_contained_count);

    let partially_contained_count = input
        .map(|line| {
            let mut parts = line.split(&['-', ',']).map(|n| n.parse::<i32>().unwrap());
            let elf1_start = parts.next().unwrap();
            let elf1_end = parts.next().unwrap();
            let elf2_start = parts.next().unwrap();
            let elf2_end = parts.next().unwrap();

            return (elf1_start <= elf2_end && elf1_end >= elf2_start)
                || (elf2_start >= elf1_end && elf2_end <= elf1_start);
        })
        .filter(|b| *b)
        .count();

    println!(
        "number of partially contained: {}",
        partially_contained_count
    );
}

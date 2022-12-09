use std::fs;
use regex::Regex;

fn print_stacks(stacks : &Vec<Vec<char>>) {
    let highest_stack : usize = stacks.iter().map(|s| s.len()).max().unwrap();

    for height in (0..highest_stack).rev() {
        println!("{}", stacks.iter().map(|stack| if stack.len() > height { format!("[{}]", stack[height]) } else { String::from("   ") }).fold(String::new(), |a, b| a + &b + " "));
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let mut inputs = input.split("\n\n");

    let setup = inputs.next().unwrap();
    let moves = inputs.next().unwrap().lines();

    let mut stacks : Vec<Vec<char>> = Vec::new();

    let num_stacks = setup.clone().lines().last().unwrap().split_whitespace().last().unwrap().parse::<i32>().unwrap();

    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    setup.lines().rev().skip(1).for_each(|line| {
        for i in 0..num_stacks {
            let c = line.chars().nth((i * 4 + 1) as usize).unwrap();
            if c != ' ' {
                stacks[i as usize].push(c);
            }
        }
    });

    let initial_stacks = stacks.clone();

    let move_regex = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").expect("Regex failed to compile");
    moves.clone().for_each(|line| {
        move_regex.captures(line).map(|move_parts| {
            let num = move_parts[1].parse::<u32>().unwrap();
            let from = move_parts[2].parse::<usize>().unwrap();
            let to = move_parts[3].parse::<usize>().unwrap();

            for _ in 0..num {
                let c = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(c);
                // stacks[to].push(stacks[from].pop().unwrap());
            }
        });
    });

    print_stacks(&stacks);

    println!("tops of stacks: {}", stacks.iter().map(|s| s.last().unwrap()).fold(String::new(), |a, b| a + &b.to_string()));

    let mut stacks = initial_stacks;

    let move_regex = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").expect("Regex failed to compile");
    moves.for_each(|line| {
        move_regex.captures(line).map(|move_parts| {
            let num = move_parts[1].parse::<u32>().unwrap();
            let from = move_parts[2].parse::<usize>().unwrap();
            let to = move_parts[3].parse::<usize>().unwrap();

            let mut tmp : Vec<char> = Vec::new();
            for _ in 0..num {
                let c = stacks[from - 1].pop().unwrap();
                tmp.push(c);
            }
            for _ in 0..num {
                stacks[to - 1].push(tmp.pop().unwrap());
            }
        });
    });

    print_stacks(&stacks);

    println!("tops of stacks 9001: {}", stacks.iter().map(|s| s.last().unwrap()).fold(String::new(), |a, b| a + &b.to_string()));
}

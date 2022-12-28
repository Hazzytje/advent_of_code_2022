use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn find_dirs(files: &HashMap<String, i32>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for (file_name, _) in files {
        let mut cur_file = file_name.clone();
        while cur_file.contains("/") {
            let last_directory_separator = cur_file.rfind('/').unwrap();
            cur_file.truncate(last_directory_separator);
            if cur_file.len() > 0 {
                res.push(cur_file.clone());
            }
        }
    }
    res
}

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let input = input.lines();

    let regex = Regex::new(r"^(?:\$ )?(\w+) ?([\w.]+)?$").expect("Regex failed to compile");
    let mut files: HashMap<String, i32> = HashMap::new();
    let mut current_dir = String::new();

    for line in input {
        regex.captures(line).map(|captures| {
            if &captures[1] == "ls" {
                // do nothing! Announces that a directory listing follows, but we just assume one
                // always does
            } else if &captures[1] == "cd" {
                if &captures[2] == ".." {
                    let last_directory_separator = current_dir.rfind('/').unwrap();
                    current_dir.truncate(last_directory_separator);
                } else {
                    current_dir += "/";
                    current_dir += &captures[2];
                }
            } else if &captures[1] == "dir" {
                // do nothing! ls listed a directory, but we'll find its name when we cd into it
            } else {
                // if none of the others match, it's the listing of a file
                let file_size = &captures[1].parse::<i32>().unwrap();
                let file_name = current_dir.clone() + "/" + &captures[2];

                files.insert(file_name, *file_size);
            }
        });
    }

    let mut all_dirs = find_dirs(&files);
    all_dirs.sort();
    all_dirs.dedup();

    let directory_sizes: HashMap<String, i32> = all_dirs
        .clone()
        .into_iter()
        .map(|dir| {
            let dir_size: i32 = files
                .iter()
                .filter(|(file, _)| file.starts_with(&(dir.to_string() + "/")))
                .map(|(_, size)| size)
                .sum();
            (dir, dir_size)
        })
        .collect();

    let sum_of_directories_smaller_than_100k: i32 = directory_sizes
        .clone()
        .into_iter()
        .filter(|(_, size)| size <= &100_000)
        .map(|(_, size)| size)
        .sum();

    println!(
        "sum of directories smaller than 100k: {}",
        sum_of_directories_smaller_than_100k
    );

    const MAX_FS_SIZE: i32 = 70_000_000;
    const NEED_FS_SIZE: i32 = 30_000_000;
    let free_space: i32 = MAX_FS_SIZE - files.iter().map(|(_, size)| size).sum::<i32>();
    let needed_space = NEED_FS_SIZE - free_space;

    let smallest_directory_to_delete: i32 = directory_sizes
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| size >= &needed_space)
        .min()
        .unwrap();

    println!(
        "smallest directory to delete to make enough space: {}",
        smallest_directory_to_delete
    );
}

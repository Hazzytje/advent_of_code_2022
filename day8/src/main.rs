use std::fs;

fn visible(forest: &Vec<Vec<i8>>, x: usize, y: usize) -> bool {
    let forest_height = forest.len();
    let forest_width = forest[0].len();
    let tree = forest[y][x];

    !((0..x).any(|i| forest[y][i] >= tree)
        && ((x + 1)..forest_width).any(|i| forest[y][i] >= tree)
        && (0..y).any(|i| forest[i][x] >= tree)
        && ((y + 1)..forest_height).any(|i| forest[i][x] >= tree))
}

fn scenic_score(forest: &Vec<Vec<i8>>, x: usize, y: usize) -> i32 {
    let forest_height = forest.len();
    let forest_width = forest[0].len();
    let tree = forest[y][x];

    let mut visibility_above = 0;
    for i in (0..x).rev() {
        visibility_above += 1;
        if forest[y][i] >= tree {
            break;
        }
    }
    let mut visibility_below = 0;
    for i in (x + 1)..forest_width {
        visibility_below += 1;
        if forest[y][i] >= tree {
            break;
        }
    }
    let mut visibility_left = 0;
    for i in (0..y).rev() {
        visibility_left += 1;
        if forest[i][x] >= tree {
            break;
        }
    }
    let mut visibility_right = 0;
    for i in (y + 1)..forest_height {
        visibility_right += 1;
        if forest[i][x] >= tree {
            break;
        }
    }

    visibility_above * visibility_below * visibility_left * visibility_right
}

fn main() {
    let input = fs::read_to_string("input").expect("Reading input file error");
    let input = input.lines();
    let input: Vec<Vec<i8>> = input
        .map(|line| {
            line.chars()
                .map(|c| (c.to_string()).parse::<i8>().unwrap())
                .collect()
        })
        .collect();

    let visible_trees = input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| visible(&input, x, y))
                .filter(|b| *b)
                .count()
        })
        .map(|c| c as i32)
        .sum::<i32>();

    println!("visible trees: {}", visible_trees);

    let highest_scenic_score = input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| scenic_score(&input, x, y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("highest scenic score: {}", highest_scenic_score);
}

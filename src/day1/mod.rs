use std::collections::HashMap;

pub fn task_1() {
    // read input
    let (mut hist1, mut hist2) = parse_input(include_str!("./input.txt"));

    hist1.sort();
    hist2.sort();

    // calculate result
    let mut result = 0;
    for (x, y) in hist1.into_iter().zip(hist2.into_iter()) {
        result += x.abs_diff(y);
    }

    println!("result {}", result)
}

pub fn task_2() {
    let (mut hist1, mut hist2) = parse_input(include_str!("./input.txt"));

    // count occurrences (keep it simple)
    let mut occ: HashMap<u64, u64> = HashMap::new();
    for val in hist2 {
        *occ.entry(val).or_default() += 1;
    }

    // calculate result
    let mut result = 0;
    for x in hist1 {
        result += x * *occ.entry(x).or_default();
    }

    println!("result {}", result)
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    input.lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .unzip()
}

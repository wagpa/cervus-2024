use regex::Regex;

pub fn task_1() {
    let input = include_str!("./input.txt");

    let regex = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    let captures = regex.captures_iter(input);

    let mut result = 0;
    for (_, [a, b]) in captures.map(|capture| capture.extract()) {
        result += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
    }

    println!("result {}", result)
}

pub fn task_2() {
    let input = include_str!("input.txt");

    let regex = Regex::new(r"(:?(do|don't)\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();
    let captures = regex.captures_iter(input);

    let mut result = 0;
    let mut skip = false;
    for capture in captures {
        if let Some(don_t) = capture.get(2) {
            skip = don_t.as_str() != "do";
        }

        if skip {
            continue
        }

        let Some(a) = capture.get(3) else { continue };
        let Some(b) = capture.get(4) else { continue };
        result += a.as_str().parse::<i64>().unwrap() * b.as_str().parse::<i64>().unwrap();
    }

    println!("result {}", result)
}
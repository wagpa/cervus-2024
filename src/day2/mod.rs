
pub fn task_1() {
    let reports = parse_input(include_str!("./input.txt"));

    let mut result = 0;
    'reports: for report in reports.iter() {
        let mut dir = 0;
        for window in report.windows(2) {
            let [x, y] = window else { panic!("invalid window size") };
            let delta = x - y;
            if delta * dir < 0 || delta == 0 || delta.abs() > 3  {
                // invalid
                continue 'reports
            }
            dir = delta;
        }
        // all valid
        result += 1;
    }

    println!("result {}", result)
}

pub fn task_2() {
    let reports = parse_input(include_str!("./input.txt"));

    fn check(report: &[i64]) -> bool {
        let mut dir = 0;
        for window in report.windows(2) {
            let [mut x, y] = window else { panic!("invalid window size") };
            let delta = x - y;
            if delta * dir < 0 || delta == 0 || delta.abs() > 3  {
                // invalid
                return false
            }
            dir = delta;
        }
        // all valid
        true
    }

    let mut result = 0;
    'reports: for report in reports.iter() {
        // check full
        if check(report) {
            result += 1;
            continue 'reports
        }

        // check subsets (inefficient but works)
        // iteratively checks report with one value removed
        for i in 0..report.len() {
            let sub: Vec<i64> = report.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| *v).collect();
            if check(&sub) {
                result += 1;
                continue 'reports
            }
        }
    }

    println!("result {}", result)
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line|
            line.split(" ")
                .map(|val| val.parse::<i64>().unwrap())
                .collect()
        )
        .collect()
}
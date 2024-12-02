fn is_safe_report(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn can_be_safe_with_dampener(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe_report(&modified_report) {
            return true;
        }
    }

    false
}

fn part01(reports: &[Vec<i32>]) -> i32 {
    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count() as i32
}

fn part02(reports: &[Vec<i32>]) -> i32 {
    reports
        .iter()
        .filter(|report| is_safe_report(report) || can_be_safe_with_dampener(report))
        .count() as i32
}

fn parse_input(input: &str) -> Result<Vec<Vec<i32>>, String> {
    let mut reports = Vec::new();
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if levels.is_empty() {
            return Err(format!("Invalid line in input: {}", line));
        }

        reports.push(levels);
    }

    Ok(reports)
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day02.txt").expect("Failed to read input");

    let reports = match parse_input(&input) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Input parsing failed: {}", error);
            return;
        }
    };

    let part01_result = part01(&reports);
    println!("Part 01 Solution: {}", part01_result);

    let part02_result = part02(&reports);
    println!("Part 02 Solution: {}", part02_result);
}

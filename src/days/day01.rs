fn part01(left_list: &[i32], right_list: &[i32]) -> i32 {
    let mut left_sorted = left_list.to_vec();
    let mut right_sorted = right_list.to_vec();

    left_sorted.sort();
    right_sorted.sort();

    left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part02(left_list: &[i32], right_list: &[i32]) -> i32 {
    left_list
        .iter()
        .map(|&num| {
            let count = right_list.iter().filter(|&&x| x == num).count();
            num * count as i32
        })
        .sum()
}

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), String> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for (line_number, line) in input.lines().enumerate() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        } else {
            return Err(format!(
                "Error on line {}: Each line must contain exactly two numbers.",
                line_number + 1
            ));
        }
    }

    Ok((left_list, right_list))
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day01.txt").expect("Failed to read input");

    let (left_list, right_list) = match parse_input(&input) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Input parsing failed: {}", error);
            return;
        }
    };

    let part01_result = part01(&left_list, &right_list);
    println!("Part 01 Solution: {}", part01_result);

    let part02_result = part02(&left_list, &right_list);
    println!("Part 02 Solution: {}", part02_result);
}

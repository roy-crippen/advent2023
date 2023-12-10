fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    println!("day01-a = {}", solve(&lines)); // 54632

    let converted_lines: Vec<String> = lines.iter().map(|x| convert(x)).collect();
    println!("day01-b = {}", solve(&converted_lines)) // 54019
}

/// Converts a written number `one` to `nine` with numbers `1` to `9`
fn convert(in_str: &str) -> String {
    in_str
        .to_string()
        .replace("twone", "21e")
        .replace("oneight", "18t")
        .replace("threeight", "38t")
        .replace("fiveight", "58t")
        .replace("nineight", "98t")
        .replace("eighthree", "83e")
        .replace("eightwo", "82o")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

/// Sum of 2 digit numbers created from first and last digit found in each line of `lines`
fn solve(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|s| s.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<char>>())
        .filter(|cs| !cs.is_empty())
        .map(|cs| cs[0].to_digit(10).unwrap() * 10 + cs.last().unwrap().to_digit(10).unwrap())
        .sum()
}

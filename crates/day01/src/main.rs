fn main() {
    let xs: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|l| l.to_owned())
        .collect();
    println!("day01-a = {}", parse(&xs)); // 54632
    let ys: Vec<String> = xs.iter().map(|x| convert(x)).collect();
    println!("day01-b = {}", parse(&ys)) // 54019
}

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

fn parse(ss: &[String]) -> u32 {
    ss.iter()
        .map(|s| {
            s.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>()
        })
        .filter(|cs| !cs.is_empty())
        .map(|cs| {
            cs.first().unwrap().to_digit(10).unwrap() * 10
                + cs.last().unwrap().to_digit(10).unwrap()
        })
        .sum()
}

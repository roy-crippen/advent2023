fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let (value_a, value_b) = solve(&lines, 12, 13, 14);

    println!("day02-a = {value_a}"); // 2679
    println!("day02-b = {value_b}") // 77607
}

/// Solves and return u32 values for both part a and b by parsing `lines`
fn solve(lines: &[String], nr: u32, ng: u32, nb: u32) -> (u32, u32) {
    let mut value_a: u32 = 0;
    let mut value_b: u32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut a_fail = false;
        let mut max_count_rgb = (0, 0, 0);
        for s1 in line.split_once(':').unwrap().1.split(';') {
            for s2 in s1.split(',') {
                let vs: Vec<&str> = s2.trim().split(' ').collect();
                let count = vs[0].parse::<u32>().unwrap();
                let color = vs[1];
                match color {
                    "red" => {
                        max_count_rgb.0 = max_count_rgb.0.max(count);
                        a_fail = a_fail || count > nr
                    }
                    "green" => {
                        max_count_rgb.1 = max_count_rgb.1.max(count);
                        a_fail = a_fail || count > ng
                    }
                    _ => {
                        max_count_rgb.2 = max_count_rgb.2.max(count);
                        a_fail = a_fail || count > nb
                    }
                }
            }
        }
        if !a_fail {
            value_a += i as u32 + 1;
        }
        value_b += max_count_rgb.0 * max_count_rgb.1 * max_count_rgb.2;
    }
    (value_a, value_b)
}

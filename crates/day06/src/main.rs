fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();

    let xs = parse(&lines);
    println!("day06-a = {}", solve_a(&xs)); // 1195150

    let (end_t, record) = get_race(&xs);
    println!("day06-b = {}", get_beat_record_cnt(end_t, record)); // 42550411
}

fn get_beat_record_cnt(end_t: usize, record: usize) -> usize {
    // find first winner counting up
    let first = (1..=end_t).find(|t| t * (end_t - t) > record).unwrap();

    // find the last winner counting down
    let last = (1..=end_t).rev().find(|t| t * (end_t - t) > record).unwrap();

    last - first + 1
}

fn solve_a(xs: &Vec<(usize, usize)>) -> usize {
    xs.iter().map(|(end_t, record)| get_beat_record_cnt(*end_t, *record)).product()
}

fn parse(lines: &[String]) -> Vec<(usize, usize)> {
    let xs: Vec<Vec<usize>> = lines
        .iter()
        .map(|l| {
            l.split_once(": ")
                .unwrap()
                .1
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<usize>().unwrap_or(0))
                .collect()
        })
        .collect();
    let xs: Vec<_> = xs[0].clone().into_iter().zip(xs[1].clone()).collect();
    xs
}

fn get_race(xs: &Vec<(usize, usize)>) -> (usize, usize) {
    let (s1, s2) = xs
        .iter()
        .map(|(u1, u2)| (u1.to_string(), u2.to_string()))
        .fold(("".to_string(), "".to_string()), |acc, v| (acc.0 + &*v.0, acc.1 + &*v.1));
    (s1.trim().parse::<usize>().unwrap_or(0), s2.trim().parse::<usize>().unwrap_or(0))
}

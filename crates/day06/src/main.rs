fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let xs = parse(&lines);

    println!("{xs:?}");
    println!("day06-a = {}", solve_a(&xs)); // 1195150
}

fn get_beat_record_cnt(end_t: usize, record: usize) -> usize {
    (1..=end_t).map(|t| t * (end_t - t)).filter(|v| *v > record).count()
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

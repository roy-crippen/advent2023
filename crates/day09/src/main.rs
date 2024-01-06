fn main() {
    let xss: Vec<Vec<isize>> = include_str!("input.txt")
        .lines()
        .map(|l| l.split(' ').map(|s| s.trim().parse::<isize>().unwrap_or(0isize)).collect())
        .collect();

    println!("day09-a = {}", solve(&xss, 'a')); // 1939607039
    println!("day09-b = {}", solve(&xss, 'b')); // 1041
}

fn solve(xss: &[Vec<isize>], part: char) -> isize {
    if part.eq(&'a') {
        xss.iter().map(|xs| diff(xs.clone(), part)).sum()
    } else {
        -xss.iter().map(|xs| diff(xs.clone(), part)).sum::<isize>()
    }
}

fn diff(xs: Vec<isize>, part: char) -> isize {
    if xs.iter().all(|x| *x == 0) {
        return 0;
    }
    let mut ys: Vec<isize> = Vec::new();
    for i in 1..xs.len() {
        ys.push(xs[i] - xs[i - 1]);
    }
    if part.eq(&'a') {
        *xs.last().unwrap() + crate::diff(ys, part)
    } else {
        -xs[0] - diff(ys, part)
    }
}

use std::collections::HashSet;

fn main() {
    let mut css: Vec<Vec<char>> = include_str!("input.txt").lines().map(|l| l.chars().collect()).collect();

    // ring the entire 2d-array with '.' for easier indexing when doing neighbor discovery
    for cs in css.iter_mut() {
        cs.insert(0, '.');
        cs.push('.');
    }
    let dots: Vec<char> = ".".repeat(css[0].len()).chars().collect();
    css.insert(0, dots.clone());
    css.push(dots);

    let (value_a, value_b) = solve(&css);
    println!("day03-a = {value_a}"); // 507214
    println!("day03-b = {value_b}"); // 72553319
}

#[derive(Debug)]
struct Part {
    pub value: u32,
    pub symbol: char,
    pub row: usize,
    pub col: usize,
}

/// Solves and return u32 values for both part a and b by parsing `css`
fn solve(css: &Vec<Vec<char>>) -> (u32, u32) {
    let mut value_a = 0;
    let mut parts: Vec<Part> = Vec::new();
    for i in 1..css.len() {
        let mut j_start = usize::MAX;
        let mut j_end = usize::MAX;
        for j in 1..css[0].len() {
            if css[i][j].is_ascii_digit() {
                if j_start == usize::MAX {
                    j_start = j;
                }
                if j_start != usize::MAX {
                    j_end = j;
                }
            } else if j_start != usize::MAX {
                if let Some(part) = get_part(css, i, j_start, j_end) {
                    value_a += part.value;
                    if part.symbol == '*' {
                        parts.push(part);
                    }
                }
                j_start = usize::MAX;
                j_end = usize::MAX;
            }
        }
    }

    // now calculate part b value from the list of parts
    let mut value_b = 0;
    for (i, part) in parts.iter().enumerate() {
        let mut v = part.value;
        let mut cnt = 0;
        let js = i + 1..parts.len();
        for j in js {
            if part.col == parts[j].col && part.row == parts[j].row {
                cnt += 1;
                v *= parts[j].value
            }
        }
        if cnt > 0 {
            value_b += v;
        }
    }

    (value_a, value_b)
}

/// Create a Part if one exists on row `i` from col `j_start` to `j_end` from data in `css`
fn get_part(css: &[Vec<char>], i: usize, j_start: usize, j_end: usize) -> Option<Part> {
    let mut ns = HashSet::new();
    let rs = i - 1..=i + 1;
    for r in rs {
        for j in j_start..=j_end {
            for c in j - 1..=j + 1 {
                let ch: char = css[r][c];
                if !ch.is_ascii_digit() && ch != '.' {
                    ns.insert((ch, r, c));
                }
            }
        }
    }

    if ns.len() == 1 {
        let value = css[i][j_start..=j_end].iter().collect::<String>().parse::<u32>().unwrap();
        let (symbol, row, col) = ns.iter().collect::<Vec<_>>()[0];
        return Some(Part { value, symbol: *symbol, row: *row, col: *col });
    }
    None
}

fn main() {
    let mut css: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    for cs in css.iter_mut() {
        cs.insert(0, '.');
        cs.push('.');
    }
    let dots: Vec<char> = ".".repeat(css[0].len()).chars().collect();
    css.insert(0, dots.clone());
    css.push(dots);
    // for cs in &css {
    //     println!("{:?}", cs.iter().collect::<String>())
    // }

    println!("day03-a = {}", solve(&css)) // 507214
}

fn solve(css: &Vec<Vec<char>>) -> u32 {
    let mut v = 0;
    for i in 0..css.len() {
        let mut j_start = usize::MAX;
        let mut j_end = usize::MAX;
        for j in 0..css[0].len() {
            if css[i][j].is_ascii_digit() {
                if j_start == usize::MAX {
                    j_start = j;
                }
                if j_start != usize::MAX {
                    j_end = j;
                }
            } else if j_start != usize::MAX {
                v += get_value(css, i, j_start, j_end);
                j_start = usize::MAX;
                j_end = usize::MAX;
            }
        }
    }

    v
}

fn get_value(ss: &[Vec<char>], i: usize, j_start: usize, j_end: usize) -> u32 {
    let has_symbol = |j| {
        for r in i - 1..=i + 1 {
            for c in j - 1..=j + 1 {
                let v: char = ss[r][c];
                if !v.is_ascii_digit() && v != '.' {
                    return true;
                }
            }
        }
        false
    };

    for j in j_start..=j_end {
        if has_symbol(j) {
            return ss[i][j_start..=j_end]
                .iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
        }
    }
    0
}

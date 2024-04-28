use std::iter;

fn main() {
    let xss: Vec<Vec<char>> = include_str!("input.txt").lines().map(|l| l.chars().collect()).collect();
    println!("day11-a = {}", solve_a(&xss)); // 9556896
}

fn solve_a(_xss: &Vec<Vec<char>>) -> usize {
    let mut xss = _xss.clone();
    expand_grid(&mut xss);
    let mut zs: Vec<(usize, usize)> = Vec::new();
    for (row, xs) in xss.iter().enumerate() {
        for (col, x) in xs.iter().enumerate() {
            if x.eq(&'#') {
                zs.push((row, col))
            }
        }
    }
    let mut tot: usize = 0;
    for i in 0..zs.len() {
        let v1 = zs[i];
        for j in i + 1..zs.len() {
            let v2 = zs[j];
            let r_tot = if v2.0 > v1.0 { v2.0 - v1.0 } else { v1.0 - v2.0 };
            let c_tot = if v2.1 > v1.1 { v2.1 - v1.1 } else { v1.1 - v2.1 };
            tot += r_tot + c_tot
        }
    }
    tot
}

fn expand_grid(xss: &mut Vec<Vec<char>>) {
    add_rows(xss);
    transpose_grid(xss);
    add_rows(xss);
    transpose_grid(xss)
}

fn add_rows(xss: &mut Vec<Vec<char>>) {
    assert!(!xss.is_empty());
    assert!(!xss[0].is_empty());
    let mut idxs: Vec<usize> = Vec::new();
    for (i, row) in xss.iter().enumerate() {
        if !row.contains(&'#') {
            idxs.push(i);
        }
    }
    let dot_row: Vec<char> = iter::repeat('.').take(xss[0].len()).collect();
    for idx in idxs.iter().rev() {
        xss.insert(*idx, dot_row.clone())
    }
}

fn transpose_grid(xss: &mut Vec<Vec<char>>) {
    assert!(!xss.is_empty());
    assert!(!xss[0].is_empty());
    let rows = xss.len();
    let cols = xss[0].len();
    let yss: Vec<Vec<_>> = (0..cols).map(|col| (0..rows).map(|row| xss[row][col]).collect()).collect();
    xss.clear();
    for ys in yss {
        xss.push(ys.clone())
    }
}

#[allow(dead_code)]
fn show_grid(xss: &Vec<Vec<char>>) {
    for xs in xss {
        for x in xs {
            print!("{x}")
        }
        println!("")
    }
    println!()
}

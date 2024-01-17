use std::iter;

fn main() {
    let ls: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let css = parse(&ls);

    println!("day10-a = {}", solve_a(&css)); // 6714
    println!("day10-b = {}", solve_b(&css)); // 429
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
    Done,
}

fn solve_a(css: &Vec<Vec<char>>) -> usize {
    let vss = fill_path(css);
    let path_len: usize = vss.iter().map(|vs| vs.iter().fold(0, |acc, v| if v != &'.' { acc + 1 } else { acc })).sum();
    (path_len + 1) / 2
}

fn solve_b(_css: &Vec<Vec<char>>) -> usize {
    // https://www.reddit.com/r/adventofcode/comments/18fgddy/2023_day_10_part_2_using_a_rendering_algorithm_to/
    let css = fill_path(_css);
    // show_grid(&css);

    let mut inside_count = 0;
    for cs in css {
        let s: String = cs
            .iter()
            .collect::<String>()
            .replace('-', "")
            .replace("F7", "")
            .replace("LJ", "")
            .replace("FJ", "|")
            .replace("L7", "|");
        // dbg!(&s);
        let mut is_inside = false;
        for ch in s.chars() {
            if ch == '|' {
                is_inside = !is_inside;
                continue;
            }
            if is_inside {
                inside_count += 1;
            }
        }
    }
    inside_count
}

fn fill_path(css: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!css.is_empty());
    assert!(!css[0].is_empty());
    let mut vss: Vec<Vec<char>> = iter::repeat(vec!['.'; css[0].len()]).take(css.len()).collect();

    let (mut row, mut col, mut dir, s_char) = find_start_pos(css);
    match dir {
        Dir::N => row -= 1,
        Dir::S => row += 1,
        Dir::E => col += 1,
        _ => col -= 1,
    }

    while dir != Dir::Done {
        let ch = css[row][col];
        vss[row][col] = ch;
        (row, col, dir) = next_pos(ch, row, col, dir);
    }

    // replace 'S' with one-of('|', '-', 'L', 'J', '7', 'F')
    vss[row][col] = s_char;
    vss
}

fn next_pos(current_ch: char, r: usize, c: usize, dir: Dir) -> (usize, usize, Dir) {
    match (current_ch, dir) {
        ('|', Dir::N) => (r - 1, c, Dir::N),
        ('|', Dir::S) => (r + 1, c, Dir::S),
        ('-', Dir::E) => (r, c + 1, Dir::E),
        ('-', Dir::W) => (r, c - 1, Dir::W),
        ('L', Dir::S) => (r, c + 1, Dir::E),
        ('L', Dir::W) => (r - 1, c, Dir::N),
        ('J', Dir::S) => (r, c - 1, Dir::W),
        ('J', Dir::E) => (r - 1, c, Dir::N),
        ('7', Dir::N) => (r, c - 1, Dir::W),
        ('7', Dir::E) => (r + 1, c, Dir::S),
        ('F', Dir::N) => (r, c + 1, Dir::E),
        ('F', Dir::W) => (r + 1, c, Dir::S),
        ('S', _) => (r, c, Dir::Done),
        _ => panic!("should never be on a '.'"),
    }
}

fn find_start_pos(css: &Vec<Vec<char>>) -> (usize, usize, Dir, char) {
    let mut nsew: (Option<Dir>, Option<Dir>, Option<Dir>, Option<Dir>) = (None, None, None, None);
    for (row, cs) in css.iter().enumerate().skip(1).take(css.len() - 2) {
        for (col, c) in cs.iter().enumerate().skip(1).take(cs.len() - 2) {
            if c.eq(&'S') {
                let mut dir: Dir = Dir::N;

                // check north
                let n = css[row - 1][col];
                if n.eq(&'|') || n.eq(&'7') || n.eq(&'F') {
                    nsew.0 = Some(Dir::N);
                }

                // check south
                let s = css[row + 1][col];
                if s.eq(&'|') || s.eq(&'L') || s.eq(&'J') {
                    nsew.1 = Some(Dir::S);
                    dir = Dir::S
                }

                // check east
                let e = css[row][col + 1];
                if e.eq(&'-') || e.eq(&'J') || e.eq(&'7') {
                    nsew.2 = Some(Dir::E);
                    dir = Dir::E
                }

                // check west
                let w = css[row][col - 1];
                if w.eq(&'-') || w.eq(&'F') || w.eq(&'L') {
                    nsew.3 = Some(Dir::W);
                    dir = Dir::W
                }

                let ch = match nsew {
                    (Some(Dir::N), Some(Dir::S), None, None) => '|',
                    (None, None, Some(Dir::E), Some(Dir::W)) => '-',
                    (Some(Dir::N), None, Some(Dir::E), None) => 'L',
                    (Some(Dir::N), None, None, Some(Dir::W)) => 'J',
                    (None, Some(Dir::S), None, Some(Dir::W)) => '7',
                    (None, Some(Dir::S), Some(Dir::E), None) => 'F',
                    _ => panic!("'S' had invalid neighbors"),
                };
                return (row, col, dir, ch);
            }
        }
    }
    panic!("failed to find 'S'")
}

#[allow(dead_code)]
fn show_grid(css: &Vec<Vec<char>>) {
    for cs in css {
        for c in cs {
            print!("{c}")
        }
        println!()
    }
    println!()
}

fn parse(ls: &[String]) -> Vec<Vec<char>> {
    let mut css: Vec<Vec<char>> = ls.iter().map(|l| l.chars().collect()).collect();

    // ring the entire 2d-array with '.' for easier indexing when doing neighbor discovery
    for cs in css.iter_mut() {
        cs.insert(0, '.');
        cs.push('.');
    }
    let dots: Vec<char> = ".".repeat(css[0].len()).chars().collect();
    css.insert(0, dots.clone());
    css.push(dots);

    css
}

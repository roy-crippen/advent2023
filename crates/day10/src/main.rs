fn main() {
    let css: Vec<Vec<char>> = include_str!("input.txt").lines().map(|l| l.chars().collect()).collect();
    // for cs in &css {
    //     println!("{cs:?}")
    // }

    println!("day10-a = {}", solve_a(&css)); // 6714
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
    assert!(!css.is_empty());
    assert!(!css[0].is_empty());

    let (mut row, mut col, mut dir) = find_start_pos(css);
    let mut steps: usize = 0;
    while dir != Dir::Done {
        steps += 1;
        let ch = css[row][col];
        // println!("ch = {ch}, dir = {:?}", dir.clone());
        (row, col, dir) = next_pos(ch, row, col, dir);
    }

    steps / 2
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

fn find_start_pos(css: &Vec<Vec<char>>) -> (usize, usize, Dir) {
    for (row, cs) in css.iter().enumerate() {
        for (col, c) in cs.iter().enumerate() {
            if c.eq(&'S') {
                // check north
                if row > 0 {
                    let n = css[row - 1][col];
                    if n.eq(&'|') || n.eq(&'7') || n.eq(&'F') {
                        return (row - 1, col, Dir::N);
                    }
                }

                // check south
                if row < css.len() {
                    let s = css[row + 1][col];
                    if s.eq(&'|') || s.eq(&'L') || s.eq(&'J') {
                        return (row + 1, col, Dir::S);
                    }
                }

                // check east
                if col > 0 {
                    let e = css[row][col - 1];
                    if e.eq(&'-') || e.eq(&'J') || e.eq(&'7') {
                        return (row, col - 1, Dir::E);
                    }
                }

                // must be west
                return (row, col + 1, Dir::W);
            }
        }
    }
    panic!("failed to find 'S'")
}

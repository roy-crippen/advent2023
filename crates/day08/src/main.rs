use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let map = parse(&lines);
    println!("day08-a = {}", solve('a', "AAA", &map)); // 16897
    println!("day08-b = {}", solve_b(&map)); // 16563603485021
}

struct Map {
    m: HashMap<String, (String, String)>,
    rev_instructions: String,
}

fn solve(part: char, start_key: &str, map: &Map) -> usize {
    let mut current_key = start_key;
    let mut rev_instructions: String = map.rev_instructions.clone();
    let mut steps = 0;
    loop {
        steps += 1;
        // get more instructions if needed
        if rev_instructions.is_empty() {
            rev_instructions = map.rev_instructions.clone();
        }

        // set the current_key based on the next instruction
        let (l_key, r_key) = map.m.get(current_key).unwrap();
        current_key = if rev_instructions.pop().unwrap().eq(&'L') { l_key } else { r_key };

        // check for end condition for part a or b
        if (current_key.eq("ZZZ") && part.eq(&'a')) || (current_key.ends_with('Z') && part.eq(&'b')) {
            return steps;
        }
    }
}

fn solve_b(map: &Map) -> usize {
    // find the solution count for each key ending in 'A'
    let solutions: Vec<usize> = map.m.keys().filter(|key| key.ends_with('A')).map(|key| solve('b', key, map)).collect();

    // return the least common multiple of the list of solutions
    assert!(solutions.len() > 1);
    solutions.iter().skip(2).fold(lcm(solutions[0], solutions[1]), |acc, v| lcm(acc, *v))
}

// https://rustp.org/number-theory/lcm/
fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

fn parse(lines: &[String]) -> Map {
    assert!(lines.len() > 2);
    let mut it = lines.iter();
    let rev_instructions = it.next().unwrap().chars().rev().collect();
    it.next();
    let mut m: HashMap<String, (String, String)> = HashMap::new();
    for line in it {
        let (key, s) = line.split_once('=').unwrap();
        let (ls, rs) = s.split_once(',').unwrap();
        let ls = ls.trim();
        let rs = rs.trim();
        m.insert(key.trim().to_string(), (ls[1..].to_string(), rs[..rs.len() - 1].to_string()));
    }

    Map { m, rev_instructions }
}

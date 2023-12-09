fn main() {
    let ss: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|l| l.to_owned())
        .collect();
    let (va, vb) = solve(&ss, 12, 13, 14);
    println!("day02-a = {va}"); // 2679
    println!("day02-b = {vb}") // 77607
}

fn solve(ss: &[String], nr: u32, ng: u32, nb: u32) -> (u32, u32) {
    let mut va: u32 = 0;
    let mut vb: u32 = 0;
    for (i, s1) in ss.iter().enumerate() {
        let mut a_fail = false;
        let mut mr = 0;
        let mut mg = 0;
        let mut mb = 0;
        for s2 in s1.split_once(':').unwrap().1.split(';') {
            for s3 in s2.split(',') {
                let vs: Vec<&str> = s3.trim().split(' ').collect();
                let n = vs[0].parse::<u32>().unwrap();
                let c = vs[1];
                match c {
                    "red" => {
                        mr = mr.max(n);
                        a_fail = a_fail || n > nr
                    }
                    "green" => {
                        mg = mg.max(n);
                        a_fail = a_fail || n > ng
                    }
                    _ => {
                        mb = mb.max(n);
                        a_fail = a_fail || n > nb
                    }
                }
            }
        }
        if !a_fail {
            va += i as u32 + 1;
        }
        vb += mr * mg * mb;
    }
    (va, vb)
}

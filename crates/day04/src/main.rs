fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let mut cards = parse(&lines);

    println!("day04-a = {}", solve_a(&cards)); // 25010
    println!("day04-b = {}", solve_b(&mut cards)); // 9924412
}

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    win_cnt: usize,
    copies: usize,
}

/// Find part 1 solution from the list of cards
fn solve_a(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|card| {
            if card.win_cnt == 0 {
                0
            } else {
                2usize.pow(card.win_cnt as u32 - 1)
            }
        })
        .sum()
}

/// Find part 2 solution from the list of cards
fn solve_b(cards: &mut [Card]) -> usize {
    let ids: Vec<usize> = cards.iter().map(|card| card.id).collect();
    for id in ids {
        let end = cards[id - 1].win_cnt + id;
        for i in id..end {
            if i < cards.len() {
                cards[i].copies += cards[id - 1].copies;
            }
        }
    }
    cards.iter().map(|card| card.copies).sum()
}

/// Parse input `lines` to create a list of Cards
fn parse(lines: &[String]) -> Vec<Card> {
    let get_vals = |in_str: &str| -> Vec<usize> {
        in_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<usize>().unwrap_or(0usize))
            .collect()
    };
    let mut cards: Vec<Card> = Vec::new();
    for (id, line) in lines.iter().enumerate() {
        let xs: Vec<&str> = line.split_once(':').unwrap().1.split('|').map(|s| s.trim()).collect();
        let winners: Vec<usize> = get_vals(xs[0]);
        let attempts: Vec<usize> = get_vals(xs[1]);
        let win_cnt = attempts.iter().map(|v| if winners.contains(v) { 1 } else { 0 }).sum();
        cards.push(Card { id: id + 1, win_cnt, copies: 1 });
    }
    cards
}

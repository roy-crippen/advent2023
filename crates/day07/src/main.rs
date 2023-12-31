fn main() {
    let lines: Vec<String> = include_str!("input.txt").lines().map(|l| l.to_owned()).collect();
    let hands = parse(&lines);

    println!("day07-a = {}", solve_a(&hands)); // 256448566
    println!("day07-b = {}", solve_b(&hands)); // 254412181
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: usize,
}

fn solve_a(_hands: &[Hand]) -> usize {
    let hands: Vec<Hand> = _hands.to_vec();
    finish(hands)
}

fn solve_b(_hands: &[Hand]) -> usize {
    let mut hands: Vec<Hand> = _hands.to_vec();

    for hand in hands.iter_mut() {
        // mutate 'J' card value from 11 to 1
        hand.cards = hand.cards.iter().map(|card| if *card == 11 { 1 } else { *card }).collect();

        // update HandType
        let joker_cnt = hand.cards.iter().filter(|card| **card == 1).count();
        match hand.hand_type {
            HandType::FiveKind => (),
            HandType::FourKind => {
                if joker_cnt == 1 || joker_cnt == 4 {
                    hand.hand_type = HandType::FiveKind
                }
            }
            HandType::FullHouse => {
                if joker_cnt > 0 {
                    hand.hand_type = HandType::FiveKind
                }
            }
            HandType::ThreeKind => {
                if joker_cnt == 3 || joker_cnt == 1 {
                    hand.hand_type = HandType::FourKind
                }
            }
            HandType::TwoPair => match joker_cnt {
                1 => hand.hand_type = HandType::FullHouse,
                2 => hand.hand_type = HandType::FourKind,
                _ => (),
            },
            HandType::OnePair => {
                if joker_cnt == 1 || joker_cnt == 2 {
                    hand.hand_type = HandType::ThreeKind
                }
            }
            HandType::HighCard => {
                if joker_cnt == 1 {
                    hand.hand_type = HandType::OnePair
                }
            }
        }
    }

    finish(hands)
}

fn finish(mut hands: Vec<Hand>) -> usize {
    hands.sort_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then(a.cards[0].cmp(&b.cards[0]))
            .then(a.cards[1].cmp(&b.cards[1]))
            .then(a.cards[2].cmp(&b.cards[2]))
            .then(a.cards[3].cmp(&b.cards[3]))
            .then(a.cards[4].cmp(&b.cards[4]))
    });
    hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

fn parse(lines: &[String]) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let (s1, s2) = line.split_once(' ').unwrap();
        let cards: Vec<u32> = s1
            .trim()
            .chars()
            .map(|ch| match ch {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => ch.to_digit(10).unwrap(),
            })
            .collect();
        let hand_type = get_hand_type(&cards);
        let bid = s2.trim().parse::<usize>().unwrap_or(0usize);

        hands.push(Hand { cards, hand_type, bid })
    }

    hands
}

fn get_hand_type(cards: &Vec<u32>) -> HandType {
    assert_eq!(cards.len(), 5);
    let mut sorted_cards = cards.clone();
    sorted_cards.sort();
    let mut dedup_cards = sorted_cards.clone();
    dedup_cards.dedup();

    match dedup_cards.len() {
        1 => HandType::FiveKind,
        2 => {
            // FullHouse or FourKind
            for w in sorted_cards.windows(4) {
                if w[0] == w[1] && w[0] == w[2] && w[0] == w[3] {
                    return HandType::FourKind;
                }
            }
            HandType::FullHouse
        }
        3 => {
            // ThreeKind or TwoPair
            for w in sorted_cards.windows(3) {
                if w[0] == w[1] && w[0] == w[2] {
                    return HandType::ThreeKind;
                }
            }
            HandType::TwoPair
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

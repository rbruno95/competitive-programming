#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap};

fn part_1(input: &str) -> u32 {
    let mut input = deserialize(input, false);
    input.sort_unstable_poker_hands_with_custom_rank_fn(PokerHand::rank_with_jacks);
    input.calculate_sum_of_bid_times_index()
}

fn part_2(input: &str) -> u32 {
    let mut input = deserialize(input, true);
    input.sort_unstable_poker_hands_with_custom_rank_fn(PokerHand::rank_with_jokers);
    input.calculate_sum_of_bid_times_index()
}

#[derive(Debug)]
struct Input {
    poker_hands_with_bid: Vec<PokerHandWithBid>,
}

impl Input {
    fn sort_unstable_poker_hands_with_custom_rank_fn(&mut self, rank_fn: fn(&PokerHand) -> u32) {
        self.poker_hands_with_bid.sort_unstable_by(|a, b| {
            let a_type = rank_fn(&a.poker_hand);
            let b_type = rank_fn(&b.poker_hand);

            if a_type < b_type {
                Ordering::Less
            } else if a_type > b_type {
                Ordering::Greater
            } else {
                a.poker_hand
                    .cards
                    .iter()
                    .zip(b.poker_hand.cards.iter())
                    .map(|(a_card_ith, b_card_ith)| {
                        if a_card_ith < b_card_ith {
                            Ordering::Less
                        } else if a_card_ith > b_card_ith {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    })
                    .filter(|x| x != &Ordering::Equal)
                    .next()
                    .unwrap()
            }
        });
    }

    fn calculate_sum_of_bid_times_index(&self) -> u32 {
        self.poker_hands_with_bid
            .iter()
            .enumerate()
            .map(|(index, card_with_bid)| (index as u32 + 1) * card_with_bid.bid)
            .sum()
    }
}

#[derive(Debug)]
struct PokerHandWithBid {
    poker_hand: PokerHand,
    bid: u32,
}

#[derive(Debug)]
struct PokerHand {
    cards: Vec<Card>,
}

impl PokerHand {
    fn rank_with_jacks(&self) -> u32 {
        self.rank_hand(self.rank_init())
    }

    fn rank_with_jokers(&self) -> u32 {
        let mut cards_hashmap = self.rank_init();

        let jokers = cards_hashmap.get(&Card::Joker);

        let card_with_greater_number_of_occurrences = cards_hashmap
            .iter()
            .filter(|&(key, _)| key != &Card::Joker)
            .max_by(|(_, value_1), (_, value_2)| value_1.cmp(&value_2))
            .map(|(key, _)| key);

        if let (Some(&jokers_amount), Some(&card_with_greater_number_of_occurrences)) =
            (jokers, card_with_greater_number_of_occurrences)
        {
            cards_hashmap
                .entry(card_with_greater_number_of_occurrences)
                .and_modify(|value| *value += jokers_amount);

            cards_hashmap.remove_entry(&Card::Joker);
        }

        self.rank_hand(cards_hashmap)
    }

    fn rank_init(&self) -> HashMap<Card, u32> {
        let mut cards_hashmap = HashMap::new();
        self.cards.iter().for_each(|&card| {
            cards_hashmap
                .entry(card)
                .and_modify(|amount| *amount += 1)
                .or_insert(1);
        });
        cards_hashmap
    }

    fn rank_hand(&self, cards_hashmap: HashMap<Card, u32>) -> u32 {
        let differents_cards_in_hand = cards_hashmap.len();
        let ocurrences_of_random_card_in_hand = cards_hashmap.values().next().unwrap();

        if differents_cards_in_hand == 1 {
            7
        } else if differents_cards_in_hand == 2
            && [1, 4].contains(ocurrences_of_random_card_in_hand)
        {
            6
        } else if differents_cards_in_hand == 2 {
            5
        } else if differents_cards_in_hand == 3 && cards_hashmap.iter().any(|x| x.1 == &3) {
            4
        } else if differents_cards_in_hand == 3 {
            3
        } else if differents_cards_in_hand == 4
            && [1, 2].contains(ocurrences_of_random_card_in_hand)
        {
            2
        } else {
            1
        }
    }
}

#[derive(Debug, Copy, Eq, PartialEq, PartialOrd, Clone, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from(card_value: char, with_jokers_instead_of_jacks: bool) -> Self {
        match card_value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => match with_jokers_instead_of_jacks {
                true => Self::Joker,
                false => Self::Jack,
            },
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

fn deserialize(input: &str, with_jokers_instead_of_jacks: bool) -> Input {
    Input {
        poker_hands_with_bid: input
            .lines()
            .map(|line| {
                line.split_once(' ')
                    .map(|(poker_hand, bid)| PokerHandWithBid {
                        bid: bid.parse::<u32>().unwrap(),
                        poker_hand: PokerHand {
                            cards: poker_hand
                                .chars()
                                .map(|card_value| {
                                    Card::from(card_value, with_jokers_instead_of_jacks)
                                })
                                .collect(),
                        },
                    })
                    .unwrap()
            })
            .collect(),
    }
}

#[test]
fn test_day_07() {
    let sample = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample), 6440);
    assert_eq!(part_1(input), 251106089);
    assert_eq!(part_2(sample), 5905);
    assert_eq!(part_2(input), 249620106);
}

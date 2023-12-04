#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, game) = line.split_once(':').unwrap();
            let (winning_numbers, numbers) = game.split_once('|').unwrap();

            let winning_numbers = winning_numbers
                .split_ascii_whitespace()
                .collect::<HashSet<&str>>();

            let amount_winner_numbers = numbers
                .split_ascii_whitespace()
                .filter(|x| winning_numbers.contains(x))
                .count();

            if amount_winner_numbers > 0 {
                2_u32.pow(amount_winner_numbers as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let mut cards_amount_by_number = HashMap::<usize, u32>::new();

    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let current_amount = *cards_amount_by_number
                .entry(index)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            let (_, game) = line.split_once(':').unwrap();
            let (winning_numbers, numbers) = game.split_once('|').unwrap();

            let winning_numbers = winning_numbers
                .split_ascii_whitespace()
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();

            let amount_winner_numbers = numbers
                .split_ascii_whitespace()
                .filter(|x| !x.is_empty() && winning_numbers.contains(x))
                .count();

            for i in 0..amount_winner_numbers {
                cards_amount_by_number
                    .entry(index + 1 + i)
                    .and_modify(|v| *v += current_amount)
                    .or_insert(current_amount);
            }

            *cards_amount_by_number.get(&index).unwrap()
        })
        .sum()
}

#[test]
fn test_day_n() {
    let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample), 13);
    assert_eq!(part_1(input), 21919);
    assert_eq!(part_2(sample), 30);
    assert_eq!(part_2(input), 9881048);
}

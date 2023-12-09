#![allow(dead_code)]

use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

fn part_1(input: &str) -> usize {
    let cubes_max_amount_restriction_by_color =
        HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let mut game_id = index + 1;

            line.split(&[',', ' ', ';', ':'])
                .filter(|x| !x.is_empty())
                .skip(2) // To skip Game <id>
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|cube| {
                    (
                        cube[0]
                            .parse::<u32>()
                            .expect("This value should be a number"),
                        cube[1],
                    )
                })
                .for_each(|(amount, color)| {
                    if cubes_max_amount_restriction_by_color
                        .get(&color)
                        .expect("Color {color} not expected")
                        < &amount
                    {
                        game_id = 0;
                    }
                });

            game_id
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut cubes_minimum_amount_by_color =
                HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            line.split(&[',', ' ', ';', ':'])
                .filter(|x| !x.is_empty())
                .skip(2) // To skip Game <id>
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|cube| {
                    (
                        cube[0]
                            .parse::<u32>()
                            .expect("This value should be a number"),
                        cube[1],
                    )
                })
                .for_each(
                    |(amount, color)| match cubes_minimum_amount_by_color.entry(color) {
                        Occupied(mut entry) => {
                            if entry.get() < &amount {
                                entry.insert(amount);
                            }
                        }
                        Vacant(_) => panic!("Color {color} not expected"),
                    },
                );

            cubes_minimum_amount_by_color.values().product::<u32>()
        })
        .sum()
}

#[test]
fn test_day_02() {
    let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample), 8);
    assert_eq!(part_1(&input), 2369);
    assert_eq!(part_2(sample), 2286);
    assert_eq!(part_2(&input), 66363);
}

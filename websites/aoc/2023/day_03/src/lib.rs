#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part_1(input: &str) -> u32 {
    let engine = input.lines().collect::<Vec<_>>();

    let mut part_numbers_sum = 0;

    engine.iter().enumerate().for_each(|(i, row)| {
        let mut next_j = 0;
        let mut candidate_part_number = 0;
        let mut has_adjacent_symbol = false;

        row.chars().enumerate().for_each(|(j, v)| {
            if v.is_ascii_digit() && j >= next_j {
                let mut k = j;

                while k < row.len() && (row.as_bytes()[k] as char).is_ascii_digit() {
                    DIRECTIONS.iter().for_each(|(mov_i, mov_j)| {
                        let new_i = i as i32 + mov_i;
                        let new_j = k as i32 + mov_j;

                        if new_i >= 0
                            && new_j >= 0
                            && new_i < engine.len() as i32
                            && new_j < row.len() as i32
                        {
                            let adjacent_char =
                                engine[new_i as usize].as_bytes()[new_j as usize] as char;

                            if !adjacent_char.is_ascii_digit() && adjacent_char != '.' {
                                has_adjacent_symbol = true;
                            }
                        }
                    });

                    candidate_part_number = candidate_part_number * 10
                        + (row.as_bytes()[k] as char).to_digit(10).unwrap();

                    k += 1;
                }

                if has_adjacent_symbol {
                    part_numbers_sum += candidate_part_number;
                }

                next_j = k;
                has_adjacent_symbol = false;
                candidate_part_number = 0;
            }
        })
    });

    part_numbers_sum
}

fn part_2(input: &str) -> u32 {
    let engine = input.lines().collect::<Vec<_>>();

    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();

    engine.iter().enumerate().for_each(|(i, row)| {
        let mut next_j = 0;
        let mut candidate_part_number = 0;
        let mut adjacent_gears = HashSet::<(usize, usize)>::new();

        row.chars().enumerate().for_each(|(j, v)| {
            if v.is_ascii_digit() && j >= next_j {
                let mut k = j;

                while k < row.len() && (row.as_bytes()[k] as char).is_ascii_digit() {
                    DIRECTIONS.iter().for_each(|(i_mov, j_mov)| {
                        let new_i = i as i32 + i_mov;
                        let new_j = k as i32 + j_mov;

                        if new_i >= 0
                            && new_j >= 0
                            && new_i < engine.len() as i32
                            && new_j < row.len() as i32
                        {
                            let adjacent_char =
                                engine[new_i as usize].as_bytes()[new_j as usize] as char;

                            if adjacent_char == '*'
                                && !adjacent_gears.contains(&(new_i as usize, new_j as usize))
                            {
                                adjacent_gears.insert((new_i as usize, new_j as usize));
                            }
                        }
                    });

                    candidate_part_number = candidate_part_number * 10
                        + (row.as_bytes()[k] as char).to_digit(10).unwrap();

                    k += 1;
                }

                adjacent_gears.iter().for_each(|(i, j)| {
                    gears
                        .entry((*i, *j))
                        .and_modify(|candidate_numbers| {
                            candidate_numbers.push(candidate_part_number)
                        })
                        .or_insert(vec![candidate_part_number]);
                });

                next_j = k;
                adjacent_gears.clear();
                candidate_part_number = 0;
            }
        })
    });

    gears
        .values()
        .into_iter()
        .map(|part_numbers| {
            if part_numbers.len() == 2 {
                part_numbers.iter().product()
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test_day_03() {
    let sample = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample), 4361);
    assert_eq!(part_1(input), 544433);
    assert_eq!(part_2(sample), 467835);
    assert_eq!(part_2(input), 76314915);
}

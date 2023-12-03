#![allow(dead_code)]

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();

            if !digits.is_empty() {
                digits.first().unwrap() * 10 + digits.last().unwrap()
            } else {
                panic!("There is no digit in line {line}")
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits_with_indices = line
                .char_indices()
                .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
                .collect::<Vec<_>>();

            let digits_with_indices_mapped_from_spelled_digits = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .flat_map(|pattern| {
                line.match_indices(pattern).map(|(i, x)| {
                    (
                        i,
                        match x {
                            "one" => 1,
                            "two" => 2,
                            "three" => 3,
                            "four" => 4,
                            "five" => 5,
                            "six" => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine" => 9,
                            _ => panic!("Not expected string"),
                        },
                    )
                })
            });

            digits_with_indices.extend(digits_with_indices_mapped_from_spelled_digits);
            digits_with_indices.sort_unstable_by_key(|d| d.0);

            if !digits_with_indices.is_empty() {
                digits_with_indices.first().unwrap().1 * 10 + digits_with_indices.last().unwrap().1
            } else {
                panic!("There is no digit in line {line}")
            }
        })
        .sum()
}

#[test]
fn test_day_1() {
    let sample_part_1 = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let sample_part_2 = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample_part_1), 142);
    assert_eq!(part_1(input), 56397);
    assert_eq!(part_2(sample_part_2), 281);
    assert_eq!(part_2(input), 55701);
}

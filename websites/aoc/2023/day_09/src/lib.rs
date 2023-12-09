#![allow(dead_code)]

fn part_1(input: &str) -> i64 {
    deserialize(input).sum_next_number_of_each_sequence()
}

fn part_2(input: &str) -> i64 {
    let mut input = deserialize(input);

    input.sequences.iter_mut().for_each(Sequence::reverse);

    input.sum_next_number_of_each_sequence()
}

struct Input {
    sequences: Vec<Sequence>,
}

impl Input {
    fn sum_next_number_of_each_sequence(&self) -> i64 {
        self.sequences.iter().map(Sequence::guess_next_number).sum()
    }
}

struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn guess_next_number(&self) -> i64 {
        let mut sum = 0;

        let mut values = self.values.clone();

        while values.iter().any(|x| x != &0) {
            sum += values.last().unwrap();
            values = values
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<_>>();
        }

        sum
    }

    fn reverse(&mut self) {
        self.values = self.values.iter().rev().map(ToOwned::to_owned).collect()
    }
}

fn deserialize(input: &str) -> Input {
    Input {
        sequences: input
            .lines()
            .map(|line| Sequence {
                values: line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            })
            .collect(),
    }
}

#[test]
fn test_day_09() {
    let sample = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample), 114);
    assert_eq!(part_1(input), 1842168671);
    assert_eq!(part_2(sample), 2);
    assert_eq!(part_2(input), 903);
}

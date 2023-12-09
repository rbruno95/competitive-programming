#![allow(dead_code)]

use std::collections::HashMap;

fn part_1(input: &str) -> u64 {
    let input = deserialize(input);

    number_of_steps_from_source_to_satisfy_condition(&input, "AAA", |str| str == "ZZZ")
}

fn part_2(input: &str) -> u64 {
    let input = deserialize(input);

    let starts = input
        .locations
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();

    let gcd = |mut a, mut b| {
        while b != 0 {
            if b < a {
                std::mem::swap(&mut b, &mut a);
            }
            b %= a;
        }
        a
    };
    let mcm = |a, b| a * b / gcd(a, b);

    starts
        .iter()
        .map(|start| {
            number_of_steps_from_source_to_satisfy_condition(&input, start, |str| {
                str.ends_with('Z')
            })
        })
        .fold(1, |a, b| mcm(a, b))
}

#[derive(Debug)]
struct Input<'l> {
    instructions: Vec<Direction>,
    locations: HashMap<&'l str, NodeNeighboursByDirection<'l>>,
}

#[derive(Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug)]
struct NodeNeighboursByDirection<'l> {
    left_neighbour: &'l str,
    right_neighbour: &'l str,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::L,
            'R' => Self::R,
            _ => unreachable!(),
        }
    }
}

fn number_of_steps_from_source_to_satisfy_condition(
    input: &Input,
    source_node_label: &str,
    condition: fn(&str) -> bool,
) -> u64 {
    let mut counter = 0;
    let mut current_node_label = source_node_label;

    loop {
        for instruction in input.instructions.iter() {
            let current_node = input.locations.get(current_node_label).unwrap();

            match instruction {
                Direction::L => current_node_label = current_node.left_neighbour,
                Direction::R => current_node_label = current_node.right_neighbour,
            };

            counter += 1;

            if condition(current_node_label) {
                return counter;
            }
        }
    }
}

fn deserialize(input: &str) -> Input {
    let (instructions, locations) = input
        .split_once("\n\n")
        .map(|(instructions, locations)| {
            (
                instructions.chars().map(Into::into).collect(),
                locations
                    .split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|direction| {
                        direction
                            .split_once(" = ")
                            .map(|(label, connections)| {
                                (
                                    label,
                                    connections
                                        .split_once(", ")
                                        .map(|(left, right)| NodeNeighboursByDirection {
                                            left_neighbour: &left[1..],
                                            right_neighbour: &right[..3],
                                        })
                                        .unwrap(),
                                )
                            })
                            .unwrap()
                    })
                    .collect(),
            )
        })
        .unwrap();

    Input {
        instructions,
        locations,
    }
}

#[test]
fn test_day_08() {
    let sample1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let sample2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    let sample3 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample1), 2);
    assert_eq!(part_1(sample2), 6);
    assert_eq!(part_1(input), 14429);
    assert_eq!(part_2(sample3), 6);
    assert_eq!(part_2(input), 10921547990923);
}

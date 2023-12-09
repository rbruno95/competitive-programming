#![allow(dead_code)]

fn part_1(input: &str) -> u64 {
    let input = deserialize_input_part_1(input);

    input
        .seeds
        .iter()
        .map(|seed| {
            let mut current_seed_mapped = *seed;

            input.mappings.iter().for_each(|mappings| {
                mappings
                    .iter()
                    .filter_map(|&(final_start, source_start, range)| {
                        if source_start <= current_seed_mapped
                            && current_seed_mapped <= source_start + range
                        {
                            Some(current_seed_mapped + final_start - source_start)
                        } else {
                            None
                        }
                    })
                    .next()
                    .map(|new_value| current_seed_mapped = new_value);
            });

            current_seed_mapped
        })
        .min()
        .unwrap()
}

fn part_2(input: &str) -> u64 {
    let input = deserialize_input_part_2(input);

    input
        .seeds_ranges
        .iter()
        .map(|&(seed_start, seed_range)| {
            let mut current_seeds = vec![(seed_start, seed_range, 0)];

            input
                .mappings
                .iter()
                .enumerate()
                .for_each(|(index, mappings)| {
                    let index = index + 1;
                    mappings
                        .iter()
                        .for_each(|&(final_start, source_start, range)| {
                            let mut new_seeds_ranges = Vec::new();

                            current_seeds
                                .iter()
                                .filter(|(_, _, level)| *level + 1 == index)
                                .for_each(|(seed_start, seed_range, level)| {
                                    if *seed_start <= source_start
                                        && source_start < *seed_start + seed_range
                                    {
                                        new_seeds_ranges.push((
                                            *seed_start,
                                            source_start - seed_start,
                                            *level,
                                        ));

                                        if source_start + range <= seed_start + seed_range {
                                            new_seeds_ranges.push((source_start, range, level + 1));
                                            new_seeds_ranges.push((
                                                source_start + range,
                                                seed_start + seed_range - (source_start + range),
                                                *level,
                                            ))
                                        } else {
                                            new_seeds_ranges.push((
                                                source_start,
                                                *seed_start + seed_range - source_start,
                                                *level + 1,
                                            ));
                                        }
                                    } else if source_start <= *seed_start
                                        && *seed_start < source_start + range
                                    {
                                        if source_start + range <= *seed_start + seed_range {
                                            new_seeds_ranges.push((
                                                *seed_start,
                                                source_start + range - *seed_start,
                                                level + 1,
                                            ));
                                            new_seeds_ranges.push((
                                                source_start + range,
                                                seed_start + seed_range - (source_start + range),
                                                *level,
                                            ));
                                        } else {
                                            new_seeds_ranges.push((
                                                *seed_start,
                                                *seed_range,
                                                level + 1,
                                            ));
                                        }
                                    } else {
                                        new_seeds_ranges.push((*seed_start, *seed_range, *level));
                                    }
                                });

                            current_seeds.retain(|(_, _, level)| *level == index);
                            new_seeds_ranges
                                .iter_mut()
                                .filter(|(_, _, level)| *level == index)
                                .for_each(|(seed_start, _, _)| {
                                    *seed_start += final_start;
                                    *seed_start -= source_start;
                                });
                            current_seeds.extend(new_seeds_ranges);
                        });

                    current_seeds
                        .iter_mut()
                        .filter(|(_, _, level)| *level + 1 == index)
                        .for_each(|(_, _, level)| *level += 1);
                });

            current_seeds
                .into_iter()
                .filter(|(_, range, _)| *range > 0)
                .map(|(seed_start, _, _)| seed_start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

struct InputPart1 {
    seeds: Vec<u64>,
    mappings: Vec<Vec<(u64, u64, u64)>>,
}

fn deserialize_input_part_1(input: &str) -> InputPart1 {
    let mut lines = input.split("\n\n");

    let seeds = lines
        .next()
        .unwrap()
        .split_once("seeds: ")
        .map(|(_, seeds)| {
            seeds
                .split_ascii_whitespace()
                .filter_map(|seed| seed.parse::<u64>().ok())
        })
        .unwrap()
        .collect::<Vec<_>>();

    let mappings = lines
        .map(|mapping_lines| {
            mapping_lines
                .split('\n')
                .skip(1) // skip mapping name
                .filter(|x| !x.is_empty())
                .map(|mapping| {
                    match &mapping
                        .split_ascii_whitespace()
                        .map(|number| number.parse::<u64>().expect("{number} is not a number"))
                        .collect::<Vec<_>>()[..]
                    {
                        &[a, b, c, ..] => (a, b, c),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    InputPart1 { seeds, mappings }
}

struct InputPart2 {
    seeds_ranges: Vec<(u64, u64)>,
    mappings: Vec<Vec<(u64, u64, u64)>>,
}

fn deserialize_input_part_2(input: &str) -> InputPart2 {
    let input_part_1 = deserialize_input_part_1(input);

    InputPart2 {
        seeds_ranges: input_part_1
            .seeds
            .chunks(2)
            .map(|seeds_range| match seeds_range {
                &[a, b, ..] => (a, b),
                _ => unreachable!(),
            })
            .collect(),
        mappings: input_part_1.mappings,
    }
}

#[test]
fn test_day_05() {
    let sample = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample), 35);
    assert_eq!(part_1(input), 214922730);
    assert_eq!(part_2(sample), 46);
    assert_eq!(part_2(input), 148041808);
}

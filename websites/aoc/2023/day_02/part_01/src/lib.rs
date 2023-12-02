#[allow(dead_code)]
fn sum_possible_game_ids(input: &str) -> usize {
    struct CubeMaxAmountRestrictionByColor {
        color: &'static str,
        max_amount: u32,
    }

    let cubes_max_amount_restriction_by_color = vec![
        CubeMaxAmountRestrictionByColor {
            color: "red",
            max_amount: 12,
        },
        CubeMaxAmountRestrictionByColor {
            color: "green",
            max_amount: 13,
        },
        CubeMaxAmountRestrictionByColor {
            color: "blue",
            max_amount: 14,
        },
    ];

    input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            let mut game_id = index + 1;

            line.split(&[',', ' ', ';', ':'])
                .into_iter()
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
                        .iter()
                        .any(|cmarbc| cmarbc.color == color && cmarbc.max_amount < amount)
                    {
                        game_id = 0;
                    } else if !cubes_max_amount_restriction_by_color
                        .iter()
                        .any(|cmarbc| cmarbc.color == color)
                    {
                        panic!("Color: {color} not expected")
                    }
                });

            game_id
        })
        .sum()
}

#[test]
fn test_sum_possible_game_ids_sample() {
    let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(sum_possible_game_ids(sample), 8);
}

#[test]
fn test_sum_possible_game_ids_input() {
    let input =
        std::fs::read_to_string("../input.txt").expect("Should have been able to read the file");

    assert_eq!(sum_possible_game_ids(&input), 2369)
}

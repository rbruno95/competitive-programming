#[allow(dead_code)]
fn calculate_sum_of_powers(input: &str) -> u32 {
    struct CubeMinimumAmountByColor {
        color: &'static str,
        minimum_amount: u32,
    }

    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut cubes_minimum_amount_by_color = vec![
                CubeMinimumAmountByColor {
                    color: "red",
                    minimum_amount: 0,
                },
                CubeMinimumAmountByColor {
                    color: "green",
                    minimum_amount: 0,
                },
                CubeMinimumAmountByColor {
                    color: "blue",
                    minimum_amount: 0,
                },
            ];

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
                    cubes_minimum_amount_by_color.iter_mut().for_each(|cmabc| {
                        if cmabc.color == color && cmabc.minimum_amount < amount {
                            cmabc.minimum_amount = amount;
                        }
                    });

                    if !cubes_minimum_amount_by_color
                        .iter()
                        .any(|cmabc| cmabc.color == color)
                    {
                        panic!("Color: {color} not expected")
                    }
                });

            cubes_minimum_amount_by_color
                .iter()
                .map(|cmabc| cmabc.minimum_amount)
                .product::<u32>()
        })
        .sum()
}

#[test]
fn test_calculate_sum_of_powers_sample() {
    let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(calculate_sum_of_powers(sample), 2286);
}

#[test]
fn test_calculate_sum_of_powers_input() {
    let input =
        std::fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    assert_eq!(calculate_sum_of_powers(&input), 66363)
}

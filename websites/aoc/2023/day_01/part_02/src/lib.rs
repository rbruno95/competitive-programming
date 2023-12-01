#[allow(dead_code)]
fn sum_calibration_values(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
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
fn test_sum_calibration_values_sample() {
    let sample = "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen";

    assert_eq!(sum_calibration_values(sample), 281);
}

#[test]
fn test_sum_calibration_values_input() {
    let input =
        std::fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    assert_eq!(sum_calibration_values(&input), 55701)
}

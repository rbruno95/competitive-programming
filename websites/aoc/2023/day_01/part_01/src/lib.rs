#[allow(dead_code)]
fn sum_calibration_values(input: &str) -> u32 {
    input
        .lines()
        .into_iter()
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

#[test]
fn test_sum_calibration_values_sample() {
    let sample = "1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet";

    assert_eq!(sum_calibration_values(sample), 142);
}

#[test]
fn test_sum_calibration_values_input() {
    let input =
        std::fs::read_to_string("../input.txt").expect("Should have been able to read the file");

    assert_eq!(sum_calibration_values(&input), 56397)
}

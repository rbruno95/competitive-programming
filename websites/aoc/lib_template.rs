#[allow(dead_code)]
fn solve(input: &str) -> u32 {
    input.lines().into_iter().map(|line| {}).sum()
}

#[test]
fn test_solve_sample() {
    let sample = "";

    assert_eq!(solve(sample), 0);
}

#[test]
fn test_solve_input() {
    let input =
        std::fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    assert_eq!(solve(&input), 0)
}

#![allow(dead_code)]

fn part_1(input: &str) -> u64 {
    deserialize_part_1(input)
        .races_records
        .iter()
        .map(RaceRecord::number_of_ways_to_beat_the_record_efficient)
        .product()
}

fn part_2(input: &str) -> u64 {
    deserialize_part_2(input)
        .race_record
        .number_of_ways_to_beat_the_record_efficient()
}

struct InputPart1 {
    races_records: Vec<RaceRecord>,
}

struct InputPart2 {
    race_record: RaceRecord,
}

struct RaceRecord {
    time: u64,
    distance: u64,
}

impl RaceRecord {
    /// Algorithm: Brute force
    ///
    /// Time complexity: `O(time)`
    fn number_of_ways_to_beat_the_record(&self) -> u64 {
        (0..self.time)
            .into_iter()
            .filter(|x| x * (self.time - x) > self.distance)
            .count() as u64
    }

    /// Algorithm:
    ///
    /// Solving the inequality: `x * (t - x) > d`
    ///
    /// Two solutions are obtained:
    ///
    /// `x_1 = ((t - (t * t - 4. * d).sqrt()) / 2.).ceil()`
    ///
    /// `x_2 = ((t + (t * t - 4. * d).sqrt()) / 2.).ceil()`
    ///
    /// So, `x_1` is the least integer such that `x_1 * (t - x_1) >= d`
    ///
    /// In case of the equality, we need to add `1` to `x_1`, let's call it `x_1f`
    ///
    /// Because we know that the problem is symmetric (if `x` works iff `t - x` works as well)
    ///
    /// it's sufficient to use `x_1f`
    ///
    /// so `1, 2, ..., x_1f - 1` don't work, but neither do `t - 1, t - 2, ..., t - (x_1f - 1)`
    ///
    /// In addition, `t` doesn't work either
    ///
    /// Then, the solution is `t - 2 * (x_1f - 1) - 1`
    ///
    /// Time complexity: `O(1)`
    fn number_of_ways_to_beat_the_record_efficient(&self) -> u64 {
        let t = self.time as f64;
        let d = self.distance as f64;

        let mut min_x = ((t - (t * t - 4. * d).sqrt()) / 2.).ceil() as u64;

        if min_x * (self.time - min_x) == self.distance {
            min_x += 1;
        }

        self.time - 2 * (min_x - 1) - 1
    }
}

fn deserialize_part_1(input: &str) -> InputPart1 {
    let mut lines = input.lines().filter(|x| !x.is_empty());

    let mut deserialize_line_as_iterator_delimiting_on = |delimiter: &str| {
        lines
            .next()
            .unwrap()
            .split_once(delimiter)
            .map(|(_, times)| {
                times
                    .split_ascii_whitespace()
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
            })
            .unwrap()
    };

    let time_iter = deserialize_line_as_iterator_delimiting_on("Time: ");
    let distance_iter = deserialize_line_as_iterator_delimiting_on("Distance: ");

    InputPart1 {
        races_records: time_iter
            .zip(distance_iter)
            .map(|(time_ith, distance_ith)| RaceRecord {
                time: time_ith,
                distance: distance_ith,
            })
            .collect(),
    }
}

fn deserialize_part_2(input: &str) -> InputPart2 {
    let mut lines = input.lines().filter(|x| !x.is_empty());

    let mut deserialize_line_as_number_delimiting_on = |delimiter: &str| {
        lines
            .next()
            .unwrap()
            .split_once(delimiter)
            .map(|(_, times)| {
                times
                    .split_ascii_whitespace()
                    .filter(|x| !x.is_empty())
                    .fold(String::new(), |a, b| a + b)
                    .parse::<u64>()
                    .unwrap()
            })
            .unwrap()
    };

    let time = deserialize_line_as_number_delimiting_on("Time: ");
    let distance = deserialize_line_as_number_delimiting_on("Distance: ");

    InputPart2 {
        race_record: RaceRecord { time, distance },
    }
}

#[test]
fn test_day_06() {
    let sample = "Time:      7  15   30
Distance:  9  40  200";
    let input = include_str!("input.txt");

    assert_eq!(part_1(sample), 288);
    assert_eq!(part_1(input), 840336);
    assert_eq!(part_2(sample), 71503);
    assert_eq!(part_2(input), 41382569);
}

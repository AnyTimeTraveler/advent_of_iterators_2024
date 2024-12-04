use crate::get_input;

#[test]
fn test_part1_example() {
    assert_eq!(
        part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        ),
        2
    );
}

#[test]
fn test_part1_full() {
    assert_eq!(part1(&get_input(file!())), 218);
}

#[test]
fn test_part2_example() {
    assert_eq!(
        part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        ),
        4
    );
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(&get_input(file!())), 290);
    // 273: too low
    // 275: too low
    // 293: too high
}

fn part1(input: &str) -> i64 {
    input.lines().filter(|line| is_safe(line)).count() as i64
}

fn part2(input: &str) -> i64 {
    input.lines().filter(|line| is_mostly_safe(line)).count() as i64
}

#[test]
fn test_is_safe() {
    assert_eq!(is_safe("7 6 4 2 1"), true);
    assert_eq!(is_safe("1 2 7 8 9"), false);
    assert_eq!(is_safe("9 7 6 2 1"), false);
    assert_eq!(is_safe("1 3 2 4 5"), false);
    assert_eq!(is_safe("8 6 4 4 1"), false);
    assert_eq!(is_safe("1 3 6 7 9"), true);
}

#[test]
fn test_is_mostly_safe() {
    assert_eq!(is_mostly_safe("7 6 4 2 1"), true);
    assert_eq!(is_mostly_safe("1 2 7 8 9"), false);
    assert_eq!(is_mostly_safe("9 7 6 2 1"), false);
    assert_eq!(is_mostly_safe("1 3 2 4 5"), true);
    assert_eq!(is_mostly_safe("8 6 4 4 1"), true);
    assert_eq!(is_mostly_safe("1 3 6 7 9"), true);
    assert_eq!(is_mostly_safe("14 13 12 11 10 7 5 8"), true);
}

fn is_safe(line: &str) -> bool {
    is_numbers_safe(&to_numbers(line))
}

fn is_numbers_safe(numbers: &[i64]) -> bool {
    let all_increasing = numbers.windows(2).all(|w| w[0] <= w[1]);
    let all_decreasing = numbers.windows(2).all(|w| w[0] >= w[1]);
    let maximum_difference = numbers
        .windows(2)
        .map(|w| (w[0] - w[1]).abs())
        .all(|diff| 1 <= diff && diff <= 3);

    maximum_difference && (all_increasing || all_decreasing)
}

fn to_numbers(line: &str) -> Vec<i64> {
    line.split(" ")
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn is_mostly_safe(line: &str) -> bool {
    if is_safe(line) {
        return true;
    }
    let numbers = to_numbers(line);

    match get_outlier(&numbers, |w| {
        let diff = (w[0] - w[1]).abs();
        1 <= diff && diff <= 3
    }) {
        None => {
            // no outliers in rate of change

            if let Some(outlier_idx) = get_outlier(&numbers, |w| !(|w: &[i64]| w[0] <= w[1])(w)) {
                if is_numbers_safe(&with_gap(&numbers, outlier_idx))
                    || is_numbers_safe(&with_gap(&numbers, outlier_idx + 1))
                {
                    return true;
                }
            }
            if let Some(outlier_idx) = get_outlier(&numbers, |w| !(|w: &[i64]| w[0] >= w[1])(w)) {
                if is_numbers_safe(&with_gap(&numbers, outlier_idx))
                    || is_numbers_safe(&with_gap(&numbers, outlier_idx + 1))
                {
                    return true;
                }
            }
            false
        }
        Some(outlier_idx) => {
            is_numbers_safe(&with_gap(&numbers, outlier_idx))
                || is_numbers_safe(&with_gap(&numbers, outlier_idx + 1))
        }
    }
}

fn get_outlier(numbers: &Vec<i64>, closure: fn(&[i64]) -> bool) -> Option<usize> {
    numbers.windows(2).position(|w| !closure(w))
}

fn with_gap(numbers: &[i64], pos: usize) -> Vec<i64> {
    let mut vec = numbers.to_vec();
    vec.remove(pos);
    vec
}

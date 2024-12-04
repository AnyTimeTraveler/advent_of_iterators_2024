use crate::get_input;
use std::collections::HashMap;

#[test]
fn test_part1_example() {
    assert_eq!(
        part1(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        ),
        11
    );
}

#[test]
fn test_part1_full() {
    assert_eq!(part1(&get_input(file!())), 2264607);
}

#[test]
fn test_part2_example() {
    assert_eq!(
        part2(
            "3   4
4   3
2   5
1   3
3   9
3   3"
        ),
        31
    );
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(&get_input(file!())), 19457120);
}

fn part1(input: &str) -> i64 {
    let (left, right) = sort_lists(input);

    left.iter().zip(right).map(|(a, b)| (b - a).abs()).sum()
}

fn part2(input: &str) -> i64 {
    let (left, right) = sort_lists(input);

    let map = right.iter().fold(HashMap::new(), |mut acc, e| {
        *acc.entry(*e).or_insert(0) += 1;
        acc
    });

    left.iter().map(|n| map.get(n).unwrap_or(&0) * n).sum()
}

fn sort_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .for_each(|(a, b)| {
            left.push(a);
            right.push(b)
        });

    left.sort();
    right.sort();
    (left, right)
}

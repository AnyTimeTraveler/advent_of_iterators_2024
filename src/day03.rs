use crate::day03::Find::{Do, Dont, Number};
use crate::get_input;

#[test]
fn test_part1_full() {
    assert_eq!(day03_part1(&get_input(file!())), 184511516);
}

#[test]
fn test_part2_full() {
    assert_eq!(day03_part2(&get_input(file!())), 90044227);
}

#[test]
fn test_part1_example() {
    assert_eq!(
        day03_part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        161
    );
}

#[test]
fn test_part2_example() {
    assert_eq!(
        day03_part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    );
}

fn day03_part1(input: &str) -> i64 {
    let mut result = 0;
    let mut pos = 0;
    while pos < input.len() {
        match find_next(&input[pos..]) {
            Some((number, offset)) => {
                result += number;
                pos += offset
            }
            None => {
                pos += 4;
            }
        }
    }
    result
}

enum Find {
    Do,
    Dont,
    Number(i64),
}

fn day03_part2(input: &str) -> i64 {
    let mut instructions: Vec<(usize, Find)> = Vec::new();

instructions.extend(
    input
        .match_indices("mul(")
        .map(|(i, _)| (i, find_next(&input[i..])))
        .filter_map(|(i, f)| f.map(|(number, _)| (i, Number(number)))),
);
instructions.extend(input.match_indices("do()").map(|(i, _)| (i, Do)));
instructions.extend(input.match_indices("don't()").map(|(i, _)| (i, Dont)));

instructions.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut result = 0;
    let mut active = true;
    for (_pos, instruction) in instructions {
        match instruction {
            Do => active = true,
            Dont => active = false,
            Number(value) => {
                if active {
                    result += value
                }
            }
        }
    }

    result
}

fn find_next(input: &str) -> Option<(i64, usize)> {
    let mul_pos = input.find("mul(")? + 4;
    let comma_pos = input[mul_pos..].find(",")? + mul_pos;
    let brac_pos = input[comma_pos..].find(")")? + comma_pos;

    let num_a: i64 = input[mul_pos..comma_pos].parse().ok()?;
    let num_b: i64 = input[comma_pos + 1..brac_pos].parse().ok()?;

    Some((num_a * num_b, brac_pos))
}

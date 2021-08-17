/// Puzzle from advent of code 2020, day 1
/// You can also read it from: https://adventofcode.com/2020/day/1
/// If you prefer, you can also swap the input's file content with the one you got from there
///
/// Puzzle instructions
///
/// There's a big list of numbers inside the file called `input`.
/// There you have two entries that sum to 2020 find those entries and then multiply those two numbers together.
///
/// Given the following input
///
/// 1456
/// 979
/// 366
/// 1721
/// 299
/// 675
///
/// The two numbers that sums to 2020 are `1721` and `299` and the product of them is `514579`
///
/// Plus, lets add a bit more to this challenge, try to:
///
/// 1. Define some output you expect from the function before you start your hacking,
///    modify the test case to match that output, it can be anything, few examples:
///    a) a string -> 'The numbers are 1721 and 299 and the product is 514579',
///    b) an array -> [1721, 299, 514579]
///    c) a tuple -> ("Product: 514579", 1721, 299)
///
///    Stick to the format you choose, no matter if it's easier to return a different format.
///    You don't know the values yet (or do you?), so, just hard code some zeroes at the test.
///    E.g.: assert_eq!(find_two(), 'The entries are 0, 0 and the product is 0');
///
/// 2. Fix the return type of the find_two function with the same output you choose on the step
///    before. If you are using the docker provided or you have cargo-watch installed locally
///    you can run `cargo watch -x test advent_of_code::tests` to let if run on every file change
///
use std::fs;

#[test]
fn tests() {
    assert_eq!(find_two(), "");
}

pub fn find_two() -> String {
    let file = fs::read_to_string("src/advent_of_code/input").expect("file not found");
    file
}

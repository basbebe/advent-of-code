use anyhow::Result;
use std::collections::HashMap;
use std::str::FromStr;

pub fn run(input: &str) -> Result<()> {
    let part_one = part_one(input);
    println!("part one: {part_one}");

    let part_two = part_two(input);
    println!("part two: {part_two}");

    Ok(())
}

pub fn part_one(input: &str) -> String {
    let (mut left, mut right) = get_tuples(input);

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(left_item, right_item)| (left_item - right_item).abs())
        .sum::<isize>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    let (left, right) = get_tuples(input);

    let r = right
        .iter()
        .fold(HashMap::<isize, isize>::new(), |mut acc, el| {
            let value = acc.entry(*el).or_insert(0);
            *value += 1;
            acc
        });
    left.iter()
        .map(|n| r.get(n).unwrap_or(&0) * n)
        .sum::<isize>()
        .to_string()
}

fn get_tuples(input: &str) -> (Vec<isize>, Vec<isize>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                // ignore parsing errors
                .flat_map(isize::from_str)
                .take(2)
        })
        .map(|mut tuple| {
            (
                tuple.next().expect("Lines consist of two numbers"),
                tuple.next().expect("Lines consist of two numbers"),
            )
        })
        .unzip()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3
";
        let result = part_one(input);
        assert_eq!(result, "11".to_string());
    }

    #[test]
    fn test_part_two() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3
";
        let result = part_two(input);
        assert_eq!(result, "31".to_string());
    }
}

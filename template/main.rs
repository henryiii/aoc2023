/*!
# 2023 Day X - ...

<https://adventofcode.com/2023/day/X>

*/

type Num = i64;

fn compute1(text: &str) -> Num {
    0
}

fn compute2(text: &str) -> Num {
    0
}

fn main() {
    let text = std::fs::read_to_string("input/15.txt").unwrap();
    let result = compute1(&text);
    println!("First = {result}");

    let second_result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
";

    #[test]
    fn test_first() {
        let result = compute1(INPUT);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_first() {
        let result = compute2(INPUT);
        assert_eq!(result, 0);
    }
}

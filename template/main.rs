#![warn(clippy::all, clippy::pedantic)]

type Num = i64;

fn compute(text: &str) -> Num {
    0
}

fn main() {
    let text = std::fs::read_to_string("input/14.txt").unwrap();
    let first_result = compute(&text);
    println!("First = {first_result}");

    let second_result = compute(&text);
    println!("Second = {second_result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
";

    #[test]
    fn test_first() {
        let result = compute(&text);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_first() {
        let result = compute(&text);
        assert_eq!(result, 0);
    }
}

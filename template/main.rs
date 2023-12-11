#![warn(clippy::all, clippy::pedantic)]

fn compute(text: &str) -> i64 {
    0
}

fn main() {
    let text = std::fs::read_to_string("input/11.txt").unwrap();
    let result = compute(&text);
    println!("Results = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
";

    #[test]
    fn test() {
        let result = compute(INPUT);
        assert_eq!(result, 1);
    }
}

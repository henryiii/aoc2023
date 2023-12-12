#![warn(clippy::all, clippy::pedantic)]

fn compute(text: &str) -> (i64, i64) {
    (0, 0)
}

fn main() {
    let text = std::fs::read_to_string("input/13.txt").unwrap();
    let (first_result, second_result) = compute(&text);
    println!("First = {first_result}");
    println!("Second = {second_result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
";

    #[test]
    fn test() {
        let (first_result, second_result) = compute(&text);
        assert_eq!(first_result, 0);
        assert_eq!(second_result, 0);
    }
}

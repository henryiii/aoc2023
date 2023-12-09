use std::io::prelude::*;


fn main() {
    let file = std::fs::File::open("input/06.txt").unwrap();
    let lines_res = std::io::BufReader::new(file).lines();
    let mut lines = lines_res.map(|x| x.unwrap());
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
";

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}

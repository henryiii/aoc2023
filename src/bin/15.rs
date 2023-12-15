#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use indexmap::IndexMap;

type Num = usize;

fn compute(text: &str) -> Num {
    let arr = str_to_array(text);
    arr.into_iter().map(hash).sum()
}

fn compute2(text: &str) -> Num {
    let arr = hashtable(&str_to_array(text));
    arr.into_iter()
        .enumerate()
        .map(|(i, x)| {
            x.values()
                .enumerate()
                .map(|(j, x)| (i + 1) * (j + 1) * x)
                .sum::<Num>()
        })
        .sum()
}

fn str_to_array(text: &str) -> Vec<&str> {
    text.trim().split(',').collect()
}

fn hash(text: &str) -> Num {
    text.bytes()
        .fold(0, |acc, x| ((acc + Num::from(x)) * 17) % 256)
}

fn hashtable<'a>(input: &[&'a str]) -> Vec<IndexMap<&'a str, Num>> {
    let mut arr: Vec<_> = (0..256).map(|_| IndexMap::<&str, Num>::new()).collect();

    for x in input {
        if let Some((code, val)) = x.split_once('=') {
            let val: Num = val.parse().unwrap();
            arr[hash(code)].insert(code, val);
        } else if let Some((code, _)) = x.split_once('-') {
            arr[hash(code)].shift_remove(code);
        }
    }
    arr
}

fn main() {
    let text = std::fs::read_to_string("input/15.txt").unwrap();
    let first_result = compute(&text);
    println!("First = {first_result}");

    let second_result = compute2(&text);
    println!("Second = {second_result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_first() {
        let result = compute(INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_second() {
        let result = compute2(INPUT);
        assert_eq!(result, 145);
    }
}

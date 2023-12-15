#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

/*!
# Problem 15 - HASH and HASHMAP

This problem is very simple if you have an ordered hashmap. I originally
solved this one on my phone in Python; for comparison, here it is:

```python
INPUT = 'rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7'

def hsh(x):
    s = 0
    for v in x:
        s += ord(v)
        s *= 17
        s %= 256
    return s

print(sum(hsh(x) for x in INPUT.split(',')))

def hshmap(x):
    boxes = {i: {} for i in range(256)}
    for val in x:
        code, op, fl = val.partition('=')
        if op:
            boxes[hsh(code)][code] = int(fl)
        else:
            code = val.removesuffix('-')
            boxes[hsh(code)].pop(code, None)

    return sum((b + 1)
               * sum(n * fl for n, fl in enumerate(slts.values(), 1))
               for b, slts in boxes.items())

print(hshmap(INPUT.split(',')))
```
*/
use indexmap::IndexMap;

fn compute_hash(text: &str) -> usize {
    let arr = str_to_array(text);
    arr.into_iter().map(hash).sum()
}

fn compute_hashmap(text: &str) -> usize {
    let arr = hashtable(&str_to_array(text));
    arr.into_iter()
        .enumerate()
        .map(|(i, x)| {
            x.values()
                .enumerate()
                .map(|(j, x)| (i + 1) * (j + 1) * x)
                .sum::<usize>()
        })
        .sum()
}

#[inline]
fn str_to_array(text: &str) -> Vec<&str> {
    text.trim().split(',').collect()
}

fn hash(text: &str) -> usize {
    text.bytes()
        .fold(0, |acc, x| ((acc + usize::from(x)) * 17) % 256)
}

fn hashtable<'a>(input: &[&'a str]) -> Vec<IndexMap<&'a str, usize>> {
    let mut arr: Vec<_> = (0..256).map(|_| IndexMap::<&str, usize>::new()).collect();

    for x in input {
        if let Some((code, val)) = x.split_once('=') {
            let val: usize = val.parse().unwrap();
            arr[hash(code)].insert(code, val);
        } else if let Some((code, _)) = x.split_once('-') {
            arr[hash(code)].shift_remove(code);
        }
    }
    arr
}

fn main() {
    let text = std::fs::read_to_string("input/15.txt").unwrap();
    let result = compute_hash(&text);
    println!("HASH = {result}");

    let result = compute_hashmap(&text);
    println!("HASHMAP = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        let result = compute_hash(INPUT);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_hashmap() {
        let result = compute_hashmap(INPUT);
        assert_eq!(result, 145);
    }
}

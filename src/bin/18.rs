/*!
# 2023 Day 18 - Diggers

<https://adventofcode.com/2023/day/18>

This is best solved by considering the area added each move.  It was easier for
part 1 and necessary for part 2.

Moving up or down adds a block of area; as long as you get the sign right, you
can get the total enclosed area.  You also need the parimeter, which is 1/2
already acounted for by the area. Corner pieces are 3/4 uncounted, so we need an
extra 1/4 for each of the four corners required to make the loop. (Any inner
corners are canceled out by the extra outer corners required.)
Python solution from my phone (Pythonista):

```python
directions = [(x.split()[0], int(x.split()[1])) for x in txt.splitlines()]

def get_area(directions):
    area = 0
    p = 0
    loc = [0, 0]
    for d, l in directions:
        p += l
        match d:
            case "R":
                loc[1] += l
            case "L":
                loc[1] -= l
            case "D":
                area += loc[1]*l
                loc[0] += l
            case "U":
                area -= loc[1]*l
                loc[0] -= l

    return area + p//2 + 1

directions2 = [("RDLU"[int(x.split()[2][-2])], int(x.split()[2][2:-2], 16)) for x in txt.splitlines()]

print(get_area(directions))
print(get_area(directions2))
```

*/

use itertools::Itertools;

fn read_directions(text: &str) -> Vec<(char, i64)> {
    text.lines()
        .map(|x| {
            let parts = x.split_whitespace();
            let (dir, len, _) = parts.collect_tuple().unwrap();
            (dir.chars().next().unwrap(), len.parse().unwrap())
        })
        .collect()
}

fn read_directions_2(text: &str) -> Vec<(char, i64)> {
    text.lines()
        .map(|x| {
            let parts = x.split_whitespace();
            let (_, _, hexstr) = parts.collect_tuple().unwrap();
            let hexstr = &hexstr[2..hexstr.len() - 1];
            let d_int = hexstr.chars().last().unwrap().to_digit(16).unwrap();
            let dir = ['R', 'D', 'L', 'U'][d_int as usize];
            let hex = i64::from_str_radix(&hexstr[..hexstr.len() - 1], 16).unwrap();
            (dir, hex)
        })
        .collect()
}

fn get_area(dirs: &[(char, i64)]) -> i64 {
    let mut area = 0;
    let mut p = 0;
    let mut loc = [0, 0];
    for (d, l) in dirs {
        p += l;
        match d {
            'R' => loc[1] += l,
            'L' => loc[1] -= l,
            'D' => {
                area += loc[1] * l;
                loc[0] += l;
            }
            'U' => {
                area -= loc[1] * l;
                loc[0] -= l;
            }
            _ => panic!("Got {d}, expected R, L, D, or U"),
        }
    }
    area + p / 2 + 1
}

fn compute(text: &str) -> i64 {
    let dirs = read_directions(text);
    get_area(&dirs)
}

fn compute_2(text: &str) -> i64 {
    let dirs = read_directions_2(text);
    get_area(&dirs)
}

fn main() {
    let text = std::fs::read_to_string("input/18.txt").unwrap();
    let result = compute(&text);
    println!("First = {result}");

    let result = compute_2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_first() {
        let result = compute(INPUT);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_second() {
        let result = compute_2(INPUT);
        assert_eq!(result, 952408144115);
    }
}

/*!
# 2023 Day 24 - Trajectories

<https://adventofcode.com/2023/day/24>

Parts 1 and 2 are really different here. The key trick to part 1 is
understanding that the time parameter must be positive for both trajectories.


```python
from pathlib import Path
import itertools
from contextlib import suppress

def read(fn):
    txt = Path(fn).read_text()
    lines = [t.replace('@', ' ').split() for t in txt.splitlines()]
    return [tuple(int(x.strip(',')) for x in a) for a in lines]

def intersect_2d(a, b):
    x1, y1, z1, dx1, dy1, dz1 = a
    x2, y2, z2, dx2, dy2, dz2 = b
    # x1 + t*dx1 == x2 + q*dx2
    # y1 + t*dy1 == y2 + q*dy2

    # dx1/dy1 == (x2 + q*dx2 - x1)/(y2 + q*dy2 - y1)
    # dx1*(y2 + q*dy2 - y1) == dy1*(x2 + q*dx2 - x1)
    # dx1*y2 + dx1*q*dy2 - dx1*y1 == dy1*x2 + dy1*q*dx2 - dy1*x1
    # dx1*q*dy2 - dy1*q*dx2 == dy1*x2 - dy1*x1 + dx1*y1 - dx1*y2

    q = (dy1*x2 - dy1*x1 + dx1*y1 - dx1*y2) / (dx1*dy2 - dy1*dx2)
    t = (x2 + q*dx2 - x1)/dx1 if dx1 != 0 else (x1 + q*dx1 - x2)/dx2
    x3 = x2 + q*dx2
    y3 = y2 + q*dy2

    return (x3, y3, q, t)

def intersect_in(vals, low, high):
    vals = [(x-low, y-low, z, dx, dy, dz) for x, y, z, dx, dy, dz in vals]
    high -= low
    for (a, b) in itertools.combinations(vals, 2):
        with suppress(ZeroDivisionError):
            x, y, q, t = intersect_2d(a, b)
            if q>=0 and t>=0 and 0<=x<=high and 0<=y<=high:
                yield (a, b, (x, y))

vals = read('24test.txt')
seen = list(intersect_in(vals, 7, 27))
print(len(seens))


vals = read('24data.txt')
seen = list(intersect_in(vals, 200000000000000, 400000000000000))
print(len(seen))


```

*/
#![allow(clippy::many_single_char_names, clippy::cast_precision_loss)]

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vector<T> {
    x: T,
    y: T,
    z: T,
}

impl From<Vector<i64>> for Vector<f64> {
    fn from(value: Vector<i64>) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
            z: value.z as f64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    p: Vector<i64>,
    d: Vector<i64>,
}

fn read(text: &str) -> Vec<Line> {
    text.lines()
        .map(|line| {
            let (x, y, z, dx, dy, dz) = line
                .replace('@', ",")
                .replace(' ', "")
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Line {
                p: Vector { x, y, z },
                d: Vector {
                    x: dx,
                    y: dy,
                    z: dz,
                },
            }
        })
        .collect()
}

#[allow(clippy::suboptimal_flops)]
fn intersect_2d(a: &Line, b: &Line) -> Option<(f64, f64, f64, f64)> {
    let ad: Vector<f64> = a.d.clone().into();
    let bd: Vector<f64> = b.d.clone().into();
    let ap: Vector<f64> = a.p.clone().into();
    let bp: Vector<f64> = b.p.clone().into();

    let top = ad.y * bp.x - ad.y * ap.x + ad.x * ap.y - ad.x * bp.y;
    let bot = a.d.x * b.d.y - a.d.y * b.d.x;
    if bot == 0 {
        return None;
    }
    let q = top / bot as f64;
    let t = if a.d.x == 0 {
        (ap.x + q * ad.x - bp.x) / bd.x
    } else {
        (bp.x + q * bd.x - ap.x) / ad.x
    };
    let x = bd.x.mul_add(q, bp.x);
    let y = bd.y.mul_add(q, bp.y);
    Some((x, y, q, t))
}

fn intersect_2d_in(vals: &[Line], low: i64, high: i64) -> impl Iterator<Item = (f64, f64)> + '_ {
    vals.iter()
        .map(move |a| Line {
            p: Vector::<i64> {
                x: a.p.x - low,
                y: a.p.y - low,
                z: a.p.z,
            },
            d: a.d.clone(),
        })
        .combinations(2)
        .filter_map(move |pair| {
            let (a, b) = pair.into_iter().collect_tuple().unwrap();
            let (x, y, q, t) = intersect_2d(&a, &b)?;
            if q >= 0.0
                && t >= 0.0
                && x >= 0.0
                && y >= 0.0
                && x <= (high - low) as f64
                && y <= (high - low) as f64
            {
                Some((x + low as f64, y + low as f64))
            } else {
                None
            }
        })
}

fn compute1(text: &str, low: i64, high: i64) -> usize {
    let hail_lines = read(text);
    intersect_2d_in(&hail_lines, low, high).count()
}

const fn compute2(_text: &str) -> i64 {
    0
}

fn main() {
    let text = std::fs::read_to_string("input/24.txt").unwrap();
    let result = compute1(&text, 200_000_000_000_000, 400_000_000_000_000);
    println!("First = {result}");

    let result = compute2(&text);
    println!("Second = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_first() {
        let result = compute1(INPUT, 7, 27);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_second() {
        let result = compute2(INPUT);
        assert_eq!(result, 0);
    }
}

#[cfg(feature = "progressbar")]
use indicatif::ProgressIterator;

use derive_new::new;

type Range = std::ops::Range<u64>;

struct Mapper {
    range: Range,
    to: u64,
}

impl Mapper {
    fn convert(&self, value: u64) -> Option<u64> {
        if self.range.contains(&value) {
            Some(value - self.range.start + self.to)
        } else {
            None
        }
    }
}

#[derive(new)]
struct Mappers {
    #[new(default)]
    mappers: Vec<Mapper>,
}

impl Mappers {
    fn convert(&self, value: u64) -> u64 {
        for mapper in &self.mappers {
            if let Some(result) = mapper.convert(value) {
                return result;
            }
        }
        value
    }
}

#[derive(new)]
struct AllMappers {
    #[new(default)]
    mappers: Vec<Mappers>,
}

impl AllMappers {
    fn convert(&self, value: u64) -> u64 {
        let mut value = value;
        for mapper in &self.mappers {
            value = mapper.convert(value);
        }
        value
    }
}

fn read<'a>(lines: impl Iterator<Item = &'a str>) -> (Vec<u64>, AllMappers) {
    let mut lines = lines.into_iter();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert!(lines.next().unwrap().is_empty());

    let mut all_mappers = AllMappers::new();
    for _ in 0..7 {
        let mut mappers = Mappers::new();
        assert!(lines.next().unwrap().ends_with("map:"));
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.split_whitespace();
            let to = parts.next().unwrap().parse::<u64>().unwrap();
            let from = parts.next().unwrap().parse::<u64>().unwrap();
            let size = parts.next().unwrap().parse::<u64>().unwrap();
            mappers.mappers.push(Mapper {
                range: (from..from + size),
                to,
            });
        }
        all_mappers.mappers.push(mappers);
    }
    (seeds, all_mappers)
}

fn seeds_as_ranges_brute_force(seeds: &[u64]) -> impl Iterator<Item = u64> + '_ {
    let pairs = seeds.iter().step_by(2).zip(seeds.iter().skip(1).step_by(2));
    let seeds = pairs.map(|(from, len)| (*from..*from + *len));
    seeds.flat_map(|r| (r.start)..(r.end))
}

fn main() {
    let text = std::fs::read_to_string("input/05.txt").unwrap();
    let (seeds, all_mappers) = read(text.lines());
    let min = seeds.iter().map(|x| all_mappers.convert(*x)).min().unwrap();
    println!("Min: {min}");

    let seed_iter = seeds_as_ranges_brute_force(&seeds);

    #[cfg(feature = "progressbar")]
    let seed_iter = seed_iter.progress_count(seeds.iter().skip(1).step_by(2).sum());

    let min = seed_iter.map(|x| all_mappers.convert(x)).min().unwrap();

    println!("Min assuming ranges: {min}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_05() {
        let (seeds, all_mappers) = read(INPUT.lines());
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(all_mappers.mappers.len(), 7);
        assert_eq!(all_mappers.mappers[0].convert(79), 81);
        assert_eq!(all_mappers.mappers[0].convert(14), 14);
        assert_eq!(all_mappers.mappers[0].convert(55), 57);
        assert_eq!(all_mappers.mappers[0].convert(13), 13);

        assert_eq!(all_mappers.mappers[1].convert(81), 81);
        assert_eq!(all_mappers.mappers[2].convert(81), 81);
        assert_eq!(all_mappers.mappers[3].convert(81), 74);
        assert_eq!(all_mappers.mappers[4].convert(74), 78);
        assert_eq!(all_mappers.mappers[5].convert(78), 78);
        assert_eq!(all_mappers.mappers[6].convert(78), 82);

        assert_eq!(all_mappers.convert(79), 82);
        assert_eq!(all_mappers.convert(14), 43);
        assert_eq!(all_mappers.convert(55), 86);
        assert_eq!(all_mappers.convert(13), 35);

        let min = seeds.iter().map(|x| all_mappers.convert(*x)).min().unwrap();
        assert_eq!(min, 35);
    }

    #[test]
    fn test_05b_brute_force() {
        let (seeds, all_mappers) = read(INPUT.lines());
        let all_seeds = seeds_as_ranges_brute_force(&seeds);
        let min = all_seeds.map(|x| all_mappers.convert(x)).min().unwrap();
        assert_eq!(min, 46);
    }
}

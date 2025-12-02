use advent::prelude::*;

#[derive(Debug, HasParser)]
#[parse(sep_by = "-")]
struct Range {
    left: u64,
    right: u64,
}

impl Range {
    fn contains(&self, v: u64) -> bool {
        v >= self.left && v <= self.right
    }
}

fn expand(v: u64, n: u32) -> u64 {
    let x = v.ilog(10) + 1;
    let mut vv = v;
    for _ in 0..(n - 1) {
        vv = vv * 10u64.pow(x) + v
    }
    vv
}

#[test]
fn twice_test() {
    assert_eq!(expand(1, 2), 11);
    assert_eq!(expand(12, 2), 1212);
    assert_eq!(expand(12, 3), 121212);
    assert_eq!(expand(12, 4), 12121212);
    assert_eq!(expand(998, 2), 998998);
    assert_eq!(expand(998, 3), 998998998);
}

#[part_one]
fn part_one(input: List<Range, SepBy<Comma>>) -> u64 {
    let mut total = 0;
    let range_max = input.iter().map(|r| r.right).max().unwrap();
    let end = 10u64.pow(range_max.ilog(10) / 2 + 1);
    for v in 1..end {
        let n = expand(v, 2);
        for r in &input {
            if r.contains(n) {
                total += n;
            }
        }
    }
    total
}

#[part_two]
fn part_two(input: List<Range, SepBy<Comma>>) -> u64 {
    let range_max = input.iter().map(|r| r.right).max().unwrap();
    let end = 10u64.pow(range_max.ilog(10) / 2 + 1);
    let range_digits = range_max.ilog(10) + 1;

    let mut numbers = HashSet::new();
    for v in 1..end {
        let v_digits = v.ilog(10) + 1;
        let max_expand = range_digits / v_digits;
        for x in 2..=max_expand {
            let n = expand(v, x);
            for r in &input {
                if r.contains(n) {
                    numbers.insert(n);
                }
            }
        }
    }
    numbers.into_iter().sum()
}

harness!(part_1: 54641809925, part_2: 73694270688);

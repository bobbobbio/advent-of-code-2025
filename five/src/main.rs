use advent::prelude::*;

#[derive(HasParser, Debug)]
#[parse(sep_by = "-")]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, v: u64) -> bool {
        v >= self.start && v <= self.end
    }
}

#[derive(HasParser, Debug)]
#[parse(sep_by = "\n")]
struct Input {
    fresh: List<Range, TermWith<NewLine>>,
    ingredients: List<u64, TermWith<NewLine>>,
}

#[part_one]
fn part_one(input: Input) -> usize {
    input
        .ingredients
        .iter()
        .filter(|i| input.fresh.iter().any(|f| f.contains(**i)))
        .count()
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum RangePoint {
    Start,
    End,
}

#[part_two]
fn part_two(input: Input) -> u64 {
    let mut v = input
        .fresh
        .into_iter()
        .map(|r| [(r.start, RangePoint::Start), (r.end, RangePoint::End)])
        .flatten()
        .collect::<Vec<_>>();
    v.sort();

    let mut total = 0;
    let mut range_start = None;
    let mut range_depth = 0;
    for (rp_v, rp) in v {
        match rp {
            RangePoint::Start => {
                let s = rp_v;
                if range_start.is_none() {
                    range_start = Some(s);
                }
                range_depth += 1;
            },
            RangePoint::End => {
                let e = rp_v;
                range_depth -= 1;

                if let Some(s) = range_start && range_depth == 0 {
                    total += e - s + 1;
                    range_start = None;
                }
            }
        }
    }

    total
}

harness!(part_1: 712, part_2: 332998283036769);

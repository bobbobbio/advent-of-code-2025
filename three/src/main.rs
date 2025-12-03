use advent::prelude::*;

fn find_max(s: &[Digit]) -> Option<usize> {
    let mut max: Option<(usize, Digit)> = None;
    for (i, v) in s.iter().enumerate() {
        if let Some((_, max_v)) = max {
            if *v > max_v {
                max = Some((i, *v));
            }
        } else {
            max = Some((i, *v));
        }
    }
    max.map(|(i, _)| i)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Digit(u64);

impl HasParser for Digit {
    #[into_parser]
    fn parser() -> _ {
        digit().map(|c: char| Self(c.to_string().parse::<u64>().unwrap()))
    }
}

fn get_bank_joltage(bank: &[Digit], count: usize) -> u64 {
    let i = find_max(&bank[..bank.len() - (count - 1)]).unwrap();
    if count > 1 {
        let joltage = get_bank_joltage(&bank[(i + 1)..], count - 1);
        bank[i].0 * 10u64.pow((count - 1) as u32) + joltage
    } else {
        bank[i].0
    }
}

fn get_joltage(batteries: List<List<Digit, Nil>, TermWith<NewLine>>, count: usize) -> u64 {
    let mut joltage_sum = 0;
    for bank in batteries {
        let joltage = get_bank_joltage(&bank, count);
        joltage_sum += joltage;
    }
    joltage_sum
}

#[part_one]
fn part_one(batteries: List<List<Digit, Nil>, TermWith<NewLine>>) -> u64 {
    get_joltage(batteries, 2)
}

#[part_two]
fn part_two(batteries: List<List<Digit, Nil>, TermWith<NewLine>>) -> u64 {
    get_joltage(batteries, 12)
}

harness!(part_1: 17311, part_2: 171419245422055);

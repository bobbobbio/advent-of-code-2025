use advent::prelude::*;

#[derive(Debug, HasParser)]
enum Move {
    #[parse(before = "L")]
    Left(u64),
    #[parse(before = "R")]
    Right(u64),
}

#[derive(PartialEq, Eq)]
struct FieldElem<const N: u64> {
    value: u64,
}

impl<const N: u64> FieldElem<N> {
    const ZERO: Self = Self { value: 0 };

    fn new(v: u64) -> Self {
        Self { value: v % N }
    }

    /// Add v and return the number of times we crossed zero.
    fn add(&mut self, v: u64) -> u64 {
        let new_value = self.value + v;
        self.value = new_value % N;
        new_value / N
    }

    /// Subtract v and return the number of times we crossed zero.
    fn sub(&mut self, v: u64) -> u64 {
        let nv = v % N;
        if nv > self.value {
            let old_value = self.value;
            self.value = N - (nv - self.value);
            v / N + (old_value != 0) as u64
        } else {
            self.value = self.value - nv;
            v / N + (self.value == 0) as u64
        }
    }
}

#[part_one]
fn part_one(input: List<Move, TermWith<NewLine>>) -> u64 {
    let mut pos = FieldElem::<100>::new(50);
    let mut zeros = 0;
    for i in input {
        match i {
            Move::Left(d) => pos.sub(d),
            Move::Right(d) => pos.add(d),
        };
        if pos == FieldElem::ZERO {
            zeros += 1;
        }
    }
    zeros
}

#[part_two]
fn part_two(input: List<Move, TermWith<NewLine>>) -> u64 {
    let mut pos = FieldElem::<100>::new(50);
    let mut zeros = 0;
    for i in input {
        zeros += match i {
            Move::Left(d) => pos.sub(d),
            Move::Right(d) => pos.add(d),
        };
    }
    zeros
}

harness!(part_1: 962, part_2: 5782);

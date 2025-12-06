use advent::prelude::*;

#[derive(Debug)]
struct InputLine(Vec<u64>);

impl HasParser for InputLine {
    #[into_parser]
    fn parser() -> _ {
        (
            many(token(' ')).map(|_: String| ()),
            many1(u64::parser().skip(many(token(' ')).map(|_: String| ()))),
        )
            .map(|(_, v)| Self(v))
    }
}

#[derive(Debug)]
struct OperatorLine(Vec<Operator>);

impl HasParser for OperatorLine {
    #[into_parser]
    fn parser() -> _ {
        (
            many(token(' ')).map(|_: String| ()),
            many1(Operator::parser().skip(many(token(' ')).map(|_: String| ()))),
        )
            .map(|(_, v)| Self(v))
    }
}

#[derive(HasParser, Debug)]
#[parse(sep_by = "")]
struct Input {
    inputs: List<InputLine, TermWith<NewLine>>,
    operators: OperatorLine,
}

#[derive(HasParser, Debug, Clone, Copy)]
enum Operator {
    #[parse(string = "+")]
    Add,
    #[parse(string = "*")]
    Multiply,
}

impl Operator {
    fn initial(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Multiply => 1,
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

#[part_one]
fn part_one(input: Input) -> u64 {
    let mut total = 0;
    let width = input.inputs.iter().map(|i| i.0.len()).max().unwrap();
    for i in 0..width {
        let operator = input.operators.0[i];
        let mut line_total = operator.initial();
        for line in &input.inputs {
            line_total = operator.apply(line.0[i], line_total);
        }
        total += line_total;
    }
    total
}

struct Digit(u64);

impl HasParser for Digit {
    #[into_parser]
    fn parser() -> _ {
        digit().map(|d| d.to_string().parse().unwrap()).map(Self)
    }
}

#[derive(HasParser)]
enum Cell {
    Number(Digit),
    Operator(Operator),
    #[parse(string = " ")]
    Space,
}

#[part_two]
fn part_two(input: Grid<Cell>) -> u64 {
    let mut total = 0;
    let mut numbers = vec![];
    for column in input.columns().rev() {
        let mut n = None;
        for c in column {
            match c {
                Cell::Number(Digit(d)) => {
                    if let Some(nn) = n {
                        n = Some(nn * 10 + d);
                    } else {
                        n = Some(*d);
                    }
                }
                Cell::Operator(op) => {
                    numbers.push(n.take().unwrap());

                    let mut t = op.initial();
                    for n in std::mem::take(&mut numbers) {
                        t = op.apply(t, n);
                    }
                    total += t;
                }
                Cell::Space => {}
            }
        }
        if let Some(nn) = n {
            numbers.push(nn);
        }
    }
    total
}

harness!(part_1: 4771265398012, part_2: 10695785245101);

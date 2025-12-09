use advent::prelude::*;

#[derive(HasParser, Copy, Clone, DebugMore)]
#[parse(sep_by = ",")]
#[debug("({x}, {y}, {z})")]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl Position {
    fn distance(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2))
        .isqrt()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CircuitId(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BoxId(u64);

struct Circuits {
    box_to_circuit: HashMap<BoxId, CircuitId>,
    circuits: HashMap<CircuitId, HashSet<BoxId>>,
}

impl Circuits {
    fn new(size: u64) -> Self {
        let box_to_circuit: HashMap<BoxId, CircuitId> =
            (0..size).map(|bx| (BoxId(bx), CircuitId(bx + 1))).collect();
        let circuits: HashMap<CircuitId, HashSet<BoxId>> = (0..size)
            .map(|bx| {
                let mut s = HashSet::new();
                s.insert(BoxId(bx));
                (CircuitId(bx + 1), s)
            })
            .collect();
        Self {
            box_to_circuit,
            circuits,
        }
    }

    fn move_from_circuit(&mut self, from: CircuitId, to: CircuitId) {
        assert!(from != to);

        let old_members = self.circuits.remove(&from).unwrap();
        for member in &old_members {
            self.box_to_circuit.insert(*member, to);
        }
        self.circuits.get_mut(&to).unwrap().extend(old_members);
    }

    fn circuit_for_box(&self, box_id: BoxId) -> CircuitId {
        self.box_to_circuit[&box_id]
    }

    fn len(&self) -> usize {
        self.circuits.len()
    }
}

fn build_distances(
    input: &List<Position, TermWith<NewLine>>,
) -> BTreeMap<u64, Vec<(BoxId, BoxId)>> {
    let var_name = Default::default();
    let mut distances: BTreeMap<u64, Vec<(BoxId, BoxId)>> = var_name;
    for (i, a) in input.iter().enumerate() {
        for (j, b) in (&input[(i + 1)..]).iter().enumerate() {
            distances
                .entry(a.distance(b))
                .or_default()
                .push((BoxId(i as u64), BoxId((i + j + 1) as u64)));
        }
    }
    distances
}

fn connect_circuit(
    distances: &mut BTreeMap<u64, Vec<(BoxId, BoxId)>>,
    circuits: &mut Circuits,
) -> Option<(BoxId, BoxId)> {
    let Some(mut entry) = distances.first_entry() else {
        return None;
    };

    let (bx_a, bx_b) = entry.get_mut().pop().unwrap();
    if entry.get().is_empty() {
        entry.remove_entry();
    }

    let new_circuit = circuits.circuit_for_box(bx_a);
    let old_circuit = circuits.circuit_for_box(bx_b);
    if new_circuit != old_circuit {
        circuits.move_from_circuit(old_circuit, new_circuit);
        return Some((bx_a, bx_b));
    }

    None
}

#[part_one]
fn part_one(input: List<Position, TermWith<NewLine>>) -> u64 {
    let mut distances = build_distances(&input);
    let mut circuits = Circuits::new(input.len() as u64);

    for _ in 0..1000 {
        connect_circuit(&mut distances, &mut circuits);
    }

    let mut circuit_sizes = circuits
        .circuits
        .values()
        .map(|c| c.len())
        .collect::<Vec<_>>();
    circuit_sizes.sort();
    circuit_sizes.iter().rev().take(3).product::<usize>() as u64
}

#[part_two]
fn part_two(input: List<Position, TermWith<NewLine>>) -> u64 {
    let mut distances = build_distances(&input);
    let mut circuits = Circuits::new(input.len() as u64);

    let mut value = 0;
    while circuits.len() > 1 {
        if let Some((bx_a, bx_b)) = connect_circuit(&mut distances, &mut circuits) {
            value = input[bx_a.0 as usize].x * input[bx_b.0 as usize].x;
        }
    }

    value
}

harness!(part_1: 46398, part_2: 8141888143);

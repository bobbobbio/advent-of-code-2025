use advent::prelude::*;

#[derive(HasParser)]
enum Cell {
    #[parse(string = ".")]
    Empty,
    #[parse(string = "@")]
    Roll,
}

fn removable_rolls(map: &Grid<Cell>) -> Vec<(usize, usize)> {
    let mut to_remove = vec![];
    for (i, j) in map.positions() {
        if matches!(map[i][j], Cell::Roll) {
            let mut paper_neighbors = 0;
            for d in Direction8::iter() {
                if let Some((ii, jj)) = d.advance(i, j, map.width(), map.height()) {
                    if matches!(map[ii][jj], Cell::Roll) {
                        paper_neighbors += 1;
                    }
                }
            }
            if paper_neighbors < 4 {
                to_remove.push((i, j));
            }
        }
    }
    to_remove
}

#[part_one]
fn part_one(map: Grid<Cell>) -> u64 {
    removable_rolls(&map).len() as u64
}

#[part_two]
fn part_two(mut map: Grid<Cell>) -> u64 {
    let mut removed = 0;
    loop {
        let to_remove = removable_rolls(&map);
        if to_remove.is_empty() {
            break;
        }
        for (i, j) in to_remove {
            map[i][j] = Cell::Empty;
            removed += 1;
        }
    }
    removed
}

harness!(part_1: 1537, part_2: 8707);

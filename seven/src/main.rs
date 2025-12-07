use advent::prelude::*;

#[derive(HasParser, Copy, Clone)]
enum Cell {
    #[parse(string = ".")]
    Empty,
    #[parse(string = "S")]
    Start,
    #[parse(string = "^")]
    Splitter,
}

fn grid_walk1(grid: &Grid<Cell>, beams: &mut HashSet<(usize, usize)>, y: usize, x: usize) -> usize {
    if x >= grid.width() || y >= grid.height() {
        return 0;
    }
    if !beams.insert((y, x)) {
        return 0;
    }

    match grid[y][x] {
        Cell::Start | Cell::Empty => grid_walk1(grid, beams, y + 1, x),
        Cell::Splitter => {
            let mut splits = 1 + grid_walk1(grid, beams, y, x + 1);
            if x > 0 {
                splits += grid_walk1(grid, beams, y, x - 1)
            }
            splits
        }
    }
}

#[part_one]
fn part_one(grid: Grid<Cell>) -> usize {
    let (y, x) = grid.position(|c| matches!(c, Cell::Start)).unwrap();

    let mut beams = HashSet::new();
    grid_walk1(&grid, &mut beams, y + 1, x)
}

fn grid_walk2(
    grid: &Grid<Cell>,
    cache: &mut HashMap<(usize, usize), usize>,
    y: usize,
    x: usize,
) -> usize {
    if x >= grid.width() || y >= grid.height() {
        return 1;
    }

    if let Some(v) = cache.get(&(y, x)) {
        return *v;
    }

    let paths = match grid[y][x] {
        Cell::Start | Cell::Empty => grid_walk2(grid, cache, y + 1, x),
        Cell::Splitter => {
            let mut paths = grid_walk2(grid, cache, y, x + 1);
            if x > 0 {
                paths += grid_walk2(grid, cache, y, x - 1)
            }
            paths
        }
    };

    cache.insert((y, x), paths);

    paths
}

#[part_two]
fn part_two(grid: Grid<Cell>) -> usize {
    let (y, x) = grid.position(|c| matches!(c, Cell::Start)).unwrap();

    let mut cache = HashMap::new();
    grid_walk2(&grid, &mut cache, y + 1, x)
}

harness!();

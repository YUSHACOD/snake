use nanorand::{Rng, WyRand};

const ALIVE: char = 'X';
const DEAD: char = ' ';

pub fn first_gen(
    cells: &mut Vec<Vec<[char; 2]>>,
    alive_cells: usize,
    (xmax, ymax): (usize, usize),
) {
    let mut rnd = WyRand::new();
    let one_by_eight = |x: usize| x / 8;
    let seven_by_eight = |x| 7 * one_by_eight(x);
    for _ in 0..alive_cells {
        let (x, y) = (
            rnd.generate_range(one_by_eight(xmax)..seven_by_eight(xmax)),
            rnd.generate_range(one_by_eight(ymax)..seven_by_eight(ymax)),
        );
        cells[y][x][0] = ALIVE;
    }
}

pub fn next_generation(cells: &mut Vec<Vec<[char; 2]>>, gen: usize, (xmax, ymax): (usize, usize)) {
    let (current, previous) = (gen % 2, (gen + 1) % 2);
    for y in 0..ymax {
        for x in 0..xmax {
            let alive_neighbors = alive_neighbors(cells, gen, (y, x), (xmax, ymax));
            cells[y][x][current] = if cells[y][x][previous] == ALIVE {
                // for alive
                match alive_neighbors {
                    0 | 1 => DEAD,  // dead
                    2 | 3 => ALIVE, // alive
                    4..=8 => DEAD,  // overpopulation
                    _ => panic!("phantom neighbors"),
                }
            } else {
                // for dead
                match alive_neighbors {
                    3 => ALIVE,
                    x if x <= 8 => DEAD,
                    _ => panic!("phantom neighbors"),
                }
            }
        }
    }
}

fn alive_neighbors(
    cells: &Vec<Vec<[char; 2]>>,
    gen: usize,
    coordinate: (usize, usize),
    (xmax, ymax): (usize, usize),
) -> u8 {
    let adjacent: [(i16, i16); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 1),
        (-1, -1),
    ];
    let (_current, previous) = (gen % 2, (gen + 1) % 2);

    let (y, x) = coordinate;
    let alive_neighbors = adjacent
        .map(|(y0, x0)| (y0 + y as i16, x0 + x as i16))
        .into_iter()
        .filter(|(y, x)| (*y as usize) < ymax && (*x as usize) < xmax)
        .filter(|(y, x)| cells[*y as usize][*x as usize][previous] == ALIVE) // TODO: take care in this section
        .count();

    alive_neighbors as u8
}

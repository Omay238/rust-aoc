use aoc_lib;

fn main() {
    let mut answers = (0, 0);

    let input = String::from(aoc_lib::get_input().trim());

    let mut grid = input.split("\n").map(|x| x.chars().map(|y| y == '#').collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    for _ in 0..100 {
        let mut holder_grid = Vec::new();
        for y in 0..grid.len() {
            holder_grid.push(Vec::new());
            for x in 0..grid[y].len() {
                let mut neighbors = 0;
                if let Some(ym1) = grid.get(y - 1) {
                    if let Some(xm1) = ym1.get(x - 1) {
                        if *xm1 {neighbors += 1}
                    }
                    if let Some(xp1) = ym1.get(x + 1) {
                        if *xp1 {neighbors += 1}
                    }
                    if ym1[x] {neighbors += 1}
                }
                if let Some(xm1) = grid[y].get(x - 1) {
                    if *xm1 {neighbors += 1}
                }
                if let Some(xp1) = grid[y].get(x + 1) {
                    if *xp1 {neighbors += 1}
                }
                if let Some(yp1) = grid.get(y + 1) {
                    if let Some(xm1) = yp1.get(x - 1) {
                        if *xm1 {neighbors += 1}
                    }
                    if let Some(xp1) = yp1.get(x + 1) {
                        if *xp1 {neighbors += 1}
                    }
                    if yp1[x] {neighbors += 1}
                }
                if neighbors == 3 {
                    holder_grid[y].push(true);
                } else if neighbors == 2 {
                    holder_grid[y].push(grid[y][x]);
                } else {
                    holder_grid[y].push(false);
                }
            }
        }
        grid = holder_grid;
    }

    answers.0 = grid.iter().map(|x| x.iter().map(|y| if *y {1} else {0}).sum::<i32>()).sum();

    grid = input.split("\n").map(|x| x.chars().map(|y| y == '#').collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    let w = grid[0].len() - 1;
    let h = grid.len() - 1;
    grid[0][0] = true;
    grid[0][w] = true;
    grid[h][0] = true;
    grid[h][w] = true;

    for _ in 0..100 {
        let mut holder_grid = Vec::new();
        for y in 0..grid.len() {
            holder_grid.push(Vec::new());
            for x in 0..grid[y].len() {
                let mut neighbors = 0;
                if let Some(ym1) = grid.get(y - 1) {
                    if let Some(xm1) = ym1.get(x - 1) {
                        if *xm1 {neighbors += 1}
                    }
                    if let Some(xp1) = ym1.get(x + 1) {
                        if *xp1 {neighbors += 1}
                    }
                    if ym1[x] {neighbors += 1}
                }
                if let Some(xm1) = grid[y].get(x - 1) {
                    if *xm1 {neighbors += 1}
                }
                if let Some(xp1) = grid[y].get(x + 1) {
                    if *xp1 {neighbors += 1}
                }
                if let Some(yp1) = grid.get(y + 1) {
                    if let Some(xm1) = yp1.get(x - 1) {
                        if *xm1 {neighbors += 1}
                    }
                    if let Some(xp1) = yp1.get(x + 1) {
                        if *xp1 {neighbors += 1}
                    }
                    if yp1[x] {neighbors += 1}
                }
                if neighbors == 3 || (x == 0 && y == 0) || (x == 0 && y == grid.len() - 1) || (x == grid[y].len() - 1 && y == grid.len() - 1) || (x == grid[y].len() - 1 && y == 0) {
                    holder_grid[y].push(true);
                } else if neighbors == 2 {
                    holder_grid[y].push(grid[y][x]);
                } else {
                    holder_grid[y].push(false);
                }
            }
        }
        grid = holder_grid;
    }

    answers.1 = grid.iter().map(|x| x.iter().map(|y| if *y {1} else {0}).sum::<i32>()).sum();

    aoc_lib::print_day(1);
    println!("{}", answers.0);
    aoc_lib::print_day(2);
    println!("{}", answers.1);
}

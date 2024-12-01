use image::GrayImage;
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;
use std::str::FromStr;
const SCREEN_SIZE: [usize; 2] = [500, 500];
const GRID_SIZE: [usize; 2] = [10, 10];
const PADDING_SIZE: usize = 4;
const CELL_SIZE: [usize; 2] = cell_size();
const ITERATIONS: usize = 10;

type Screen = [[u8; SCREEN_SIZE[0]]; SCREEN_SIZE[1]];
type Grid = [[bool; GRID_SIZE[0]]; GRID_SIZE[1]];

const fn cell_size() -> [usize; 2] {
    [SCREEN_SIZE[0] / GRID_SIZE[0], SCREEN_SIZE[1] / GRID_SIZE[1]]
}

const DIRECTIONS: [[isize; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];
fn main() -> std::io::Result<()> {
    // save_grid();
    println!("Welcome to conway's game of life");
    // parse args
    let args = parse_args();
    let save_dir = args.save_dir;
    // if the dir exists, clear the contents
    if save_dir.exists() {
        // delete everything in the dir
        assert!(save_dir.is_dir(), "path is not a directory path");
        for entry in fs::read_dir(&save_dir)? {
            let entry = entry?;
            fs::remove_file(entry.path())?;
        }
    // if not then make the dir
    } else {
        fs::create_dir_all(&save_dir)?;
    }

    let mut grid = State::new();
    // make a starting state for the grid
    [[3, 3], [3, 4], [3, 5]]
        .iter()
        .for_each(|coord| grid.grid_array[coord[0]][coord[1]] = true);
    grid.update_screen();
    // loop through and save successive iterator
    // do the first step
    let image = screen_to_image(grid.screen_array);
    let file_name = format!("frame_{:03}.png", &0);
    let save_path = &save_dir.join(file_name);
    image.save(save_path).expect("error saving image");
    println!("starting loop");
    // loop
    for i in 1..ITERATIONS {
        // update the grid
        grid.update_grid();
        grid.update_screen();
        let image = screen_to_image(grid.screen_array);
        let file_name = format!("frame_{:03}.png", i);
        let save_path = &save_dir.join(file_name);
        image.save(save_path).expect("error saving image");
    }

    Ok(())
}

struct Args {
    save_dir: path::PathBuf,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2, "more than 1 arg provided");

    let save_dir = PathBuf::from_str(&args[1]).unwrap();
    Args { save_dir }
}

#[derive(Debug)]
struct State {
    screen_array: Screen,
    grid_array: Grid,
}

impl State {
    fn new() -> Self {
        let screen_array = [[0; SCREEN_SIZE[0]]; SCREEN_SIZE[1]];
        let grid_array = [[false; GRID_SIZE[0]]; GRID_SIZE[1]];
        State {
            screen_array,
            grid_array,
        }
    }
    fn fill_screen_cell(&mut self, row: &usize, col: &usize, value: &u8) {
        // fill the grid cell specified with the value specified

        let start_i = (row * CELL_SIZE[0]) + PADDING_SIZE;
        let end_i = ((row + 1) * CELL_SIZE[0]) - PADDING_SIZE;
        let start_j = (col * CELL_SIZE[1]) + PADDING_SIZE;
        let end_j = ((col + 1) * CELL_SIZE[1]) - PADDING_SIZE;

        for screen_row in start_i..end_i {
            self.screen_array[screen_row][start_j..end_j].fill(*value);
        }
    }
    fn count_cell_neighbors(&self, row: &usize, col: &usize) -> usize {
        // loop through the directions
        DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let target_row = *row as isize + dir[0];
                let target_col = *col as isize + dir[1];

                if target_row < 0 || target_col < 0 {
                    None
                } else {
                    let target_row = target_row as usize;
                    let target_col = target_col as usize;

                    self.get_grid_cell(&target_row, &target_col)
                }
            })
            .filter(|cell| **cell)
            .count()
    }

    fn get_grid_cell(&self, row: &usize, col: &usize) -> Option<&bool> {
        if let Some(row_arr) = self.grid_array.get(*row) {
            return row_arr.get(*col);
        }
        None
    }
    fn next_cell_state(&mut self, row: &usize, col: &usize) -> bool {
        let state = self.grid_array[*row][*col];
        let num_neighbors = self.count_cell_neighbors(row, col);
        if state {
            // if the cell is alive.
            println!("alive at {},{} n: {}", row, col, num_neighbors);
            if num_neighbors < 2 || num_neighbors > 3 {
                return false;
            } else {
                return true;
            }
        } else {
            // if cell was dead
            if num_neighbors == 3 {
                return true;
            } else {
                return false;
            }
        }
    }
    fn update_grid(&mut self) {
        let mut new_grid = [[false; GRID_SIZE[0]]; GRID_SIZE[1]];
        for row in 0..GRID_SIZE[0] {
            for col in 0..GRID_SIZE[1] {
                new_grid[row][col] = self.next_cell_state(&row, &col);
            }
        }
        self.grid_array = new_grid;
    }

    // fn count_neighbors(&self) -> [[usize; GRID_SIZE[0]]; GRID_SIZE[1]] {
    //     for row in 0..GRID_SIZE[0] {
    //         for col in 0..GRID_SIZE[1] {
    //             let row = row as isize;
    //             let col = col as isize;
    //             // loop through the directions and get the results
    //             DIRECTIONS.iter().filter_map(|dir| {
    //                 match self.grid_array.get(dir[0]) {

    //                 }
    //             })
    //         }
    //     }
    // }

    fn update_screen(&mut self) {
        for row in 0..GRID_SIZE[0] {
            for col in 0..GRID_SIZE[1] {
                // get the grid value and fill the cell accordingly
                let cell_state = self.grid_array[row][col];
                if cell_state {
                    self.fill_screen_cell(&row, &col, &255);
                } else {
                    self.fill_screen_cell(&row, &col, &0);
                }
            }
        }
    }
}

fn screen_to_image(screen: Screen) -> GrayImage {
    GrayImage::from_raw(
        SCREEN_SIZE[1] as u32,
        SCREEN_SIZE[0] as u32,
        screen.into_iter().flatten().collect(),
    )
    .unwrap()
}

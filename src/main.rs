use rand::Rng;
use raylib::prelude::*;
use std::env::args;

mod cell;
use cell::*;

const WINDOW_SIZE: i32 = 1000;

fn parse_arguments(arguments: &[String]) -> i32 {
    let cells = arguments
        .get(1)
        .unwrap_or(&String::from("100"))
        .parse()
        .unwrap_or(100);
    cells
}

fn initialize_grid(window_size: i32, n_cells: i32) -> Vec<Cell> {
    let mut grid = Vec::with_capacity((n_cells * n_cells).try_into().unwrap());
    let offset = (window_size as f32 / n_cells as f32).round() as i32;
    let mut rng = rand::thread_rng();

    for y in 0..n_cells {
        let y_offset = y * offset;
        for x in 0..n_cells {
            let x_offset = x * offset;
            let state = if rng.gen_bool(0.5) {
                CellState::Alive
            } else {
                CellState::Dead
            };
            grid.push(Cell {
                state: state,
                x: x_offset,
                y: y_offset,
                size: offset,
            });
        }
    }
    grid
}

fn get_neighbors(cells: &[Cell], position: (i32, i32)) -> i32 {
    let mut live_neighbors = 0;
    let width = (cells.len() as f64).sqrt() as i32;
    let offset: usize = (position.0 + position.1 * width).try_into().unwrap();
    let cell = &cells[offset];

    let left_index = offset - 1;
    if cell.x > 0 && cells[left_index as usize].state == CellState::Alive {
        live_neighbors += 1;
    }

    let side_check = cell.x < WINDOW_SIZE - cell.size;
    let right_index = offset + 1;
    if side_check && cells[right_index].state == CellState::Alive {
        live_neighbors += 1;
    }

    let upper_index = offset - (width as usize);
    if cell.y > 0 && cells[upper_index].state == CellState::Alive {
        live_neighbors += 1;
    }

    let bottom_index = offset + (width as usize);
    if cells[bottom_index].state == CellState::Alive {
        live_neighbors += 1;
    }
    /*
        let upper_left = offset - (width as usize) - 1;
        if cells[upper_left].state == CellState::Alive {
            live_neighbors += 1;
        }

        let upper_right = offset - (width as usize) + 1;
        if cells[upper_right].state == CellState::Alive {
            live_neighbors += 1;
        }

        let bottom_left = offset + (width as usize) - 1;
        if cells[bottom_left].state == CellState::Alive {
            live_neighbors += 1;
        }

        let bottom_right = offset + (width as usize) + 1;
        if cells[bottom_right].state == CellState::Alive {
            live_neighbors += 1;
        }
    */

    live_neighbors
}

fn update_grid(cells: &[Cell]) -> () {
    for cell in cells {}
}

fn draw_grid(cells: &[Cell], dh: &mut RaylibDrawHandle) -> () {
    for cell in cells {
        if cell.state == CellState::Alive {
            dh.draw_rectangle(cell.x, cell.y, cell.size - 1, cell.size - 1, Color::GREEN);
        }
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    println!("Usage: {} <width> <cells>\n\n", args[0]);
    let cells = parse_arguments(&args[0..]);

    let cell_grid = initialize_grid(WINDOW_SIZE, cells);
    //println!("number of neighbors {}", get_neighbors(&cell_grid, (1,1)));

    set_trace_log(TraceLogLevel::LOG_ERROR);
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .title("game of life")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        draw_grid(&cell_grid, &mut d);
    }
}

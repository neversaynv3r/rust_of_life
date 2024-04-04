// Rust of Life
// Copyright (c) 2024 Marcos Pardo

use crate::cell::*;
use raylib::prelude::*;
use std::env::args;

mod cell;
mod universe;
use universe::*;

const WINDOW_SIZE: i32 = 1000;

/// Parses command-line arguments to extract the number of cells.
///
/// ### Arguments
///
/// * `arguments` - A slice of strings representing command-line arguments.
///
/// ### Returns
///
/// The number of cells parsed from the command-line arguments, or 100 if parsing fails.
fn parse_arguments(arguments: &[String]) -> i32 {
    let cells = arguments
        .get(1)
        .unwrap_or(&String::from("100"))
        .parse()
        .unwrap_or(100);
    cells
}

/// Draws the grid of cells on the screen using the given drawing handle.
///
/// ### Arguments
///
/// * `cells` - A slice of cells representing the grid to be drawn.
/// * `dh` - A mutable reference to the RaylibDrawHandle used for drawing.
fn draw_grid(cells: &[Cell], dh: &mut RaylibDrawHandle) -> () {
    for cell in cells {
        if cell.state == CellState::Alive {
            dh.draw_rectangle(cell.x, cell.y, cell.size - 1, cell.size - 1, Color::GREEN);
        }
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    println!("Usage: {} <num_cells>\n\n", args[0]);
    let cell_side_length = parse_arguments(&args[0..]) as u32;

    let mut universe = Universe::new(WINDOW_SIZE, cell_side_length);

    //deactivate Raylib initial log message
    set_trace_log(TraceLogLevel::LOG_ERROR);
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_SIZE, WINDOW_SIZE)
        .title("≛ rust of life ≛")
        .build();
    rl.set_target_fps(25);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        draw_grid(&universe.cells, &mut d);
        universe.update();
    }
}

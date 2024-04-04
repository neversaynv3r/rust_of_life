/*
The universe implementation is based on the rustwasm book by Aaron Turon.
https://rustwasm.github.io/docs/book/game-of-life/implementing.html

Copyright (c) 2018 Aaron Turon

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
*/

// Rust of Life
// Copyright (c) 2024 Marcos Pardo

use rand::Rng;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
    pub x: i32,
    pub y: i32,
    pub size: i32,
}

pub struct Universe {
    width: u32,
    height: u32,
    pub cells: Vec<Cell>,
}

impl Universe {
    /// Creates a new `Universe` with the specified window size and cell side length.
    ///
    /// ### Arguments
    ///
    /// * `window_size` - The size of the window.
    /// * `cell_side_length` - The length of each side of the cells.
    ///
    /// ### Returns
    ///
    /// A new `Universe` instance.
    pub fn new(window_size: i32, cell_side_length: u32) -> Universe {
        let offset = (window_size as f32 / cell_side_length as f32).round() as i32;
        let mut rng = rand::thread_rng();

        let cells = (0..cell_side_length * cell_side_length)
            .map(|i| {
                let x = (i % cell_side_length) as i32 * offset;
                let y = (i / cell_side_length) as i32 * offset;
                let state = if rng.gen_bool(0.15) {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
                Cell {
                    state,
                    x,
                    y,
                    size: offset,
                }
            })
            .collect();

        Universe {
            width: cell_side_length,
            height: cell_side_length,
            cells,
        }
    }

    /// Calculates the index of a cell in the universe's cells vector based on its coordinates.
    ///
    /// ### Arguments
    ///
    /// * `x` - The x-coordinate of the cell.
    /// * `y` - The y-coordinate of the cell.
    ///
    /// ### Returns
    ///
    /// The index of the cell in the cells vector.
    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }

    /// Counts the number of live neighbors around a given cell.
    ///
    /// ### Arguments
    ///
    /// * `y` - The y-coordinate of the cell.
    /// * `x` - The x-coordinate of the cell.
    ///
    /// ### Returns
    ///
    /// The number of live neighbors around the cell.
    fn live_neighbor_count(&self, y: u32, x: u32) -> u8 {
        let mut count = 0;
        for delta_y in [self.height - 1, 0, 1].iter().cloned() {
            for delta_x in [self.width - 1, 0, 1].iter().cloned() {
                if delta_y == 0 && delta_x == 0 {
                    continue;
                }

                let neighbor_y = (y + delta_y) % self.height;
                let neighbor_x = (x + delta_x) % self.width;
                let idx = self.get_index(neighbor_y, neighbor_x);
                count += self.cells[idx].state as u8;
            }
        }
        count
    }

    /// Progresses the universe to the next generation based on the rules of the Game of Life.
    pub fn update(&mut self) {
        let mut next = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell = match (cell.state, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (CellState::Alive, count) if count < 2 => CellState::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (CellState::Alive, count) if count > 3 => CellState::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (CellState::Dead, 3) => CellState::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx].state = next_cell;
            }
        }

        self.cells = next;
    }
}

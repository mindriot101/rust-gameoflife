extern crate rand;

use std::io::{BufWriter, Write};
use std::fs::File;

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

impl rand::Rand for CellState {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let is_alive: bool = rng.gen();
        if is_alive {
            CellState::Alive
        } else {
            CellState::Dead
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Board {
    cells: Vec<CellState>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
        let i = self.index(x, y);
        self.cells[i] = state;
    }

    pub fn next_steps(&self) {}

    pub fn get(&self, x: usize, y: usize) -> CellState {
        let i = self.index(x, y);
        self.cells[i]
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Neighbours {
        Neighbours {
            board: &self,
            ix: -1,
            iy: -1,
            x: x,
            y: y,
            counter: 0,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

pub struct Neighbours<'a> {
    board: &'a Board,
    ix: i8,
    iy: i8,
    x: usize,
    y: usize,
    counter: usize,
}

impl<'a> std::iter::Iterator for Neighbours<'a> {
    type Item = CellState;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 8 {
            return None;
        }

        if self.ix == 0 && self.iy == 0 {
            self.ix += 1;
        }

        let mut newx = self.x as i32 + self.ix as i32;
        let mut newy = self.y as i32 + self.iy as i32;

        if newx < 0 {
            newx = (self.board.width - 1) as _;
        }

        if newx == self.board.width as _ {
            newx = 0 as _;
        }

        if newy < 0 {
            newy = (self.board.height - 1) as _;
        }

        if newy == self.board.height as _ {
            newy = 0 as _;
        }

        if self.ix == 1 {
            self.ix = -1;
            self.iy += 1;
        } else {
            self.ix += 1;
        }

        self.counter += 1;

        Some(self.board.get(newx as _, newy as _))
    }
}

fn update(board: Board) -> Board {

    let mut new_board = board.clone();

    for h in 0..board.height {
        for w in 0..board.width {
            let cell_value = new_board.get(w, h);
            let neighbours: Vec<_> = board.neighbours(w, h).collect();
            assert_eq!(neighbours.len(), 8);
            let alive_count = neighbours.iter().filter(|c| **c == CellState::Alive)
                .count();

            use CellState::*;
            let new_value = if cell_value == Alive {
                if alive_count < 2 {
                    Dead
                } else if alive_count > 3 {
                    Dead
                } else {
                    Alive
                }
            } else {
                if alive_count == 3 { Alive } else { Dead }
            };

            new_board.set(w, h, new_value);
        }
    }

    new_board
}

fn main() {
    let mut board = read_initial_board();
    let iterations = 100;
    let file = File::create("gol.txt").unwrap();
    let mut writer = BufWriter::new(file);

    // Write header
    write!(&mut writer, "{} {}\n---\n", board.width, board.height);


    for _ in 0..iterations {

        render(&board, &mut writer);

        board = update(board);
    }
}

fn read_initial_board() -> Board {
    let width = 20;
    let height = 20;
    let mut board = Board {
        cells: vec![CellState::Dead; width * height],
        width: width,
        height: height,
    };

    /* Update the board here */
    for row in 0..board.height {
        for col in 0..board.width {
            board.set(col, row, rand::thread_rng().gen());
        }
    }

    board
}

/*
fn render<T: Write>(board: &Board, writer: &mut T) {
    for row in 0..board.height {
        for col in 0..board.width {
            /* Invert the x and y axis for a more natural rendering */
            let value = board.get(col, row);
            let c = if value == CellState::Alive { 'A' } else { '.' };
            write!(writer, "{}", c).unwrap();
        }
        write!(writer, "\n").unwrap();
    }
}
*/
fn render<T: Write>(board: &Board, writer: &mut T) {
    for row in 0..board.height {
        for col in 0..board.width {
            let value = board.get(col, row);
            if value == CellState::Alive {
                write!(writer, "{} {}\n", col, row).unwrap();
            }
        }
    }
    write!(writer, "---\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        use CellState::*;

        /* Large grid so boundary conditions do not play a part */
        let mut board = Board {
            width: 4,
            height: 4,
            cells: vec![Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Alive, Alive, Dead, Dead, Dead, Dead, Dead, Dead, Alive,
                        Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Dead],
        };
        for _ in 0..100 {
            board = update(board);
        }

        assert_eq!(board.cells,
                   vec![Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Alive, Alive, Dead, Dead, Dead, Dead, Dead, Dead,
                        Alive, Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
                        Dead, Dead, Dead, Dead, Dead]);
    }
}

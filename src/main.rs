extern crate rand;
extern crate clap;

use std::io::{BufWriter, Write};
use std::fs::File;

use rand::Rng;
use clap::{Arg, App};

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
            board: self,
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
            let alive_count = neighbours
                .iter()
                .filter(|c| **c == CellState::Alive)
                .count();

            use CellState::*;
            let new_value = if cell_value == Alive {
                if alive_count < 2 || alive_count > 3 {
                    Dead
                } else {
                    Alive
                }
            } else if alive_count == 3 {
                Alive
            } else {
                Dead
            };

            new_board.set(w, h, new_value);
        }
    }

    drop(board);

    new_board
}

fn read_board(_filename: &str) -> Board {
    read_initial_board(20, 20)
}

fn main() {
    let matches = App::new("GOL")
        .author("Simon Walker")
        .version("1.0")
        .arg(Arg::with_name("niterations")
                 .short("N")
                 .long("niter")
                 .value_name("NITERATIONS")
                 .takes_value(true)
                 .help("Number of iterations to run"))
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .value_name("OUTPUT")
                 .takes_value(true)
                 .required(true)
                 .help("Output text file"))
        .arg(Arg::with_name("input")
                 .short("-i")
                 .long("input")
                 .value_name("INPUT")
                 .takes_value(true)
                 .help("Input board state"))
        .arg(Arg::with_name("width")
                 .short("w")
                 .long("width")
                 .value_name("WIDTH")
                 .takes_value(true)
                 .help("Width of board")
                 .conflicts_with("input"))
        .arg(Arg::with_name("height")
                 .short("H")
                 .long("height")
                 .value_name("HEIGHT")
                 .takes_value(true)
                 .help("Height of board")
                 .conflicts_with("input"))
        .get_matches();

    let niterations = matches
        .value_of("niterations")
        .unwrap_or("100")
        .parse()
        .unwrap();
    let output = matches.value_of("output").unwrap();

    let mut board = match matches.value_of("input") {
        Some(filename) => read_board(filename),
        None => {
            let width = matches
                .value_of("width")
                .expect("board width argument required")
                .parse()
                .unwrap();
            let height = matches
                .value_of("height")
                .expect("board height argument required")
                .parse()
                .unwrap();
            read_initial_board(width, height)
        }
    };

    let file = File::create(output).unwrap();
    let mut writer = BufWriter::new(file);

    // Write header
    write!(&mut writer, "{} {}\n{}\n---\n", board.width, board.height, niterations).unwrap();


    for _ in 0..niterations {

        render(&board, &mut writer);

        board = update(board);
    }
}

fn read_initial_board(width: usize, height: usize) -> Board {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Default, Debug)]
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

    pub fn get(&self, x: usize, y: usize) -> CellState {
        let i = self.index(x, y);
        self.cells[i]
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn update(board: Board) -> Board {
    Board {
        cells: board.cells.clone(),
        width: board.width,
        height: board.height,
    }
}


fn main() {
    let mut board = read_initial_board();
    let iterations = 100;
    for _ in 0..iterations {
        board = update(board);
        render(&board);
    }
}

fn read_initial_board() -> Board {
    let width = 100;
    let height = 100;
    let board = Board {
        cells: vec![CellState::Dead; width * height],
        width: width,
        height: height,
    };

    /* Update the board here */
    board
}

fn render(_board: &Board) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        use CellState::*;
        let mut board = Board {
            width: 4,
            height: 4,
            cells: vec![
                Dead, Dead, Dead, Dead,
                Dead, Alive, Alive, Dead,
                Dead, Alive, Alive, Dead,
                Dead, Dead, Dead, Dead,
            ],
        };
        for _ in 0..100 {
            board = update(board);
        }

        assert_eq!(board.cells, vec![
                Dead, Dead, Dead, Dead,
                Dead, Alive, Alive, Dead,
                Dead, Alive, Alive, Dead,
                Dead, Dead, Dead, Dead,
            ]);
    }
}

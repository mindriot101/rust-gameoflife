#[derive(Debug, Clone, Copy)]
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
        cells: Vec::new(),
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
    Board::default()
}

fn render(_board: &Board) {
}

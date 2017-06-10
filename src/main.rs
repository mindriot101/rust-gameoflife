enum CellState {
    Alive,
    Dead,
}

struct Board {
    cells: Vec<CellState>,
}

fn update(board: Board) -> Board {
    Board {
        cells: Vec::new(),
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
    Board {
        cells: Vec::new()
    }
}

fn render(board: &Board) {
}

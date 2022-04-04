pub fn square(s: u32) -> u64 {
    let board_cell = calculated_grains_per_cell(s);
    board_cell
}

fn calculated_grains_per_cell(cell_value: u32) -> u64 {
    match cell_value {
        1..= 64 => {
            let board = build_calculated_grains_board(64);
            let grains_per_cell = board[(cell_value - 1) as usize];
            grains_per_cell
        }
        _ => {
            panic!("Square must be between 1 and 64")
        }
    }
}

fn build_board(_cells: u32) -> Vec<u32> {
    let mut board = Vec::new();
    for number in 1.._cells+1 {
        board.push(number)
    }
    board
}

fn build_calculated_grains_board(cells: u32) -> Vec<u64> {
    let chess_board = build_board(cells);
    let mut chess_board_with_grains: Vec<u64> = Vec::new();
    for (index, _) in chess_board.iter().enumerate() {
        match index {
            0 => chess_board_with_grains.push(1),
            _ => {
                let previous_value = chess_board_with_grains[index-1];
                chess_board_with_grains.push(previous_value as u64 * 2);
            }
        }
    }
    chess_board_with_grains
}

pub fn total() -> u64 {
    let total_sum_of_grains_in_board = build_calculated_grains_board(64).iter().map(|x| x).sum();
    total_sum_of_grains_in_board
}
use std::time::Instant;

const NUM_ROWS: usize = 18;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    let success = place_queens_4(&mut board, 0);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

fn dump_board(board: &[[char; NUM_COLS]; NUM_ROWS]) {
    print!("   c ");
    for c in 0..NUM_COLS {
        print!("[{:<02}]", c);
    }
    println!();

    for r in 0..NUM_ROWS {
        print!("r[{}]| ", r);
        for c in 0..NUM_COLS {
            print!("{:<02}  ", board[r][c]);
        }
        println!();
    }
    println!();
}

fn board_is_a_solution(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    // See if it is legal.
    if !board_is_legal(board) {
        return false;
    }

    // See if the board contains exactly NUM_ROWS queens.
    let mut num_queens = 0;
    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            if board[r as usize][c as usize] == 'Q' {
                num_queens += 1;
            }
        }
    }
    return num_queens == NUM_ROWS;
}

fn board_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    // See if each row is legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, 0, 0, 1) {
            return false;
        }
    }

    // See if diagonals down to the right are legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, 0, 1, 1) {
            return false;
        }
    }
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, 1) {
            return false;
        }
    }

    // See if diagonals down to the left are legal.
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, INUM_ROWS - 1, 1, -1) {
            return false;
        }
    }
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, -1) {
            return false;
        }
    }

    // If we survived this long, then the board is legal.
    return true;
}

fn series_is_legal(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut has_queen = false;

    let mut r = r0;
    let mut c = c0;
    loop {
        if board[r as usize][c as usize] == 'Q' {
            // If we already have a queen on this row,
            // then this board is not legal.
            if has_queen {
                return false;
            }

            // Remember that we have a queen on this row.
            has_queen = true;
        }

        // Move to the next square in the series.
        r += dr;
        c += dc;

        // If we fall off the board, then the series is legal.
        if r >= INUM_ROWS || c >= INUM_COLS || r < 0 || c < 0 {
            return true;
        }
    }
}

fn place_queens_4(board: &mut [[char; NUM_COLS]; NUM_ROWS], c: i32) -> bool {
    // if we have assigned a queen to every column, check if board is legal
    if c == INUM_ROWS {
        return board_is_legal(board);
    }

    // if the column being checked is less than the number of rows, see if the board is legal and exit early if it isn't
    if c < INUM_ROWS {
        if !board_is_legal(board) {
            return false;
        }
    }

    // try assigning a queen to this column by looping through all rows
    for r in 0..INUM_ROWS {
        board[r as usize][c as usize] = 'Q';

        let result = place_queens_4(board, c + 1);

        if result {
            return true;
        } else {
            // reset board
            board[r as usize][c as usize] = '.';
        }
    }

    return false;
}

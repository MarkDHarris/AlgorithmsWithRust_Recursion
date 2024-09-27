use std::time::Instant;

const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

const REQUIRE_CLOSED_TOUR: bool = false;

const UNVISITED: i32 = -1;

fn main() {
    // legal moves
    let mut offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    board[0][0] = 0;

    let start = Instant::now();
    let success = find_tour(&mut board, &mut offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}

fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &mut [[i32; 2]; 8],
    curr_row: i32,
    curr_col: i32,
    num_visited: i32,
) -> bool {
    if num_visited == (NUM_ROWS * NUM_COLS) as i32 {
        if !REQUIRE_CLOSED_TOUR {
            return true;
        } else {
            // then loop through the knight’s 8 possible next moves.
            // If any of them puts the knight back on the starting square, return true to indicate that we have a valid solution.
            // If none of the possible next moves returns to the starting square, return false to indicate that the current board position doesn’t work

            for i in 0..8 {
                // get the move
                let r = curr_row + offsets[i][0];
                let c = curr_col + offsets[i][1];

                // check if the move is on the board
                if r < 0 || r >= INUM_ROWS {
                    continue;
                }
                if c < 0 || c >= INUM_COLS {
                    continue;
                }

                // check if we already visited this position
                if board[r as usize][c as usize] != UNVISITED {
                    continue;
                }

                // check if this moves us back to the first move
                if board[r as usize][c as usize] == 0 {
                    return true;
                }
            }

            // position doesn't work
            return false;
        }
    }

    for i in 0..8 {
        // get the move
        let r = curr_row + offsets[i][0];
        let c = curr_col + offsets[i][1];

        // check if the move is on the board
        if r < 0 || r >= INUM_ROWS {
            continue;
        }
        if c < 0 || c >= INUM_COLS {
            continue;
        }

        // check if we already visited this position
        if board[r as usize][c as usize] != UNVISITED {
            continue;
        }

        // move is viable, so make the move
        board[r as usize][c as usize] = num_visited;

        if find_tour(board, offsets, r, c, num_visited + 1) {
            return true;
        }

        // if we did not find a tour with this move, unmake the move
        board[r as usize][c as usize] = UNVISITED;
    }

    // none of the possible moves worked
    return false;
}

fn dump_board(board: &mut [[i32; NUM_COLS]; NUM_ROWS]) {
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

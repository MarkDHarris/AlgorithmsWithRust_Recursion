use std::time::Instant;

const NUM_ROWS: usize = 5;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn main() {
    do_1(); // < 5
            // do_2();  // < 7
            // do_3();  // < 10
}

fn do_1() {
    let mut board1 = [['.'; NUM_COLS]; NUM_ROWS];

    let start1 = Instant::now();
    let success1 = place_queens_1(&mut board1, 0, 0);
    let duration1 = start1.elapsed();

    println!("Time 1: {:?}", duration1);
    if success1 {
        println!("Success for #1!");
    } else {
        println!("Could not find a tour for #1.");
    }
    println!();
    dump_board(&mut board1);
    println!();
}

#[allow(dead_code)]
fn do_2() {
    let mut board2 = [['.'; NUM_COLS]; NUM_ROWS];

    let start2 = Instant::now();
    let success2 = place_queens_2(&mut board2, 0, 0, 0);
    let duration2 = start2.elapsed();

    println!("Time 2: {:?}", duration2);
    if success2 {
        println!("Success for #2!");
    } else {
        println!("Could not find a tour for #2.");
    }
    println!();
    dump_board(&mut board2);
    println!();
}

#[allow(dead_code)]
fn do_3() {
    let mut board3 = [['.'; NUM_COLS]; NUM_ROWS];

    let start3 = Instant::now();
    let success3 = place_queens_2(&mut board3, 0, 0, 0);
    let duration3 = start3.elapsed();

    println!("Time 3: {:?}", duration3);
    if success3 {
        println!("Success for #3!");
    } else {
        println!("Could not find a tour for #3.");
    }
    println!();
    dump_board(&mut board3);
    println!();
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
            // if we already have a queen on this row then its not legal
            if has_queen {
                return false;
            }

            has_queen = true;
        }

        // move to next square
        r += dr;
        c += dc;

        if r >= INUM_ROWS || c >= INUM_COLS || r < 0 || c < 0 {
            return true;
        }
    }
}

fn board_is_legal(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    // check each row
    for r in 0..INUM_ROWS {
        if !series_is_legal(board, r, 0, 0, 1) {
            return false;
        }
    }

    // check each column
    for c in 0..INUM_COLS {
        if !series_is_legal(board, 0, c, 1, 0) {
            return false;
        }
    }

    // check diagonals down to the right
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

    // check diagonals down to the left
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

    // if this point is reached its legal
    return true;
}

fn board_is_a_solution(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    if !board_is_legal(board) {
        return false;
    }

    let mut num_queens = 0;

    for r in 0..NUM_ROWS {
        for c in 0..NUM_COLS {
            if board[r as usize][c as usize] == 'Q' {
                num_queens += 1;
            }
        }
    }

    if num_queens == NUM_ROWS {
        return true;
    } else {
        return false;
    }
}

fn place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], r: i32, c: i32) -> bool {
    // check to see if we've exceeded the number of rows
    if r >= INUM_ROWS {
        return board_is_a_solution(board);
    }

    // if we have not exceeded the board rows
    // find the next r and c.. go to next c but if it exceeds then wrap to next row
    let mut next_r = r;
    let mut next_c = c + 1;
    if next_c >= INUM_ROWS {
        next_r += 1;
        next_c = 0;
    }

    // see what happens if we do not place a queen at [r][c]
    if place_queens_1(board, next_r, next_c) {
        // found a solution
        return true;
    }

    // see what happens if we do place a queen at [r][c]
    board[r as usize][c as usize] = 'Q';
    if place_queens_1(board, next_r, next_c) {
        // found a solution
        return true;
    }

    // didn't find a solution so backtrack and reset
    board[r as usize][c as usize] = '.';
    return false;
}

#[allow(dead_code)]
fn place_queens_2(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
    mut num_queens_placed: i32,
) -> bool {
    // if we placed all the queens, check if this is a solution and stop searching
    if num_queens_placed >= INUM_ROWS {
        return board_is_a_solution(board);
    }

    // check to see if we've exceeded the number of rows
    if r >= INUM_ROWS {
        return false;
    }

    // if we have not exceeded the board rows
    // find the next r and c.. go to next c but if it exceeds then wrap to next row
    let mut next_r = r;
    let mut next_c = c + 1;
    if next_c >= INUM_ROWS {
        next_r += 1;
        next_c = 0;
    }

    // see what happens if we do not place a queen at [r][c]
    if place_queens_2(board, next_r, next_c, num_queens_placed) {
        // found a solution
        return true;
    }

    // see what happens if we do place a queen at [r][c]
    board[r as usize][c as usize] = 'Q';

    // record placement of a queen
    num_queens_placed += 1;

    if place_queens_2(board, next_r, next_c, num_queens_placed) {
        // found a solution
        return true;
    }

    // didn't find a solution so backtrack and reset
    board[r as usize][c as usize] = '.';

    // record removal of a queen
    num_queens_placed -= 1;

    return false;
}

#[allow(dead_code)]
// Set up and call place_queens_3.
fn place_queens_3(board: &mut [[char; NUM_COLS]; NUM_ROWS]) -> bool {
    // Make the num_attacking array.
    // The value num_attacking[r as usize][c as usize] is the number
    // of queens that can attack square (r, c).
    let mut num_attacking = [[0; NUM_COLS]; NUM_ROWS];

    // Call do_place_queens_3.
    let num_placed = 0;
    return do_place_queens_3(board, num_placed, 0, 0, &mut num_attacking);
}

#[allow(dead_code)]
// Try placing a queen at position [r][c].
// Keep track of the number of queens placed.
// Keep running totals of the number of queens attacking a square.
// Return true if we find a legal board.
fn do_place_queens_3(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    mut num_placed: i32,
    r: i32,
    c: i32,
    num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
) -> bool {
    // See if we have placed all of the queens.
    if num_placed == INUM_ROWS {
        // See if this is a solution.
        return board_is_a_solution(board);
    }

    // See if we have examined the whole board.
    if r >= INUM_ROWS {
        // We have examined all of the squares but this is not a solution.
        return false;
    }

    // Find the next square.
    let mut next_r = r;
    let mut next_c = c + 1;
    if next_c >= INUM_ROWS {
        next_r += 1;
        next_c = 0;
    }

    // Leave no queen in this square and
    // recursively assign the next square.
    if do_place_queens_3(board, num_placed, next_r, next_c, num_attacking) {
        return true;
    }

    // See if we can place a queen at (r, c).
    if num_attacking[r as usize][c as usize] == 0 {
        // Try placing a queen here and
        // recursively assigning the next square.
        board[r as usize][c as usize] = 'Q';
        num_placed += 1;

        // Increment the attack counts for this queen.
        adjust_attack_counts(num_attacking, r, c, 1);

        if do_place_queens_3(board, num_placed, next_r, next_c, num_attacking) {
            return true;
        }

        // That didn't work so remove this queen.
        board[r as usize][c as usize] = '.';
        num_placed -= 1;
        adjust_attack_counts(num_attacking, r, c, -1);
    }

    // If we get here, then there is no solution from
    // the board position before this function call.
    // Return false to backtrack and try again farther up the call stack.
    return false;
}

#[allow(dead_code)]
// Add amount to the attack counts for this square.
fn adjust_attack_counts(
    num_attacking: &mut [[i32; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
    amount: i32,
) {
    // Attacks in the same row.
    for i in 0..INUM_COLS {
        num_attacking[r as usize][i as usize] += amount
    }

    // Attacks in the same column.
    for i in 0..INUM_ROWS {
        num_attacking[i as usize][c as usize] += amount
    }

    // Attacks in the upper left to lower right diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = r + i;
        let test_c = c + i;
        if test_r >= 0 && test_r < INUM_ROWS && test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }

    // Attacks in the upper right to lower left diagonal.
    for i in -INUM_ROWS..INUM_ROWS {
        let test_r = r + i;
        let test_c = c - i;
        if test_r >= 0 && test_r < INUM_ROWS && test_c >= 0 && test_c < INUM_ROWS {
            num_attacking[test_r as usize][test_c as usize] += amount;
        }
    }
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

use std::time::Instant;

const NUM_DISKS: usize = 3;

fn main() {
    let mut posts = [[0; NUM_DISKS]; 3];

    // put the disks on the first post in smallest first
    for i in 0..NUM_DISKS {
        posts[0][i] = i + 1;
    }

    draw_posts(&mut posts);

    let start_time = Instant::now();
    move_disks(&mut posts, NUM_DISKS, 0, 1, 2);
    let stop_time = start_time.elapsed();

    draw_posts(&mut posts);

    println!("ok!");
    println!("time elapsed: {:.2?}", stop_time);
}

fn draw_posts(posts: &mut [[usize; NUM_DISKS]; 3]) {
    for row in 0..NUM_DISKS {
        for post_num in 0..3 {
            print!("{} ", posts[post_num][row]);
        }
        println!();
    }
    println!("------");
}

fn move_disk(posts: &mut [[usize; NUM_DISKS]; 3], from_post: usize, to_post: usize) {
    // find the first non-empty row in from
    let mut from_row = 0usize;
    for row in 0..NUM_DISKS {
        if posts[from_post][row] != 0 {
            from_row = row;
            break;
        }
    }

    // find the last empty row in post
    let mut to_row = NUM_DISKS - 1;
    for row in 0..NUM_DISKS {
        if posts[to_post][row] != 0 {
            to_row = row - 1;
            break;
        }
    }

    // swap the values
    (posts[from_post][from_row], posts[to_post][to_row]) =
        (posts[to_post][to_row], posts[from_post][from_row]);
}

fn move_disks(
    posts: &mut [[usize; NUM_DISKS]; 3],
    num_to_move: usize,
    from_post: usize,
    to_post: usize,
    temp_post: usize,
) {
    // if we need to move more than 1 disk, recursively move disks from => temp, using to as temp
    if num_to_move > 1 {
        move_disks(posts, num_to_move - 1, from_post, temp_post, to_post);
    }

    // then move the top disk from => to
    move_disk(posts, from_post, to_post);

    // and display status
    draw_posts(posts);

    // if we needed to move more than 1 disk, recursively move disks temp => to using from as temp
    if num_to_move > 1 {
        move_disks(posts, num_to_move - 1, temp_post, to_post, from_post);
    }
}

use mines::board::Board;

fn main() {
    let mines = [
        (0,4), (0,7),
        (2,0), (2,1),
        (3,1), 
        (4,3),
        (5,3), (5,6),
        (7,0), (7,3),
    ];
    let mut board = Board::new_with_mines_at(8, 8, &mines);
    dbg!(&board);

    board.reveal(2, 7);
    println!("{board}");

    board.reveal(5, 0);
    println!("{board}");
}
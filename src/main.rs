use crate::field::Field;

mod field;
// mod tui;
// mod gtk;

fn main() {
    let mines = [
        (0,4), (0,7),
        (2,0), (2,1),
        (3,1), 
        (4,3),
        (5,3), (5,6),
        (7,0), (7,3),
    ];
    let mut field = Field::new_with_mines_at(8, 8, &mines);
    dbg!(&field);
    println!("xxxxxxxxxxxxxxxxxxx");

    field.reveal(2, 7);
    println!("{field}");
    println!("xxxxxxxxxxxxxxxxxxx");

    field.reveal(5, 0);
    println!("{field}");
    println!("xxxxxxxxxxxxxxxxxxx");

    field.auto_flag(3,2);
    println!("{field}");
    println!("xxxxxxxxxxxxxxxxxxx");

    field.auto_reveal(2,2);
    println!("{field}");
    println!("xxxxxxxxxxxxxxxxxxx");
}
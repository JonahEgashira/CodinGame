use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x0 = parse_input!(inputs[0], i32);
    let y0 = parse_input!(inputs[1], i32);

    let (mut current_x, mut current_y) = (x0, y0);
    let (mut x_min, mut x_max) = (-1, w);
    let (mut y_min, mut y_max) = (-1, h);
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        if bomb_dir.contains('U') {
            y_max = current_y;
            let middle_y = (y_min + y_max) / 2;
            current_y = middle_y;
        }
        if bomb_dir.contains('D') {
            y_min = current_y;
            let middle_y = (y_min + y_max) / 2;
            current_y = middle_y;
        }
        if bomb_dir.contains('L') {
            x_max = current_x;
            let middle_x = (x_min + x_max) / 2;
            current_x = middle_x;
        }
        if bomb_dir.contains('R') {
            x_min = current_x;
            let middle_x = (x_min + x_max) / 2;
            current_x = middle_x;
        }
        println!("{} {}", current_x, current_y);
    }
}

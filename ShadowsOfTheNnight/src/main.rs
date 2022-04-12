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
    let (mut xl, mut xr) = (0, w);
    let (mut yl, mut yr) = (0, h);
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        let first_char = bomb_dir.chars().next().unwrap();

        let middle_x = (xl + xr) / 2;
        let middle_y = (yl + yr) / 2;
        match first_char {
            'U' => yr = middle_y,
            'D' => yl = middle_y,
            'R' => xl = middle_x,
            'L' => xr = middle_x,
            _ => panic!(),
        };
    }
}

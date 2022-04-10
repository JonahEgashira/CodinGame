use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    let mut current_tx = initial_tx;
    let mut current_ty = initial_ty;
        // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // N  -> y + 1
        // NE -> y + 1, x + 1
        // E  -> x + 1
        // SE -> y - 1, x + 1
        // S  -> y - 1
        // SW -> y - 1, x - 1
        // W  -> x - 1
        // NW -> y + 1, x - 1

        let delta_x = current_tx - light_x;
        let delta_y = current_ty - light_y;

        let mut move_thor = |dx: i32, dy: i32| {
            current_tx += dx;
            current_ty += dy;
        };

        let mut direction = "";
        if delta_x < 0 {
            if delta_y > 0 {
                direction = "NE";
                move_thor(1, -1);
            } else if delta_y == 0 {
                direction = "E";
                move_thor(1, 0);
            } else if delta_y < 0 {
                direction = "SE";
                move_thor(1, 1);
            }
        } else if delta_x == 0 {
            if delta_y > 0 {
                direction = "N";
                move_thor(0, -1);
            } else if delta_y < 0 {
                direction = "S";
                move_thor(0, 1);
            }
        } else if delta_x > 0 {
            if delta_y > 0 {
                direction = "NW";
                move_thor(-1, -1);
            } else if delta_y == 0 {
                direction = "W";
                move_thor(-1, 0);
            } else if delta_y < 0 {
                direction = "SW";
                move_thor(-1, 1);
            }
        }
        eprintln!("Thor's position:  x:{}, y:{}", current_tx, current_ty);
        println!("{}", direction);
    }
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let num_of_0 = count_num_of_dial_0(file_path);

    println!("Number of zeros: {num_of_0}");

    let num_of_0_with_protocol = count_num_of_dial_0_protocol(file_path);

    println!("Number of zeros with protocol: {num_of_0_with_protocol}");
}

fn count_num_of_dial_0(file_path: &String) -> usize {
    match fs::read_to_string(file_path) {
        Ok(content) => count_from_string(content),
        Err(err) => {
            println!("Invalid file path {err}");
            0
        }
    }
}

fn count_num_of_dial_0_protocol(file_path: &String) -> usize {
    match fs::read_to_string(file_path) {
        Ok(content) => count_with_protocol(content),
        Err(err) => {
            println!("Invalid file path {err}");
            0
        }
    }
}

fn count_from_string(content: String) -> usize {
    let max_move = 100;
    let mut zero_counter = 0;
    let mut dial_pos = 50;

    for line in content.lines() {
        let is_left = line.starts_with('L');
        let move_dial = line[1..].parse::<u32>().unwrap() % max_move;

        // println!("dial pos: {dial_pos}, move: {move_dial}, left: {is_left}");

        dial_pos = if is_left {
            rotate_dial_left(dial_pos, move_dial)
        } else {
            rotate_dial_right(dial_pos, move_dial)
        };

        if dial_pos == 0 {
            zero_counter += 1;
        }
    }

    zero_counter
}

fn rotate_dial_right(pos: u32, num: u32) -> u32 {
    let max_val = 99;
    let mut res = pos + num;

    if res > max_val {
        res -= max_val + 1;
    }

    res
}

fn rotate_dial_left(pos: u32, num: u32) -> u32 {
    let max_val = 99;

    if pos < num {
        (max_val + 1) - (num - pos)
    } else {
        pos - num
    }
}

fn count_with_protocol(content: String) -> usize {
    let max_move = 100;
    let mut zero_counter = 0;
    let mut dial_pos = 50;

    for line in content.lines() {
        let is_left = line.starts_with('L');
        let mut move_dial = line[1..].parse::<u32>().unwrap();
        zero_counter += match move_dial.checked_div(max_move) {
            Some(num) => num as usize,
            None => 0,
        };

        move_dial %= max_move;

        // println!(
        //     "dial pos: {dial_pos}, move: {move_dial}, left: {is_left}, counter: {zero_counter}"
        // );

        let clicked_zero;

        (dial_pos, clicked_zero) = if is_left {
            protocol_rotate_dial_left(dial_pos, move_dial)
        } else {
            protocol_rotate_dial_right(dial_pos, move_dial)
        };

        if clicked_zero || (dial_pos == 0) {
            zero_counter += 1;
        }
    }

    zero_counter
}

fn protocol_rotate_dial_right(pos: u32, num: u32) -> (u32, bool) {
    let max_val = 99;
    let mut res = pos + num;
    let mut click_zero = false;

    if res > max_val {
        res -= max_val + 1;
        click_zero = true;
    }

    (res, click_zero)
}

fn protocol_rotate_dial_left(pos: u32, num: u32) -> (u32, bool) {
    let max_val = 99;

    if pos < num {
        ((max_val + 1) - (num - pos), pos != 0)
    } else {
        (pos - num, false)
    }
}

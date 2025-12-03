use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        let sum = sum_invalid_ids(file_path);

        println!("total sum: {sum}");
    } else {
        println!("Error: no input file was provided");
    }
}

fn sum_invalid_ids(file_path: &String) -> u64 {
    match fs::read_to_string(file_path) {
        Ok(content) => sum_from_content(content),
        Err(err) => {
            println!("Error while reading file: {err}");
            0
        }
    }
}

fn sum_from_content(mut content: String) -> u64 {
    let mut ids_sum = 0;
    content.pop(); // '\n' at the end of the file

    for range in content.split(',') {
        if let Some((from, to)) = range.split_once('-') {
            if let Ok(from_num) = from.parse::<u64>() {
                if let Ok(to_num) = to.parse::<u64>() {
                    ids_sum += sum_id_nums(from_num, to_num);
                } else {
                    println!("error parsing number: {to}")
                }
            } else {
                println!("error parsing number: {from}")
            }
        } else {
            println!("error parsing at range: {range}");
        }
    }

    ids_sum
}

fn sum_id_nums(from: u64, to: u64) -> u64 {
    let mut sum = 0;

    for id in from..=to {
        if is_mirror(id) {
            sum += id;
        }
    }

    sum
}

fn is_mirror(num: u64) -> bool {
    let mut str_num = num.to_string();
    let str_len = str_num.len();
    let mut res = false;

    if (str_len & 1) == 0 {
        let half_str = str_num.split_off(str_len / 2);
        if half_str == str_num {
            res = true;
        }
    }

    res
}

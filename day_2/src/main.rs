use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        let sum = sum_invalid_ids(file_path, false);

        println!("total sum: {sum}");

        let new_sum = sum_invalid_ids(file_path, true);

        println!("new rules total sum: {new_sum}");
    } else {
        println!("Error: no input file was provided");
    }
}

fn sum_invalid_ids(file_path: &String, new_rules: bool) -> u64 {
    match fs::read_to_string(file_path) {
        Ok(content) => sum_from_content(content, new_rules),
        Err(err) => {
            println!("Error while reading file: {err}");
            0
        }
    }
}

fn sum_from_content(mut content: String, new_rules: bool) -> u64 {
    let mut ids_sum = 0;
    content.pop(); // '\n' at the end of the file

    for range in content.split(',') {
        if let Some((from, to)) = range.split_once('-') {
            if let Ok(from_num) = from.parse::<u64>() {
                if let Ok(to_num) = to.parse::<u64>() {
                    ids_sum += sum_id_nums(from_num, to_num, new_rules);
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

fn sum_id_nums(from: u64, to: u64, new_rules: bool) -> u64 {
    let mut sum = 0;
    let rules = if new_rules {
        is_repeated_sequence
    } else {
        is_mirror
    };

    for id in from..=to {
        if rules(id) {
            sum += id;
            // println!("invalid id: {id}");
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

fn is_repeated_sequence(num: u64) -> bool {
    let str_num = num.to_string();
    let mut double_str_num = str_num.clone();
    double_str_num.push_str(&str_num);
    let second_digit = &double_str_num[1..];

    let match_index = second_digit.find(&str_num);

    match match_index {
        Some(index) => {
            // println!("match index: {index}");
            index < (str_num.len() - 1)
        }
        None => false,
    }
}

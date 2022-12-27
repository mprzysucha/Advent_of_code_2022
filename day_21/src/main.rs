extern crate core;

use std::collections::HashMap;
use aoctools::{parse_f64, read_whole_file};

fn main() {
    let content = read_whole_file("input.txt");
    let lines: Vec<&str> = content.split("\n").into_iter().filter(|s| !s.is_empty()).collect();
    let mut map: HashMap<String, String> = HashMap::new();


    for line in lines {
        let split = line.split(": ").into_iter().collect::<Vec<&str>>();
        let key = split[0].to_string();
        let val = split[1].to_string();
        map.insert(key, val);
    }

    let part_one_result = get_number("root".to_string(), &map);

    assert!(part_one_result.floor() == part_one_result.ceil());
    println!("Part one: {:?}", part_one_result as i64);

    let root = &map.get("root");
    let left = root.unwrap()[0..4].to_string();
    let right = root.unwrap()[7..11].to_string();

    let you_key: String = String::from("humn");
    let you_val = 3502.0; // original value

    let right_res = get_number_with_default_for_you(right.to_string(), &map, &you_key, you_val);
    let mut next_guess = you_val; // start with original
    let mut left_res = get_number_with_default_for_you(left.to_string(), &map, &you_key, next_guess);

    let mut diff = left_res - right_res;
    // let mut prev_diff = diff;
    let prev_guess = next_guess;
    let prev_left_res = left_res;

    let diff_guess = 1000.0;
    next_guess = next_guess + diff_guess;

    left_res = get_number_with_default_for_you(left.to_string(), &map, &you_key, next_guess);

    let res_diff = prev_left_res - left_res;
    let guess_diff = prev_guess - next_guess;

    let factor: f64 = res_diff / guess_diff;

    while diff.floor() != 0.0 {
        next_guess = ((next_guess + ((right_res - left_res) / factor)) as i128) as f64;
        left_res = get_number_with_default_for_you(left.to_string(), &map, &you_key, next_guess);
        // prev_diff = diff;
        diff = left_res - right_res;
    }

    assert!(next_guess.floor() == next_guess.ceil());
    println!("Part two: {}", next_guess as i64);

}

fn get_number_with_default_for_you(key: String, map: &HashMap<String, String>, you_key: &String, you_val: f64) -> f64 {
    if key == *you_key {
        you_val
    } else if let Some(v) = map.get(key.as_str()) {
        if v.contains(" + ") {
            let split: Vec<&str> = v.split(" + ").collect();
            get_number_with_default_for_you(split[0].to_string(), &map, &you_key, you_val) + get_number_with_default_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else if v.contains(" - ") {
            let split: Vec<&str> = v.split(" - ").collect();
            get_number_with_default_for_you(split[0].to_string(), &map, &you_key, you_val) - get_number_with_default_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else if v.contains(" * ") {
            let split: Vec<&str> = v.split(" * ").collect();
            get_number_with_default_for_you(split[0].to_string(), &map, &you_key, you_val) * get_number_with_default_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else if v.contains(" / ") {
            let split: Vec<&str> = v.split(" / ").collect();
            get_number_with_default_for_you(split[0].to_string(), &map, &you_key, you_val) / get_number_with_default_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else {
            parse_f64(v)
        }
    } else {
        panic!("no key {} inside", key)
    }
}

fn get_number(key: String, map: &HashMap<String, String>) -> f64 {
    if let Some(v) = map.get(key.as_str()) {
        if v.contains(" + ") {
            let split: Vec<&str> = v.split(" + ").collect();
            get_number(split[0].to_string(), &map) + get_number(split[1].to_string(), &map)
        } else if v.contains(" - ") {
            let split: Vec<&str> = v.split(" - ").collect();
            get_number(split[0].to_string(), &map) - get_number(split[1].to_string(), &map)
        } else if v.contains(" * ") {
            let split: Vec<&str> = v.split(" * ").collect();
            get_number(split[0].to_string(), &map) * get_number(split[1].to_string(), &map)
        } else if v.contains(" / ") {
            let split: Vec<&str> = v.split(" / ").collect();
            get_number(split[0].to_string(), &map) / get_number(split[1].to_string(), &map)
        } else {
            parse_f64(v)
        }
    } else {
        panic!("no key {} inside", key)
    }
}

//
//
//
//
//
// #[derive(Debug)]
// enum Yell {
//     Number(i32),
//     MathOp(MathOperation),
// }
//
// #[derive(Debug)]
// struct MathOperation {
//     op_type: OpType,
//     k1: String,
//     k2: String,
//     v1: Option<i32>,
//     v2: Option<i32>,
//     res: Option<i32>,
// }
//
// impl MathOperation {
//
//
//
//     // const Ops: HashMap<OpType, fn(i32, i32) -> i32> = {
//     //     const add: fn(i32, i32) -> i32 = |x: i32, y: i32| x + y;
//     //     const sub: fn(i32, i32) -> i32 = |x: i32, y: i32| x - y;
//     //     const mul: fn(i32, i32) -> i32 = |x: i32, y: i32| x * y;
//     //     const div: fn(i32, i32) -> i32 = |x: i32, y: i32| x / y;
//     //
//     //     HashMap::from([
//     //         (Addition, add),
//     //         (Subtraction, sub),
//     //         (Multiplication, mul),
//     //         (Division, div),
//     //     ])
//     // };
//
//     fn new(op_type: OpType, k1: String, k2: String) -> MathOperation {
//         MathOperation {
//             op_type,
//             k1,
//             k2,
//             v1: None,
//             v2: None,
//             res: None
//         }
//     }
//
//     fn with_v1(&mut self, v1: i32) {
//         self.v1 = Some(v1);
//         if let Some(v2) = self.v2 {
//             // Ops.
//             // self.res =
//         }
//     }
// }
//
// #[derive(Debug)]
// #[derive(PartialEq)]
// #[derive(Eq)]
// #[derive(Hash)]
// enum OpType {
//     Addition,
//     Subtraction,
//     Multiplication,
//     Division,
// }
//
// fn line_to_math_op(l: &str) -> Option<MathOperation> {
//     if l.contains(" + ") {
//         let split: Vec<&str> = l.split(" + ").collect();
//         Some(MathOperation::new(Addition, split[0].to_string(), split[1].to_string()))
//     } else if l.contains(" - ") {
//         let split: Vec<&str> = l.split(" - ").collect();
//         Some(MathOperation::new(Subtraction, split[0].to_string(), split[1].to_string()))
//     } else if l.contains(" * ") {
//         let split: Vec<&str> = l.split(" * ").collect();
//         Some(MathOperation::new(Multiplication, split[0].to_string(), split[1].to_string()))
//     } else if l.contains(" / ") {
//         let split: Vec<&str> = l.split(" / ").collect();
//         Some(MathOperation::new(Division, split[0].to_string(), split[1].to_string()))
//     } else {
//         None
//     }
// }
//
// // fn line_to_math_op(l: &str) -> Option<Op> {
// //     if l.contains(" + ") {
// //         let s = l.split(" + ").collect::<Vec<&str>>();
// //         Some(Addition(s[0].to_string(), s[1].to_string(), None, None, None))
// //     } else if l.contains(" - ") {
// //         let s = l.split(" - ").collect::<Vec<&str>>();
// //         Some(Subtraction(s[0].to_string(), s[1].to_string(), None, None, None))
// //     } else if l.contains(" * ") {
// //         let s = l.split(" * ").collect::<Vec<&str>>();
// //         Some(Multiplication(s[0].to_string(), s[1].to_string(), None, None, None))
// //     } else if l.contains(" / ") {
// //         let s = l.split(" / ").collect::<Vec<&str>>();
// //         Some(Division(s[0].to_string(), s[1].to_string(), None, None, None))
// //     } else {
// //         None
// //     }
// // }
//
//
//
//
//

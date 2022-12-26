extern crate core;

use std::cmp::max;
use std::collections::HashMap;
// use std::ops::Div;
// use std::string::ToString;
use aoctools::{parse_i128, read_whole_file};
// use crate::OpType::{Addition, Division, Multiplication, Subtraction};
// use crate::Yell::{MathOp, Number};

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

    println!("Part one: {:?}", get_number("root".to_string(), &map));

    let root = &map.get("root");
    println!("root: {}", root.unwrap());;
    let left = root.unwrap()[0..4].to_string();
    let right = root.unwrap()[7..11].to_string();
    println!("left: {}, right: {}", left, right);
    println!("left: {}, right: {}", get_number(left.to_string(), &map), get_number(right.to_string(), &map));

    let you_key: String = String::from("humn");
    let you_val = 3502;

    let right_res = get_number_with_different_for_you(right.to_string(), &map, &you_key, you_val);
    // let mut next_guess = 3848301405790 as i128;
    let mut next_guess = -34543543534 as i128;
    let mut left_res = get_number_with_different_for_you(left.to_string(), &map, &you_key, next_guess);
    while (left_res != right_res) {
        println!("  -->  {} {} {}", left_res, right_res, next_guess);
        left_res = get_number_with_different_for_you(left.to_string(), &map, &you_key, next_guess);
        if (left_res > right_res) {
            next_guess += max(1, left_res - right_res / 10);
            // next_guess += 1;
        } else if (left_res < right_res) {
            next_guess -= max(1, right_res - left_res / 10);
            // next_guess -= 1;
        }
    }





    println!("Part two: {}", next_guess);


        // + 1_000_000_000 -> *10
    // 2265089891482 -> 2256183602420
    // 2256183602420



}

// Part one: 38731621732448
// Part two: 3848301405790 - too high

fn get_number_with_different_for_you(key: String, map: &HashMap<String, String>, you_key: &String, you_val: i128) -> i128 {
    if key == *you_key {
        you_val
    } else if let Some(v) = map.get(key.as_str()) {
        if v.contains(" + ") {
            let split: Vec<&str> = v.split(" + ").collect();
            get_number_with_different_for_you(split[0].to_string(), &map, &you_key, you_val) + get_number_with_different_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else if v.contains(" - ") {
            let split: Vec<&str> = v.split(" - ").collect();
            get_number_with_different_for_you(split[0].to_string(), &map, &you_key, you_val) - get_number_with_different_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else if v.contains(" * ") {
            let split: Vec<&str> = v.split(" * ").collect();
            get_number_with_different_for_you(split[0].to_string(), &map, &you_key, you_val) * get_number_with_different_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else if v.contains(" / ") {
            let split: Vec<&str> = v.split(" / ").collect();
            get_number_with_different_for_you(split[0].to_string(), &map, &you_key, you_val) / get_number_with_different_for_you(split[1].to_string(), &map, &you_key, you_val)
        } else {
            parse_i128(v)
        }
    } else {
        panic!("no key {} inside", key)
    }
}

fn get_number(key: String, map: &HashMap<String, String>) -> i128 {
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
            parse_i128(v)
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

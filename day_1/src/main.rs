use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let file_name = String::from("input_part1.txt");
    let file = File::open(file_name).expect("Can't read a file");
    let buffer = BufReader::new(file);
    let mut v = Vec::new();
    v.push(0);
    let mut idx = 0;
    let mut max_values = vec![0, 0, 0];
    'lines: for line in buffer.lines() {
        let str = line?;
        if str.trim().is_empty() {
            v.push(0);
            update_vec(&mut max_values, v[idx]);
            idx += 1;
            continue 'lines;
        }
        let num: i32 = str.trim().parse().unwrap();
        v[idx] = v[idx] + num;
    }
    println!("Part one: {:?}", max_values[0]);
    println!("Part two: {:?}", max_values.iter().sum::<i32>());
    Ok(())
}

fn update_vec(v: &mut Vec<i32>, mut value: i32) -> () {
    for i in 0..3 {
        if value > v[i] {
            let old_value = v[i];
            v[i] = value;
            value = old_value;
        }
    }
}
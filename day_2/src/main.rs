use std::fs::File;
use std::io::{ BufReader, Result, Lines, prelude::* };

fn main() -> Result<()>{
    let mut sum_score_part_1 = 0;
    let mut sum_score_part_2 = 0;
    for line_res in read_file("input.txt") {
        let line = line_res?;
        let a: Vec<char> = line.split(" ").map(|x| x.chars().next().unwrap()).collect();
        sum_score_part_1 += a[1] as i32 - 65 - 22 + ((a[1] as i32 - 23 - a[0] as i32) + 4) % 3 * 3;
        sum_score_part_2 += (a[1] as i32 - 65 - 22) * 3 - 3 + (a[1] as i32 - 65 - 23 + a[0] as i32 - 65 + 2) % 3 + 1;
    }
    println!("Part one: {:?}", sum_score_part_1);
    println!("Part two: {:?}", sum_score_part_2);
    Ok(())
}

fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file_name = String::from(file_name);
    let file = File::open(file_name).expect("Can't read a file");
    BufReader::new(file).lines()
}

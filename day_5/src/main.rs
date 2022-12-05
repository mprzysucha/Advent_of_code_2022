use std::collections::LinkedList;
use aoctools::{parse, read_file, read_file_and_process};

fn main() {

    const NOE: usize = 9;
    let mut positions = [1; NOE];
    for i in 1..NOE {
        positions[i] = positions[i - 1] + 4;
    }
    let mut elements: Vec<LinkedList<char>> = Vec::new();
    let mut elements2: Vec<LinkedList<char>> = Vec::new();
    for i in 0..NOE {
        elements.push(LinkedList::new());
        elements2.push(LinkedList::new());
    }

    for line_res in read_file("input.txt") {
        match line_res {
            Ok(line) => {
                if line.starts_with("[") {
                    let chars = line.chars().collect::<Vec<char>>();
                    for i in 0..NOE {
                        if chars[positions[i]] != ' ' {
                            elements[i].push_back(chars[positions[i]]);
                            elements2[i].push_back(chars[positions[i]]);
                        }
                    }
                } else if line.starts_with("move") {
                    let split = line.split(" ").collect::<Vec<&str>>();
                    let amount: usize = parse(split[1]) as usize;
                    let from = parse(split[3]) as usize;
                    let to = parse(split[5]) as usize;
                    let mut temp_buf: LinkedList<char> = LinkedList::new();
                    for i in 0..amount {

                        // part 1
                        let e = elements[from - 1].pop_front().unwrap();
                        elements[to - 1].push_front(e);

                        // part 2
                        let e2 = elements2[from - 1].pop_front().unwrap();
                        temp_buf.push_front(e2);
                    }

                    // part 2
                    for i in 0..amount {
                        elements2[to - 1].push_front(temp_buf.pop_front().unwrap());
                    }
                }
            }
            Err(e) => panic!("Problem when reading a file: {:?}", e),
        }
    }

    print!("Part one: ");
    for i in 0..NOE {
        print!("{}", elements[i].pop_front().unwrap());
    }
    println!();

    print!("Part two: ");
    for i in 0..NOE {
        print!("{}", elements2[i].pop_front().unwrap());
    }
    println!();


    // to be used in future, I don't know how to mutate variable inside closure
    // maybe check interior mutability ??? not sure
    // https://users.rust-lang.org/t/cannot-borrow-write-as-mutable-as-it-is-a-captured-variable-in-a-fn-closure/78506
    // https://doc.rust-lang.org/reference/interior-mutability.html
    read_file_and_process("input.txt", |line: &str| -> () {


    });
}

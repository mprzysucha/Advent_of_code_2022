use aoctools::read_file;
use foollections::FixedVecDequeue;

fn main() {
    match read_file("input.txt").next() {
        Some(Ok(line)) => {
            println!("Part one: {}", process_line(line.as_str(), 4));
            println!("Part two: {}", process_line(line.as_str(), 14));
        }
        Some(Err(e)) => panic!("Error reading file: {}", e),
        None => panic!("Empty file"),
    }
}

trait UniqueElemsQueue {
    fn all_different(&self) -> bool;
}

impl<T : PartialEq> UniqueElemsQueue for FixedVecDequeue<T> {
    fn all_different(&self) -> bool {
        for i in 0..self.size() - 1 {
            for j in 1..self.size() {
                if i != j && *self.get(i) == *self.get(j) {
                    return false;
                }
            }
        }
        return true;
    }
}

fn process_line(line: &str, num_of_distinct_char: usize) -> usize {
    let mut q: FixedVecDequeue<char> = FixedVecDequeue::new(num_of_distinct_char);
    let mut counter = 0;
    for c in line.chars() {
        q.push(c);
        counter += 1;
        if q.size() == num_of_distinct_char && q.all_different() {
            break;
        }
    }
    counter
}

pub mod foollections {
    use std::collections::VecDeque;

    pub struct FixedVecDequeue<T> {
        queue: VecDeque<T>,
        size: usize,
    }

    impl<T> FixedVecDequeue<T> {
        pub fn new(size: usize) -> FixedVecDequeue<T> {
            FixedVecDequeue {
                queue: VecDeque::with_capacity(size),
                size,
            }
        }
        pub fn push(&mut self, elem: T) -> () {
            if self.queue.len() == self.size {
                self.queue.pop_front();
            }
            self.queue.push_back(elem);
        }
        pub fn size(&self) -> usize {
            self.queue.len()
        }
        pub fn get(&self, i: usize) -> &T {
            if i >= self.queue.len() {
                panic!("Queue too small")
            }
            &self.queue[i]
        }
    }

}

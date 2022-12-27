use std::cell::RefCell;
use std::collections::{HashSet, LinkedList};
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;
use aoctools::{parse, read_whole_file};
use crate::Direction::{BackForSnack, FirstGo, GoWithSnack};
use crate::List::{Cons, Nil};

fn main() {

    let content = read_whole_file("input.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| !s.trim().is_empty()).collect();

    let mut blizzards0: Vec<Blizzard> = Vec::new();

    let height = lines.len() - 2;
    let max_y = height - 1;
    let mut max_x = 0;
    let mut number_of_blizzards = 0;
    for i in 0..height {
        let chars = lines[i + 1].chars().collect::<Vec<char>>();
        let num_of_chars = chars.len();
        max_x = num_of_chars - 3;
        for j in 0..num_of_chars-2 {
            if chars[j + 1] != '.' {
                blizzards0.push(Blizzard::new(j, i, chars[j + 1]));
                number_of_blizzards += 1;
            }
        }
    }

    let now = Instant::now();
    let mut minute: usize = 0;

    let blizzards: Rc<RefCell<Vec<Blizzard>>> = Rc::new(RefCell::new(blizzards0));

    let mut direction: Direction = FirstGo;
    let previous = Rc::new(Nil);

    let mut tasks: LinkedList<LinkedList<Task>> = LinkedList::new();
    let start_position = direction.start_position(max_x, max_y);
    let task = Task {
        expedition: (start_position.0 as i32, start_position.1 as i32),
        previous: Rc::clone(&previous),
    };

    tasks.push_back(LinkedList::from([task]));
    let mut stop = false;

    let mut part_one_answer = 0;
    let mut part_two_answer = 0;

    while !stop {

        let mut tasks_for_this_minute: LinkedList<Task> = tasks.pop_front().unwrap();
        let mut not_moved_yet = true; // move blizzards only once per each minute
        let mut tasks_for_next_minute: LinkedList<Task> = LinkedList::new();
        let mut adjacent_for_next_minute: HashSet<(i32, i32)> = HashSet::new();

        if tasks_for_this_minute.len() == 0 {
            move_blizzards(Rc::clone(&blizzards), number_of_blizzards, max_x, max_y);
        }

        while let Some(task) = tasks_for_this_minute.pop_front() {

            let expedition = task.expedition;

            let solution = direction.solution(max_x, max_y);
            let solution_found = expedition.0 == solution.0 && expedition.1 == solution.1;
            if solution_found {
                if direction == FirstGo {
                    // let list = Rc::new(Cons(expedition, Rc::clone(&task.previous)));
                    // let mut path: LinkedList<(i32, i32)> = LinkedList::new();
                    // build_path(Rc::clone(&list), &mut path);
                    // print_all(path, Rc::clone(&blizzards), minute, max_x, max_y);
                    part_one_answer = minute;
                    // stop = true;
                }
                if direction == GoWithSnack {
                    let list = Rc::new(Cons(expedition, Rc::clone(&task.previous)));
                    let mut path: LinkedList<(i32, i32)> = LinkedList::new();
                    build_path(Rc::clone(&list), &mut path);
                    print_all(path, Rc::clone(&blizzards), minute, max_x, max_y);
                    part_two_answer = minute;
                    stop = true;
                }
                while let Some(_) = tasks_for_this_minute.pop_front() {}
                while let Some(_) = tasks_for_next_minute.pop_front() {}
                adjacent_for_next_minute.clear();
                direction = direction.next();
            }
            if not_moved_yet {
                move_blizzards(Rc::clone(&blizzards), number_of_blizzards, max_x, max_y);
                not_moved_yet = false;
            }
            let adjacents = vec!((expedition.0, expedition.1 - 1),
                                 (expedition.0 - 1, expedition.1),
                                 (expedition.0, expedition.1),
                                 (expedition.0 + 1, expedition.1),
                                 (expedition.0, expedition.1 + 1));
            for i in 0..adjacents.len() {
                let x = adjacents[i].0;
                let y = adjacents[i].1;
                let normal_move = x >= 0 && x <= max_x as i32 && y >= 0 && y <= max_y as i32;
                let solution_move = x == solution.0 && y == solution.1;
                let dont_move = x == expedition.0 && y == expedition.1; // used for start position - e.g. can stay in (0, -1) but can't move there from (0, 0)
                if normal_move || solution_move || dont_move {
                    let blizzard_here = check_if_blizzard_here(Rc::clone(&blizzards), number_of_blizzards, x as usize, y as usize);
                    if !blizzard_here && !adjacent_for_next_minute.contains(&adjacents[i]) {
                        let previous = Rc::new(Cons(expedition, Rc::clone(&task.previous)));
                        let task = Task {
                            expedition: adjacents[i],
                            previous: previous,
                        };
                        tasks_for_next_minute.push_back(task);
                        adjacent_for_next_minute.insert(adjacents[i]);
                    }
                }
            }
        }
        minute += 1;
        tasks.push_back(tasks_for_next_minute);
    }

    println!("Part one: {}", part_one_answer);
    println!("Part two: {}", part_two_answer);


    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);

}

#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    FirstGo,
    BackForSnack,
    GoWithSnack,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            FirstGo => BackForSnack,
            BackForSnack => GoWithSnack,
            GoWithSnack => FirstGo,
        }
    }

    fn _first_move(&self, max_x: usize, max_y: usize) -> (usize, usize) {
        match self {
            FirstGo | GoWithSnack => (0, 0),
            BackForSnack => (max_x, max_y),
        }
    }

    fn start_position(&self, max_x: usize, max_y: usize) -> (i32, i32) {
        match self {
            FirstGo | GoWithSnack => (0, -1),
            BackForSnack => (max_x as i32, max_y as i32 + 1),
        }
    }

    fn solution(&self, max_x: usize, max_y: usize) -> (i32, i32) {
        match self {
            FirstGo | GoWithSnack => (max_x as i32, max_y as i32 + 1),
            BackForSnack => (0, -1),
        }
    }
}

fn move_blizzards(blizzards: Rc<RefCell<Vec<Blizzard>>>, number_of_blizzards: usize, max_x: usize, max_y: usize) {
    for i in 0..number_of_blizzards {
        (*blizzards).borrow_mut()[i].mov(max_x, max_y);
    }
}

fn _move_blizzards_and_print(blizzards: Rc<RefCell<Vec<Blizzard>>>, number_of_blizzards: usize, max_x: usize, max_y: usize, expedition: (i32, i32)) {
    for i in 0..number_of_blizzards {
        (*blizzards).borrow_mut()[i].mov(max_x, max_y);
    }
    print(Rc::clone(&blizzards), expedition, max_x, max_y);
}

fn check_if_blizzard_here(blizzards: Rc<RefCell<Vec<Blizzard>>>, number_of_blizzards: usize, check_x: usize, check_y: usize) -> bool {
    let mut blizzard_here = false;
    let mut j = 0;
    while !blizzard_here && j < number_of_blizzards {
        let bs = blizzards.deref().borrow();
        if bs[j].x == check_x && bs[j].y == check_y {
            blizzard_here = true;
        }
        j += 1;
    }
    blizzard_here
}

fn _wait() {
    use std::io;
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn _print_path(list: Rc<List>, minute: i32) {
    match list.deref() {
        Cons((x, y), prev) => {
            println!("minute {}: ({}, {})", minute, x, y);
            _print_path(Rc::clone(prev), minute - 1);
        }
        Nil => (),
    }
}

fn build_path_rec(list: Rc<List>, path: &mut LinkedList<(i32, i32)>) {
    match list.deref() {
        Cons((x, y), prev) => {
            let xx = *x;
            let yy = *y;
            path.push_front((xx, yy));
            // println!("minute {}: ({}, {})", minute, x, y);
            build_path_rec(Rc::clone(prev), path);
        }
        Nil => (),
    }
}

fn build_path(list: Rc<List>, path: &mut LinkedList<(i32, i32)>) {
    build_path_rec(Rc::clone(&list), path);
    // path.pop_front(); // remove minut zero
}

fn print_all(path: LinkedList<(i32, i32)>, blizzards: Rc<RefCell<Vec<Blizzard>>>, minutes: usize, max_x: usize, max_y: usize) {
    let mut bs = blizzards.deref().borrow_mut();
    for _ in 0..minutes + 1 { // + 1 to show initial state
        for i in 0..bs.len() {
            bs[i].mov_back(max_x, max_y);
        }
    }
    let mut bs2 = vec![];
    let mut num_of_blizzards = 0;
    for i in 0..bs.len() {
        bs2.push(Blizzard::new(bs[i].x, bs[i].y, bs[i].d));
        num_of_blizzards += 1;
    }
    let rc_bs2 = Rc::new(RefCell::new(bs2));
    for _ in 0..minutes + 1 {
        for i in 0..bs.len() {
            bs[i].mov(max_x, max_y);
        }
    }
    let mut m = 0;
    println!("************ PATH ************");
    for p in path {
        if m > 0 {
            println!("\nMinute: {}", m);
        } else {
            println!("\nInitial state:");
        }
        print(Rc::clone(&rc_bs2), p, max_x, max_y);
        move_blizzards(Rc::clone(&rc_bs2), num_of_blizzards, max_x, max_y);
        m += 1;
    }
    println!();
    println!("************ PATH ************");
}

fn print(blizzards: Rc<RefCell<Vec<Blizzard>>>, expedition: (i32, i32), max_x: usize, max_y: usize) {
    let mut m: Vec<Vec<char>> = vec![];
    for x in 0..max_x + 1 {
        m.push(vec![]);
        for y in 0..max_y + 1 {
            if expedition.0 != x as i32 || expedition.1 != y as i32 {
                m[x].push('.');
            } else {
                m[x].push('E');
            }

        }
    }

    let bs = blizzards.deref().borrow();
    for i in 0..bs.len() {
        let cu = m[bs[i].x][bs[i].y];
        if cu == '.' {
            m[bs[i].x][bs[i].y] = bs[i].d;
        } else if cu == '<' || cu == '>' || cu == '^' || cu == 'v' {
            m[bs[i].x][bs[i].y] = '2';
        } else if cu != 'E' {
            m[bs[i].x][bs[i].y] = (parse(cu.to_string().as_str()) + 1).to_string().chars().next().unwrap();
        }
    }

    if expedition.0 == 0 && expedition.1 == -1 {
        print!("#E#");
    } else {
        print!("#.#");
    }
    for _ in 0..max_x {
        print!("#");
    }
    println!();
    for y in 0..max_y + 1 {
        print!("#");
        for x in 0..max_x + 1 {
            print!("{}", m[x][y]);
        }
        println!("#");
    }
    for _ in 0..max_x + 1 {
        print!("#");
    }
    if expedition.0 == max_x as i32 && expedition.1 == max_y as i32 + 1 {
        print!("E");
    } else {
        print!(".");
    }
    print!("#");
    println!()
}

#[derive(Debug)]
enum List {
    Cons((i32, i32), Rc<List>),
    Nil
}

#[derive(Debug)]
struct Task {
    expedition: (i32, i32),
    previous: Rc<List>,
}

#[derive(Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    d: char,
}

impl Blizzard {
    fn new(x: usize, y: usize, d: char) -> Blizzard {
        // println!("Blizzard ( x: {}, y: {}, d: {} )", x, y, d);
        Blizzard { x, y, d }
    }

    fn mov_back(&mut self, max_x: usize, max_y:usize) {
        if self.d == '>' {
            if self.x > 0 {
                self.x -= 1;
            } else {
                self.x = max_x;
            }
        } else if self.d == '<' {
            if self.x < max_x {
                self.x += 1;
            } else {
                self.x = 0;
            }
        } else if self.d == 'v' {
            if self.y > 0 {
                self.y -= 1;
            } else {
                self.y = max_y;
            }
        } else if self.d == '^' {
            if self.y < max_y {
                self.y += 1;
            } else {
                self.y = 0;
            }
        }
    }

    fn mov(&mut self, max_x: usize, max_y:usize) {
        if self.d == '<' {
            if self.x > 0 {
                self.x -= 1;
            } else {
                self.x = max_x;
            }
        } else if self.d == '>' {
            if self.x < max_x {
                self.x += 1;
            } else {
                self.x = 0;
            }
        } else if self.d == '^' {
            if self.y > 0 {
                self.y -= 1;
            } else {
                self.y = max_y;
            }
        } else if self.d == 'v' {
            if self.y < max_y {
                self.y += 1;
            } else {
                self.y = 0;
            }
        }
    }

}
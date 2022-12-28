use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};
use aoctools::read_whole_file;

fn main() {
    let now_whole = Instant::now();

    let content = read_whole_file("input.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| !s.trim().is_empty()).collect();

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut elves: Vec<Elve> = Vec::new();

    let mut elve_num = 0;
    for y in 0..lines.len() {
        matrix.push(Vec::new());
        for c in lines[y].chars() {
            if c == '#' {
                let e = Elve {
                    number: elve_num,
                    x: RefCell::new(matrix[y].len() as i32),
                    y: RefCell::new(y as i32),
                };
                elves.push(e);
                elve_num += 1;
            }
            matrix[y].push(c);
        }
    }

    let north = Rc::new(Direction::N);
    let south = Rc::new(Direction::S);
    let west = Rc::new(Direction::W);
    let east = Rc::new(Direction::E);

    let mut first_direction = Rc::clone(&north);

    let mut stop = 0;
    let mut round = 0;

    let mut sum_of_durations = Duration::new(0, 0);
    let mut number_of_durations = 0;

    while stop < 2 {
        let now = Instant::now();

        round += 1;
        let mut propositions: HashMap<i32, (String, Rc<Direction>)> = HashMap::new();

        let mut els = vec![Duration::new(0, 0), Duration::new(0, 0), Duration::new(0, 0)];

        let mut map: Vec<Vec<bool>> = Vec::new();
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for elve in &elves {
            if elve.x() < min_x {
                min_x = elve.x();
            }
            if elve.x() > max_x {
                max_x = elve.x();
            }
            if elve.y() < min_y {
                min_y = elve.y();
            }
            if elve.y() > max_y {
                max_y = elve.y();
            }
        }
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;
        for i in 0..=(max_x-min_x) {
            map.push(Vec::new());
            for j in 0..=(max_y-min_y) {
                map[i as usize].push(false);
            }
        }
        for elve in &elves {
            map[(elve.x()-min_x) as usize][(elve.y()-min_y) as usize] = true;
        }
        // pri(&map);

        let build_map_time = now.elapsed();

        let now = Instant::now();
        for elve in &elves {
            let now0 = Instant::now();
            if !elve.should_i_move(&elves, &map, min_x, min_y) {
                continue;
            }
            let mut direction = Rc::clone(&first_direction);
            let mut direction_count = 0;

            els[0] += now0.elapsed();
            let now1 = Instant::now();

            while direction_count < 4 && !elve.can_i_move(&direction, &elves, &map, min_x, min_y) {
                direction = match direction.next() {
                    Direction::N => Rc::clone(&north),
                    Direction::S => Rc::clone(&south),
                    Direction::W => Rc::clone(&west),
                    Direction::E => Rc::clone(&east),
                };
                direction_count += 1;
            }

            els[1] += now1.elapsed();
            let now2 = Instant::now();

            if direction_count < 4 {
                let d = &*direction.borrow();
                match d {
                    Direction::N => propositions.insert(elve.id(), (cord(elve.x(),elve.y() - 1), Rc::clone(&direction))),
                    Direction::S => propositions.insert(elve.id(), (cord(elve.x(),elve.y() + 1), Rc::clone(&direction))),
                    Direction::W => propositions.insert(elve.id(), (cord(elve.x() - 1,elve.y()), Rc::clone(&direction))),
                    Direction::E => propositions.insert(elve.id(), (cord(elve.x() + 1,elve.y()), Rc::clone(&direction))),
                };
            }

            els[2] += now2.elapsed();


        }

        // println!("els: {:?}", els);

        let elapsed1 = now.elapsed();

        let mut propositions_counter: HashMap<String, i32> = HashMap::new();
        for proposition in &propositions {
            let k = proposition.1.0.to_string();
            if propositions_counter.contains_key(&k) {
                propositions_counter.insert((&k).to_string(), propositions_counter.get(&k).unwrap() + 1);
            } else {
                propositions_counter.insert((&k).to_string(), 1);
            }
        }
        let elapsed2 = now.elapsed();

        let fd = &*first_direction.borrow();
        match fd {
            Direction::N => first_direction = Rc::clone(&south),
            Direction::S => first_direction = Rc::clone(&west),
            Direction::W => first_direction = Rc::clone(&east),
            Direction::E => first_direction = Rc::clone(&north),
        }
        let mut number_of_moves = 0;
        for proposition in &propositions {
            let k = proposition.1.0.to_string();
            let d = proposition.1.1.borrow();
            if *propositions_counter.get(&k).unwrap() == 1 {
                number_of_moves += 1;
                match d {
                    Direction::N => (&elves).into_iter().find(|e| e.id() == *proposition.0).unwrap().move_n(),
                    Direction::S => (&elves).into_iter().find(|e| e.id() == *proposition.0).unwrap().move_s(),
                    Direction::W => (&elves).into_iter().find(|e| e.id() == *proposition.0).unwrap().move_w(),
                    Direction::E => (&elves).into_iter().find(|e| e.id() == *proposition.0).unwrap().move_e(),
                };
            }

        }
        let elapsed3 = now.elapsed();

        let elapsed = now.elapsed();
        sum_of_durations += elapsed;
        number_of_durations += 1;
        // let avg_duration = sum_of_durations / number_of_durations;
        // println!("Round {}, number of moves {} time {:?} avg time: {:?}     build_map_time: {:?}, after propositions: {:?}, after propositions count: {:?} (only count {:?}), after moves: {:?} (only moves: {:?})  ", round, number_of_moves, elapsed, avg_duration, build_map_time, elapsed1, elapsed2, elapsed2 - elapsed1, elapsed3, elapsed3 - elapsed2);

        if number_of_moves == 0 {
            println!("Part two: {}", round);
            stop += 1;
        }

        if round == 10 {
            let mut min_x = elves[0].x();
            let mut max_x = elves[0].x();
            let mut min_y = elves[0].y();
            let mut max_y = elves[0].y();
            let mut number_of_elves = 0;
            for elve in &elves {
                number_of_elves += 1;
                if elve.x() < min_x {
                    min_x = elve.x();
                }
                if elve.x() > max_x {
                    max_x = elve.x();
                }
                if elve.y() < min_y {
                    min_y = elve.y();
                }
                if elve.y() > max_y {
                    max_y = elve.y()
                }
            }
            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let area = width * height;
            let area_without_elves = area - number_of_elves;

            println!("Part one: {}", area_without_elves);
            stop += 1;

        }



    }

    println!("Time: {:?}", now_whole.elapsed());

}

fn pri(map: &Vec<Vec<bool>>) {
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            if map[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn cord(x: i32, y: i32) -> String {
    let mut id = x.to_string();
    id.push_str(":");
    id.push_str(y.to_string().as_str());
    id
}

#[derive(Debug)]
enum Direction {
    N, S, W, E
}

impl Direction {
    pub fn next(&self) -> Direction {
        match &self {
            Direction::N => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::E,
            Direction::E => Direction::N,
        }
    }
}

#[derive(Debug)]
struct Elve {
    number: i32,
    x: RefCell<i32>,
    y: RefCell<i32>,
}

impl Elve {

    pub fn id(&self) -> i32 {
        self.number
    }

    fn w_border(&self, min_x: i32) -> bool {
        self.x() - min_x == 0
    }
    fn e_border(&self, min_x: i32, max_x: i32) -> bool {
        self.x() - min_x == max_x
    }
    fn n_border(&self, min_y: i32) -> bool {
        self.y() - min_y == 0
    }
    fn s_border(&self, min_y: i32, max_y: i32) -> bool {
        self.y() - min_y == max_y
    }


    pub fn should_i_move(&self, elves: &Vec<Elve>, map: &Vec<Vec<bool>>, min_x: i32, min_y: i32) -> bool {
        let nw = map[((self.x() - 1)-min_x) as usize][((self.y() - 1)-min_y) as usize];
        let n  = map[((self.x()    )-min_x) as usize][((self.y() - 1)-min_y) as usize];
        let ne = map[((self.x() + 1)-min_x) as usize][((self.y() - 1)-min_y) as usize];
        let w  = map[((self.x() - 1)-min_x) as usize][((self.y()    )-min_y) as usize];
        let e  = map[((self.x() + 1)-min_x) as usize][((self.y()    )-min_y) as usize];
        let sw = map[((self.x() - 1)-min_x) as usize][((self.y() + 1)-min_y) as usize];
        let s  = map[((self.x()    )-min_x) as usize][((self.y() + 1)-min_y) as usize];
        let se = map[((self.x() + 1)-min_x) as usize][((self.y() + 1)-min_y) as usize];
        nw || n || ne || e || w || sw || s || se
    }

    pub fn can_i_move(&self, d: &Direction, elves: &Vec<Elve>, map: &Vec<Vec<bool>>, min_x: i32, min_y: i32) -> bool {
        match d {
            Direction::N => {
                let nw = map[((self.x() - 1)-min_x) as usize][((self.y() - 1)-min_y) as usize];
                let n  = map[((self.x()    )-min_x) as usize][((self.y() - 1)-min_y) as usize];
                let ne = map[((self.x() + 1)-min_x) as usize][((self.y() - 1)-min_y) as usize];
                !nw && !n && !ne
            },
            Direction::S => {
                let sw = map[((self.x() - 1)-min_x) as usize][((self.y() + 1)-min_y) as usize];
                let s  = map[((self.x()    )-min_x) as usize][((self.y() + 1)-min_y) as usize];
                let se = map[((self.x() + 1)-min_x) as usize][((self.y() + 1)-min_y) as usize];
                !sw && !s && !se
            },
            Direction::W => {
                let nw = map[((self.x() - 1)-min_x) as usize][((self.y() - 1)-min_y) as usize];
                let w  = map[((self.x() - 1)-min_x) as usize][((self.y()    )-min_y) as usize];
                let sw = map[((self.x() - 1)-min_x) as usize][((self.y() + 1)-min_y) as usize];
                !nw && !w && !sw
            },
            Direction::E => {
                let ne = map[((self.x() + 1)-min_x) as usize][((self.y() - 1)-min_y) as usize];
                let e  = map[((self.x() + 1)-min_x) as usize][((self.y()    )-min_y) as usize];
                let se = map[((self.x() + 1)-min_x) as usize][((self.y() + 1)-min_y) as usize];
                !ne && !e && !se
            },
        }
    }
    pub fn x(&self) -> i32 {
        *self.x.borrow()
    }
    pub fn y(&self) -> i32 {
        *self.y.borrow()
    }
    pub fn move_n(&self) {
        *self.y.borrow_mut() -= 1;
    }

    pub fn move_s(&self) {
        *self.y.borrow_mut() += 1;
    }

    pub fn move_w(&self) {
        *self.x.borrow_mut() -= 1;
    }

    pub fn move_e(&self) {
        *self.x.borrow_mut() += 1;
    }
}

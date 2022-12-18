use std::rc::Rc;
use aoctools::{parse, read_whole_file};

fn main() {
    let positions: Vec<Position> = read_whole_file("input.txt").split("\n").filter(|s| !s.trim().is_empty()).map(|s| s.split(",").map(parse).collect::<Position>()).collect();
    let mut sum_exposed_sides = 0;
    let mut max = Position(0, 0, 0);
    for i in 0..positions.len() {
        if &positions[i].0 > &max.0 {
            max.0 = positions[i].0;
        }
        if &positions[i].1 > &max.1 {
            max.1 = positions[i].1;
        }
        if &positions[i].2 > &max.2 {
            max.2 = positions[i].2;
        }
    }
    let mut externals: Vec<Rc<Position>> = Vec::new();
    let mut visited: Vec<Rc<Position>> = Vec::new();
    external(Rc::new(Position(0, 0, 0)), &mut visited, &mut externals, &positions, &max);

    for i in 0..positions.len() {
        let mut exposed_sides = 6;
        for j in 0..positions.len() {
            if i == j {
                continue;
            }

            if is_next(&positions[i], &positions[j]) {
                exposed_sides -= 1;
            }

        }
        sum_exposed_sides += exposed_sides;

    }
    println!("Part one: {}", sum_exposed_sides);


    let mut sides = 0;
    for i in 0..positions.len() {

        let next = Rc::new(Position(positions[i].0 - 1, positions[i].1, positions[i].2));
        if externals.contains(&next) && !positions.contains(&next) {
            sides += 1;
        }
        let next = Rc::new(Position(positions[i].0 + 1, positions[i].1, positions[i].2));
        if externals.contains(&next) && !positions.contains(&next) {
            sides += 1;
        }

        let next = Rc::new(Position(positions[i].0,positions[i].1 - 1, positions[i].2));
        if externals.contains(&next) && !positions.contains(&next) {
            sides += 1;
        }
        let next = Rc::new(Position(positions[i].0,positions[i].1 + 1, positions[i].2));
        if externals.contains(&next) && !positions.contains(&next) {
            sides += 1;
        }

        let next = Rc::new(Position(positions[i].0,positions[i].1, positions[i].2 - 1));
        if externals.contains(&next) && !positions.contains(&next) {
            sides += 1;
        }
        let next = Rc::new(Position(positions[i].0,positions[i].1, positions[i].2 + 1));
        if externals.contains(&next) && !positions.contains(&next) {
            sides += 1;
        }

    }

    println!("Part two: {}", sides);

}

fn external(p: Rc<Position>, visited: &mut Vec<Rc<Position>>, externals: &mut Vec<Rc<Position>>, lava: &Vec<Position>, max: &Position) {
    let x = p.0;
    let y = p.1;
    let z = p.2;
    visited.push(Rc::clone(&p));
    if lava.contains(&p) || p.0 < -1 || p.0 > max.0 + 1 || p.1 < -1 || p.1 > max.1 + 1  || p.2 < -1 || p.2 > max.2 + 1 {
        return;
    }

    externals.push(Rc::clone(&p));

    let check = Rc::new(Position(x - 1, y, z));
    if !visited.contains(&check) {
        external(check, visited, externals, lava, &max);
    }

    let check =  Rc::new(Position(x + 1, y, z));
    if !visited.contains(&check) {
        external(check, visited, externals, lava, &max);
    }

    let check =  Rc::new(Position(x , y - 1, z));
    if !visited.contains(&check) {
        external(check, visited, externals, lava, &max);
    }

    let check =  Rc::new(Position(x , y + 1, z));
    if !visited.contains(&check) {
        external(check, visited, externals, lava, &max);
    }

    let check =  Rc::new(Position(x , y, z - 1));
    if !visited.contains(&check) {
        external(check, visited, externals, lava, &max);
    }

    let check =  Rc::new(Position(x , y, z + 1));
    if !visited.contains(&check) {
        external(check, visited, externals, lava, &max);
    }

}

fn is_next(p1: &Position, p2: &Position) -> bool {
    let d = diff(p1, p2);
    let res = (d.0 == 0 && d.1 == 0 && d.2 == 1) || (d.0 == 0 && d.1 == 1 && d.2 == 0) || (d.0 == 1 && d.1 == 0 && d.2 == 0);
    res
}

fn diff(p1: &Position, p2: &Position) -> Position {
    Position((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs(), (p1.2 - p2.2).abs())
}

#[derive(Debug)]
struct Position(i32, i32, i32);

impl FromIterator<u32> for Position {
    fn from_iter<T: IntoIterator<Item=u32>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        match (it.next(), it.next(), it.next()) {
            (Some(x), Some(y), Some(z)) => Position(x as i32, y as i32, z as i32),
            _ => panic!("Something is wrong with numbers"),
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
        // self.0 != other.0 || self.1 != other.1 || self.2 != other.2
    }
}
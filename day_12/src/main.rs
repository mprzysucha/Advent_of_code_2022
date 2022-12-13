use std::cmp::min;
use std::fs;
use std::rc::Rc;
use crate::Route::{Cons, Nil};

const START: i16 = -13; // 'S' as i16 - 96
const END: i16 = -27; // 'E' as i16 - 96

fn main() {
    println!("Hello, world!");



    let matrix: Vec<Vec<i16>> = read_whole_file("input.txt").split("\n").filter(|s| s.trim().len() > 0).map(|s| s.split("").map(|s| s.chars().next()).filter(|o| o.is_some()).map(|o| toi(o.unwrap())).collect::<Vec<i16>>()).collect::<Vec<Vec<i16>>>();

    let height: usize = matrix.len(); // 41
    let width: usize = matrix[0].len(); //81

    let mut visited: Vec<Vec<i32>> = Vec::new();

    let mut start_point = Point::unknown();
    let mut _end_point = Point::unknown();

    for h in 0..height {
        visited.push(Vec::new());
        for w in 0..width {
            visited[h].push(0);
            if matrix[h][w] == START {
                start_point = Point(h, w);
            }
            if matrix[h][w] == END {
                _end_point = Point(h, w);
            }
        }
    }






    let route = Rc::new(Cons(Rc::new(Node {
        point: Rc::new(Point(start_point.0, start_point.1)),
        elevation: matrix[start_point.0][start_point.1],
        length: 0
    }), Rc::new(Nil)));

    let result = next_steps(Rc::clone(&route), &matrix, &mut visited, true);

    println!("Part one {:?}", result);

    let mut results: Vec<i16> = Vec::new();


    for h in 0..height {
        for w in 0..width {
            if matrix[h][w] == START || matrix[h][w] == toi('a') {

                start_point = Point(h, w);
                // println!("Starting point: {:?}", start_point);
                let route = Rc::new(Cons(Rc::new(Node {
                    point: Rc::new(Point(start_point.0, start_point.1)),
                    elevation: matrix[start_point.0][start_point.1],
                    length: 0
                }), Rc::new(Nil)));
                clean_visited(&mut visited, &matrix, height, width);

                let result = next_steps(Rc::clone(&route), &matrix, &mut visited, false);

                if let Some(r) = result {
                    results.push(r);
                }

            }
            if matrix[h][w] == END {
                _end_point = Point(h, w);
            }
        }
    }

    println!("Part two {:?}", results.iter().min());





}

fn clean_visited(mut visited: &mut Vec<Vec<i32>>, matrix: &Vec<Vec<i16>>, height: usize, width: usize) {
    for h in 0..height {
        visited[h] = Vec::new();
        for w in 0..width {
            visited[h].push(0);
            // if matrix[h][w] == START {
            //     start_point = Point(h, w);
            // }
            // if matrix[h][w] == END {
            //     _end_point = Point(h, w);
            // }
        }
    }
}


// fn next_steps(route: Rc<Route>, matrix: &Vec<Vec<i16>>, visited: &Vec<Vec<i32>>, now: &Instant, next_nodes_map: Rc<HashMap<i32, Rc<Node>>>) -> Option<i16> {
fn next_steps(route: Rc<Route>, matrix: &Vec<Vec<i16>>, mut visited: &mut Vec<Vec<i32>>, part_1: bool) -> Option<i16> {
    if let Some(head) = Route::head(Rc::clone(&route)) {



        // if (tmp_now.elapsed() - now.elapsed() > std::time::Duration::from_millis(5000)) {
        //     new_now = tmp_now;
        // }



        // let sum_visited: i32 = visited.into_iter().map(|v| v.into_iter().sum::<i32>()).sum();
        //
        // if (10 > sum_visited) {
        //     println!("visited: {:?}", visited);
        // }
        //
        // println!("Now: {:?}, sum_visited: {}", head, sum_visited);

        let current_elevation = matrix[head.point.0][head.point.1];

        // println!("point: [{}, {}], visited: {}, elevation: {}, length: {}", head.point.0, head.point.1, how_many_visited, current_elevation, head.length);

        let next_points: Vec<(usize, usize)> = vec!(
            (head.point.0 as i32, head.point.1 as i32 + 1),
            (head.point.0 as i32 + 1, head.point.1 as i32),
            (head.point.0 as i32, head.point.1 as i32 - 1),
            (head.point.0 as i32 - 1, head.point.1 as i32),
        ).into_iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x < matrix.len() as i32 && *y < matrix[0].len() as i32).map(|(x, y)| (x as usize, y as usize)).collect();

        let mut next_nodes = next_points.into_iter().filter(|(x, y)| {

            let next_elevation = matrix[*x][*y];
            // let cond = !visited[*x][*y] && (current_elevation == START || (next_elevation == END && current_elevation == toi('z')) || (next_elevation != END && next_elevation <= current_elevation + 1));


            let cond_part_1 = current_elevation == START || (next_elevation == END && (current_elevation == toi('z') || current_elevation == toi('y'))) || (next_elevation != END && next_elevation <= current_elevation + 1);



            let cond_part_2 = (next_elevation == END && (current_elevation == toi('z') || current_elevation == toi('y'))) || (next_elevation != END && next_elevation <= current_elevation + 1);

            // if !part_1 {
                // println!("current_elevation: {}, next_elevation: {}, cond: {}", current_elevation, next_elevation, ((part_1 && (cond_part_1 && next_elevation != START)) || (!part_1 && cond_part_2)));
            // }

            (part_1 && (cond_part_1 && next_elevation != START)) || (!part_1 && cond_part_2 && next_elevation != START && (current_elevation != toi('a') || (current_elevation == toi('a') && next_elevation == toi('b'))))
        } ).map(|(x, y)| Rc::new(Node {
            point: Rc::new(Point(x, y)),
            elevation: matrix[x][y],
            length: head.length + 1,
        })).collect::<Vec<Rc<Node>>>();


        next_nodes.sort_by(|n1, n2| n2.elevation.cmp(&n1.elevation));

        // if !part_1 {
            // println!("node: {:?}, next nodes: {:?}", head, next_nodes);
        // }


        // let mut sum_visited: i32 = 0;
        // let mut new_visited = Vec::new();
        // for x in 0..visited.len() {
        //     new_visited.push(Vec::new());
        //     for y in 0..visited[x].len() {
        //         new_visited[x].push(visited[x][y]);
        //         sum_visited += new_visited[x][y];
        //     }
        // }



        // let mut new_now  = *now;
        // let tmp_now = Instant::now();
        // if tmp_now.duration_since(*now).as_millis() > 1000 {
        //     println!("Now: {:?}, branch: {}, next_nodes: {:?}", head, branch, next_nodes);
        //     new_now = tmp_now;
        // }

        let mut found_route: Option<i16> = None;

        // let mut count_branch = 0;



        for next_node in next_nodes {
            // if !part_1 {
            //     println!("(1) next node: {:?}", next_node);
            //     println!("visited[next_node.point.0][next_node.point.1] == 0: {:?}", visited[next_node.point.0][next_node.point.1] == 0);
            //     println!("visited[next_node.point.0][next_node.point.1]: {:?}", visited[next_node.point.0][next_node.point.1]);
            //     println!("visited[head.point.0][head.point.1] + 1 < visited[next_node.point.0][next_node.point.1]: {:?}", visited[head.point.0][head.point.1] + 1 < visited[next_node.point.0][next_node.point.1]);
            //     println!("visited[head.point.0][head.point.1] + 1: {:?}", visited[head.point.0][head.point.1] + 1);
            //     println!("visited[next_node.point.0][next_node.point.1]: {:?}", visited[next_node.point.0][next_node.point.1]);
            // }
            if next_node.elevation == END {
                // println!("FOUND: {}, next node: {:?}, current node: {:?}", next_node.length, next_node, head);
                // Route::scan(Rc::new(Cons(Rc::clone(&next_node), Rc::clone(&route))), 0);
                // if !part_1 {
                //     Route::to_str(Rc::new(Cons(Rc::clone(&next_node), Rc::clone(&route))));
                // }
                return Some(next_node.length);

            } else {

                if visited[next_node.point.0][next_node.point.1] == 0 || visited[head.point.0][head.point.1] + 1 < visited[next_node.point.0][next_node.point.1] {

                    // if !part_1 {
                    //     println!("(2) next node: {:?}", next_node);
                    // }


                    visited[next_node.point.0][next_node.point.1] = visited[head.point.0][head.point.1] + 1;

                    if let Some(found) = next_steps(Rc::new(Cons(Rc::clone(&next_node), Rc::clone(&route))), &matrix, &mut visited, part_1) {
                        match found_route {
                            Some(previous) => found_route = Some(min(found, previous)),
                            None => found_route = Some(found),
                        }
                    }
                } else {
                    // println!("Not going there: {:?}", next_node);
                }

            }
        }

        return found_route;
    } else {
        panic!("Route empty")
    }

}

#[derive(Debug)]
enum Route {
    Cons(Rc<Node>, Rc<Route>),
    Nil,
}

impl Route {

    fn to_str_priv(route: Rc<Route>, mut matrix: Vec<Vec<char>>, prev_x: usize, prev_y: usize) -> () {
        match &*route {
            Cons(node, tail) => {
                let mut _c = ' ';
                let mut c = ' ';
                if node.point.0 > prev_x {
                    _c = '^';
                    c = matrix[node.point.0][node.point.1].to_ascii_uppercase();
                }
                if node.point.0 < prev_x {
                    _c = 'v';
                    c = matrix[node.point.0][node.point.1].to_ascii_uppercase();
                }
                if node.point.1 > prev_y {
                    _c = '<';
                    c = matrix[node.point.0][node.point.1].to_ascii_uppercase();
                }
                if node.point.1 < prev_y {
                    _c = '>';
                    c = matrix[node.point.0][node.point.1].to_ascii_uppercase();
                }
                matrix[node.point.0][node.point.1] = c;
                Route::to_str_priv(Rc::clone(tail), matrix, node.point.0, node.point.1);
            },
            Nil => {
                matrix[prev_x][prev_y] = 'S';
                println!("---------------------------------------------");
                for i in 0..matrix.len() {
                    for j in 0..matrix[i].len() {
                        print!("{}", matrix[i][j])
                    }
                    println!("");
                }
                println!("---------------------------------------------");
            },
        }
    }

    fn to_str(route: Rc<Route>) -> () {

        let mut matrix = read_whole_file("input.txt").split("\n").filter(|s| s.trim().len() > 0).map(|s| s.split("").map(|s| s.chars().next()).filter(|o| o.is_some()).map(|o| o.unwrap()).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        // let mut matrix: Vec<Vec<char>> = Vec::new();
        // for i in 0..41 {
        //     matrix.push(Vec::new());
        //     for j in 0..81 {
        //         matrix[i].push('.');
        //     }
        // }

        match &*route {
            Cons(node, tail) => {
                matrix[node.point.0][node.point.1] = 'E';
                Route::to_str_priv(Rc::clone(tail), matrix, node.point.0, node.point.1);
            },
            Nil => (),
        }
    }

    #[allow(dead_code)]
    fn scan(route: Rc<Route>, len: usize) -> () {
        match &*route {
            Cons(node, tail) => {
                print!("[{},{}], ", node.point.0, node.point.1);
                Route::scan(Rc::clone(tail), len + 1);
            },
            Nil => println!("END, len: {}", len),
        }
    }

    #[allow(dead_code)]
    fn len(route: Rc<Route>) -> i16 {
        match &*route {
            Cons(node, _tail) => node.length,
            Nil => 0
        }
    }

    fn head(route: Rc<Route>) -> Option<Rc<Node>> {
        match &*route {
            Cons(node, _tail) => Some(Rc::clone(node)),
            Nil => None
        }
    }
}


// fn cons(point: Rc<Point>, step_number: i32, tail: Rc<Route>) -> Route {
//     // Cons { point: point, step_number: step_number, tail: tail }
//     Cons(point, step_number, tail)
// }

#[derive(Debug)]
struct Node {
    point: Rc<Point>,
    elevation: i16,
    length: i16,
}


#[derive(Debug)]
struct Point (usize, usize);

impl Point {
//     fn new(x: i16, y: i16, elevation: i16) -> Point {
//         Point { x, y }
//     }
    fn unknown() -> Point {
        Point(9999, 9999)
    }
}

fn read_whole_file(file_name: &str) -> String {
    if let Ok(content) = fs::read_to_string(file_name) {
        return content;
    } else {
        panic!("Error reading {}", file_name);
    }
}

fn toi(c: char) -> i16 {
    c as i16 - 96
}

// fn toc(i: i16) -> char {
//     (i + 96) as char
// }
use std::cmp::{min, Ordering};
use std::rc::Rc;
use aoctools::parse;

fn main() {

    let pairs: Vec<Vec<String>> = aoctools::read_whole_file("input.txt").split("\n\n").filter(|p| !p.trim().is_empty()).map(|p| p.trim().split("\n").map(|s| s.to_string()).collect()).collect();
    let mut sum = 0;
    let mut all = vec!["[[2]]", "[[6]]"];
    for i in 0..pairs.len() {
        let left = &pairs[i][0];
        let right = &pairs[i][1];
        all.push(pairs[i][0].as_str());
        all.push(pairs[i][1].as_str());
        let cmp = compare_lists(left, right);
        if cmp == -1 {
            sum += i + 1;
        }
    }
    println!("Part one: {}", sum);

    all.sort_by(|a, b| cmp(a,b));

    let mut c = 1;
    let mut answer = 1;
    for e in all {
        if e == "[[2]]" || e == "[[6]]" {
            answer *= c;
        }
        c += 1;
    }

    println!("Part two: {}", answer);
}

fn cmp(l1: &str, l2: &str) -> Ordering {
    if compare_lists(l1, l2) < 0 {
        Ordering::Less
    } else if compare_lists(l1, l2) > 0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn compare_nums(n1: &str, n2: &str) -> i32 {
    let p1 = parse(n1);
    let p2 = parse(n2);
    if p1 < p2 {
        return -1;
    } else if p1 > p2 {
        return 1;
    } else {
        return 0;
    }
}

fn compare_lists(l1: &str, l2: &str) -> i32 {
    let split_1 = split_list(l1);
    let split_2 = split_list(l2);
    for i in 0..min(split_1.len(), split_2.len()) {
        if is_list(&split_1[i]) && is_list(&split_2[i]) {
            let cmp = compare_lists(&split_1[i], &split_2[i]);
            if cmp != 0 {
                return cmp;
            }
        } else if is_number(&split_1[i]) && is_list(&split_2[i]) {
            let mut new_list = String::from("[");
            new_list.push_str(&split_1[i]);
            new_list.push_str("]");
            let cmp = compare_lists(&new_list, &split_2[i]);
            if cmp != 0 {
                return cmp;
            }
        } else if is_list(&split_1[i]) && is_number(&split_2[i]) {
            let mut new_list = String::from("[");
            new_list.push_str(&split_2[i]);
            new_list.push_str("]");
            let cmp = compare_lists(&split_1[i], &new_list);
            if cmp != 0 {
                return cmp;
            }
        } else {
            let cmp = compare_nums(&split_1[i], &split_2[i]);
            if cmp != 0 {
                return cmp;
            }
        }
    }
    if split_1.len() < split_2.len() {
        return -1;
    } else if split_1.len() > split_2.len() {
        return 1;
    } else {
        return 0;
    }
}

fn is_list(s: &str) -> bool {
    s.starts_with('[')
}

fn is_number(s: &str) -> bool {
    s.chars().fold(true, |acc, x| acc && x.is_numeric())
}

fn split_list(l: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut tmp = String::from("");
    let mut started = false;
    let mut sublist_level = 0;
    for c in l.chars() {
        if c == '[' {
            if started {
                sublist_level += 1;
                tmp.push(c);
            } else {
                started = true;
            }
        } else if c == ']' {
            if sublist_level > 0 {
                sublist_level -= 1;
                tmp.push(c);
            } else {
                started = false;
                if !tmp.is_empty() {
                    res.push(tmp);
                }
                tmp = String::from("");
            }
        } else if c.is_numeric() {
            tmp.push(c);
        } else if c == ',' {
            if sublist_level > 0 {
                tmp.push(c)
            } else {
                res.push(tmp);
                tmp = String::from("");
            }
        } else {
            panic!("No idea")
        }
    }
    res
}
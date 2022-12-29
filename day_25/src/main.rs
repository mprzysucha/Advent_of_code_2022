// #[macro_use]
// extern crate lazy_static;

use std::cmp::max;
use aoctools::read_whole_file;
use lazy_static::lazy_static;
use std::collections::HashMap;

fn main() {
    let content = read_whole_file("input.txt");
    let lines: Vec<&str> = content.split("\n").filter(|s| !s.trim().is_empty()).collect();
    let mut sum = 0;
    let mut the_longest_line = 0;
    for line in lines {
        let (dec, length) = snafu_to_dec(line.to_string());
        sum += dec;
        if length > the_longest_line {
            the_longest_line = length;
        }
    }

    let mut only_twos_in_dec = vec![2];
    for i in 2..=max(the_longest_line, 20) {
        only_twos_in_dec.push(2 * POWERS_OF_5[i - 1] + only_twos_in_dec[i - 2]);
    }
    check(&only_twos_in_dec);



    println!("Part one: {}", dec_to_snafu(sum, &only_twos_in_dec));

}

fn check(only_twos_in_dec: &Vec<i128>) {
    assert!("1" == dec_to_snafu(1, &only_twos_in_dec));
    assert!("2" == dec_to_snafu(2, &only_twos_in_dec));
    assert!("1=" == dec_to_snafu(3, &only_twos_in_dec));
    assert!("1-" == dec_to_snafu(4, &only_twos_in_dec));
    assert!("10" == dec_to_snafu(5, &only_twos_in_dec));
    assert!("11" == dec_to_snafu(6, &only_twos_in_dec));
    assert!("12" == dec_to_snafu(7, &only_twos_in_dec));
    assert!("2=" == dec_to_snafu(8, &only_twos_in_dec));
    assert!("2-" == dec_to_snafu(9, &only_twos_in_dec));
    assert!("20" == dec_to_snafu(10, &only_twos_in_dec));
    assert!("1=0" == dec_to_snafu(15, &only_twos_in_dec));
    assert!("1-0" == dec_to_snafu(20, &only_twos_in_dec));
    assert!("1=11-2" == dec_to_snafu(2022, &only_twos_in_dec));
    assert!("1-0---0" == dec_to_snafu(12345, &only_twos_in_dec));
    assert!("1121-1110-1=0" == dec_to_snafu(314159265, &only_twos_in_dec));

    assert!("2=-1=0" == dec_to_snafu(4890, &only_twos_in_dec));

    assert!("10000" == dec_to_snafu(625, &only_twos_in_dec));
    assert!("10001" == dec_to_snafu(626, &only_twos_in_dec));
    assert!("1000" == dec_to_snafu(125, &only_twos_in_dec));
    assert!("1-2" == dec_to_snafu(22, &only_twos_in_dec));
    assert!("11-2" == dec_to_snafu(147, &only_twos_in_dec));
    assert!("2--1=" == dec_to_snafu(1103, &only_twos_in_dec));
}



fn snafu_to_dec_char(c: char) -> i32 {
    for i in 0..=5 {
        if c == SNAFU_CHARS[i] {
            return i as i32 - 2;
        }
    }
    panic!("incorrect character {}", c);
}

fn snafu_to_dec(s: String) -> (i128, usize) {
    let chars: Vec<char> = s.chars().into_iter().rev().collect();
    let mut sum = 0;
    for exponent in 0..chars.len() {
        sum += POWERS_OF_5[exponent] * snafu_to_dec_char(chars[exponent]) as i128
    }
    (sum, chars.len())
}

fn dec_to_snafu(d: i128, only_twos_in_dec: &Vec<i128>) -> String {

    fn convert_remaining(remain: i128, exp: usize, only_twos_in_dec: &Vec<i128>) -> String {
        let mut res = String::from("");
        let minus_remain_in_snafu = dec_to_snafu(remain.abs(), only_twos_in_dec);
        let num_of_chars = minus_remain_in_snafu.chars().into_iter().collect::<Vec<char>>().len();
        for _ in 0..exp - num_of_chars {
            res.push('0');
        }
        for c in minus_remain_in_snafu.chars() {
            if remain > 0 {
                res.push(c);
            } else {
                res.push(*NEGATIVE_CHARS.get(&c).unwrap());
            }
        }
        res
    }

    if d >= -2 && d <= 2 {
        if d == -2 {
            return "=".to_string();
        } else if d == -1 {
            return "-".to_string();
        } else {
            return d.to_string();
        }
    }
    let mut exp = 0;
    while only_twos_in_dec[exp] <= d  {
        exp += 1;
    }
    if d == only_twos_in_dec[exp - 1] {
        let mut res = "".to_string();
        for _ in 0..exp {
            res.push_str("2");
        }
        return res;
    }

    if d > POWERS_OF_5[exp] + only_twos_in_dec[exp - 1] { // d > 1222, should return 2_____ with negative in underscore (need to subtract from 20000)
        let mut res = "2".to_string();
        res.push_str(convert_remaining(d - 2 * POWERS_OF_5[exp], exp, &only_twos_in_dec).as_str());
        res

    } else if d == POWERS_OF_5[exp] + only_twos_in_dec[exp - 1] { // d == 1222
        let mut res = "1".to_string();
        for _ in 0..exp {
            res.push_str("2");
        }
        res
    } else { // 222 < d < 1222, should return 1____
        let mut res = "1".to_string();
        res.push_str(convert_remaining(d - POWERS_OF_5[exp], exp, &only_twos_in_dec).as_str());
        res
    }
}


// fn negative(c: char) -> char {
//     if c == '2' {
//         return '=';
//     } else if c == '1' {
//         return '-';
//     } else if c == '0' {
//         return '0';
//     } else if c == '-' {
//         return '1';
//     } else if c == '=' {
//         return '2';
//     } else {
//         panic!("Not know negative for {}", c);
//     }
// }

const POWERS_OF_5: [i128; 51] = [1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625, 1220703125, 6103515625, 30517578125, 152587890625, 762939453125, 3814697265625, 19073486328125, 95367431640625, 476837158203125, 2384185791015625, 11920928955078125, 59604644775390625, 298023223876953125, 1490116119384765625, 7450580596923828125, 37252902984619140625, 186264514923095703125, 931322574615478515625, 4656612873077392578125, 23283064365386962890625, 116415321826934814453125, 582076609134674072265625, 2910383045673370361328125, 14551915228366851806640625, 72759576141834259033203125, 363797880709171295166015625, 1818989403545856475830078125, 9094947017729282379150390625, 45474735088646411895751953125, 227373675443232059478759765625, 1136868377216160297393798828125, 5684341886080801486968994140625, 28421709430404007434844970703125, 142108547152020037174224853515625, 710542735760100185871124267578125, 3552713678800500929355621337890625, 17763568394002504646778106689453125, 88817841970012523233890533447265625];
const SNAFU_CHARS: [char; 5] = ['=', '-', '0', '1', '2'];


lazy_static! {
    static ref NEGATIVE_CHARS: HashMap<char, char> = {
        let mut map = HashMap::new();
        map.insert('2', '=');
        map.insert('1', '-');
        map.insert('0', '0');
        map.insert('-', '1');
        map.insert('=', '2');
        map
    };
}
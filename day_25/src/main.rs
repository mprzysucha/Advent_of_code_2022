use std::collections::HashMap;
use aoctools::read_whole_file;

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
    for i in 2..=the_longest_line {
        only_twos_in_dec.push(2 * powers_of_5[i - 1] + only_twos_in_dec[i - 2]);
    }
    println!("length: {}", the_longest_line);
    println!("sum: {}", sum);
    println!("only_twos_in_dec: {:?}", only_twos_in_dec);




    // println!("{} {}", sum, dec_to_snafu(sum, &only_twos_in_dec));
    // println!("{} {}", 312, dec_to_snafu(312, &only_twos_in_dec));
    println!("{} -> {}", 1, dec_to_snafu(1, &only_twos_in_dec));
    println!("{} -> {}", 2, dec_to_snafu(2, &only_twos_in_dec));
    println!("{} -> {}", 4, dec_to_snafu(2, &only_twos_in_dec));
    println!("{} -> {}", 7, dec_to_snafu(7, &only_twos_in_dec));
    println!("{} -> {}", 3, dec_to_snafu(3, &only_twos_in_dec));

}



fn snafu_to_dec_char(c: char) -> i32 {
    for i in 0..=5 {
        if c == snafu_chars[i] {
            return i as i32 - 2;
        }
    }
    panic!("incorrect character {}", c);
}

fn snafu_to_dec(s: String) -> (i128, usize) {
    let chars: Vec<char> = s.chars().into_iter().rev().collect();
    let mut sum = 0;
    for exponent in 0..chars.len() {
        sum += powers_of_5[exponent] * snafu_to_dec_char(chars[exponent]) as i128
    }
    (sum, chars.len())
}

fn dec_to_snafu(d: i128, only_twos_in_dec: &Vec<i128>) -> String {
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
        println!("only_twos_in_dec[{}] < d\t{} < {}", exp, only_twos_in_dec[exp], d);
        exp += 1;
    }
    println!("exp {} only_twos_in_dec[exp] {}", exp, only_twos_in_dec[exp]);
    if d == only_twos_in_dec[exp - 1] {
        let mut res = "".to_string();
        for _ in 0..exp {
            res.push_str("2");
        }
        return res;
    }
    // between 312 and 1562

    // if greater than 2000

    // println!("powers_of_5[exp - 1]: {}", powers_of_5[exp - 1]);
    // println!("only_twos_in_dec[exp - 2]: {}", only_twos_in_dec[exp - 2]);

    if d > powers_of_5[exp] + only_twos_in_dec[exp - 1] {
        let mut res = "2".to_string();
        // return 2_____ (trzeba odjac od 20000)
    } else if d == powers_of_5[exp] + only_twos_in_dec[exp - 1] {
        let mut res = "1".to_string();
        for _ in 0..exp {
            res.push_str("2");
        }
        return res;
    } else {
        let mut res = "1".to_string();
        println!("powers_of_5[exp] + only_twos_in_dec[exp - 1] = {}", powers_of_5[exp] + only_twos_in_dec[exp - 1]);
        let remain = powers_of_5[exp] - d;
        let minus_remain_in_snafu = dec_to_snafu(-remain, only_twos_in_dec);
        for c in minus_remain_in_snafu.chars() {
            res.push(negative(c));
        }
        println!("remain: {}", remain);
        println!("res: {}", res);
        return res;
    }

    // else if greater than 1000
    let mut res = "1".to_string();

    println!("{}", d);

    println!("{}", powers_of_5[exp - 1]);

    // let remain_in_snafu = dec_to_snafu(remain, only_twos_in_dec);

    // res.push_str(remain_in_snafu.as_str());



    res
}


fn negative(c: char) -> char {
    if c == '2' {
        return '=';
    } else if c == '1' {
        return '-';
    } else if c == '0' {
        return '0';
    } else if c == '-' {
        return '1';
    } else if c == '=' {
        return '2';
    } else {
        panic!("Not know negative for {}", c);
    }
}

const powers_of_5: [i128; 51] = [1, 5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625, 1220703125, 6103515625, 30517578125, 152587890625, 762939453125, 3814697265625, 19073486328125, 95367431640625, 476837158203125, 2384185791015625, 11920928955078125, 59604644775390625, 298023223876953125, 1490116119384765625, 7450580596923828125, 37252902984619140625, 186264514923095703125, 931322574615478515625, 4656612873077392578125, 23283064365386962890625, 116415321826934814453125, 582076609134674072265625, 2910383045673370361328125, 14551915228366851806640625, 72759576141834259033203125, 363797880709171295166015625, 1818989403545856475830078125, 9094947017729282379150390625, 45474735088646411895751953125, 227373675443232059478759765625, 1136868377216160297393798828125, 5684341886080801486968994140625, 28421709430404007434844970703125, 142108547152020037174224853515625, 710542735760100185871124267578125, 3552713678800500929355621337890625, 17763568394002504646778106689453125, 88817841970012523233890533447265625];
const snafu_chars: [char; 5] = ['=', '-', '0', '1', '2'];

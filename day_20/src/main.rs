use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ptr::eq;
use std::rc::{Rc, Weak};
use aoctools::{parse_with_neg, read_file, read_whole_file};

fn main() -> std::io::Result<()> {
    day20::main();
    Ok(())
}


mod day20 {
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{Ref, RefCell};
    use std::rc::{Rc, Weak};
    use aoctools::{parse_with_neg, read_whole_file};
    use crate::day20::List::{Cons, Nil};

    pub fn main() {

        println!("Part one: {}", main2(1, 1));
        println!("Part two: {}", main2(811589153, 10));

    }

    pub fn main2(multiplier: i128, number_of_rounds: usize) -> i128 {

        let mut numbers: Vec<i128> = read_whole_file("input.txt").split("\n").filter(|s| !s.trim().is_empty()).map(|s| parse_with_neg(s) as i128 * multiplier).collect();


        let root = Rc::new(Cons(numbers[0],RefCell::new(Weak::new()), RefCell::new(Weak::new()), RefCell::new(Rc::new(Nil))));
        let mut current = Rc::clone(&root);

        /* build the structure */
        for i in 1..numbers.len() {
            let mut element = Rc::new(Cons(numbers[i],RefCell::new(Weak::new()), RefCell::new(Weak::new()), RefCell::new(Rc::new(Nil))));
            if let Some(link) = element.previous() {
                *link.borrow_mut() = Rc::downgrade(&current);
            }
            if let Some(link) = current.tail_decrypted() {
                *link.borrow_mut() = Rc::downgrade(&element);
            }
            if let Some(link) = current.tail_original() {
                *link.borrow_mut() = Rc::clone(&element);
            }
            current = element;
        }
        if let Some(link) = root.previous() {
            *link.borrow_mut() = Rc::downgrade(&current);
        }
        if let Some(link) = current.tail_decrypted() {
            *link.borrow_mut() = Rc::downgrade(&root);
        }

        let mut number_of_numbers = numbers.len() as i32;
        let modulo = (number_of_numbers as i128 - 1);
        for round in 0..number_of_rounds {
            let mut current = Rc::clone(&root);
            for i in 0..number_of_numbers {

                let v = current.value().unwrap();

                if v < 0 {
                    current.move_left_n(v.abs() % modulo);
                }
                if v > 0 {
                    current.move_right_n(v.abs() % modulo)
                }
                let next = Rc::clone(&current.tail_original().unwrap().borrow());
                current = next;
            }
        }

        let mut zero = Rc::clone(&root);
        for _ in 0..number_of_numbers {
            if zero.value().unwrap() == 0 {
                break;
            }
            let next = Rc::clone(&zero.tail_decrypted().unwrap().borrow().upgrade().unwrap());
            zero = next;
        }

        let mut n1000 = Rc::clone(&zero);
        for _ in 0..1000 {
            let next = Rc::clone(&n1000.tail_decrypted().unwrap().borrow().upgrade().unwrap());
            n1000 = next;
        }

        let mut n2000 = Rc::clone(&n1000);
        for _ in 0..1000 {
            let next = Rc::clone(&n2000.tail_decrypted().unwrap().borrow().upgrade().unwrap());
            n2000 = next;
        }

        let mut n3000 = Rc::clone(&n2000);
        for _ in 0..1000 {
            let next = Rc::clone(&n3000.tail_decrypted().unwrap().borrow().upgrade().unwrap());
            n3000 = next;
        }

        // println!("n1000: {}", n1000.value().unwrap());
        // println!("n2000: {}", n2000.value().unwrap());
        // println!("n3000: {}", n3000.value().unwrap());

        n1000.value().unwrap() + n2000.value().unwrap() + n3000.value().unwrap()

    }

    #[derive(Debug)]
    pub enum List {
        Cons(i128, RefCell<Weak<List>>, RefCell<Weak<List>>, RefCell<Rc<List>>),
        Nil
    }

    impl List {

        fn next_ref(&self) -> Rc<List> {
            Rc::clone(&self.tail_decrypted().unwrap().borrow().upgrade().unwrap())
        }

        fn prev_ref(&self) -> Rc<List> {
            Rc::clone(&self.previous().unwrap().borrow().upgrade().unwrap())
        }

        fn previous(&self) -> Option<&RefCell<Weak<List>>> {
            match &*self {
                Cons(_, ref item, _, _) => Some(item),
                Nil => None,
            }
        }

        fn tail_decrypted(&self) -> Option<&RefCell<Weak<List>>> {
            match &*self {
                Cons(_, _, ref item, _) => Some(item),
                Nil => None,
            }
        }

        fn tail_original(&self) -> Option<&RefCell<Rc<List>>> {
            match &*self {
                Cons(_, _, _, ref item) => Some(item),
                Nil => None
            }
        }

        fn num(&self) -> i128 {
            self.value().unwrap()
        }

        fn value(&self) -> Option<i128> {
            match &*self {
                Cons(n, _, _, _) => Some(*n),
                Nil => None,
            }
        }

        fn to_string(&self) -> String {
            match &*self {
                Cons(n, _, _, _) => n.to_string(),
                Nil => "".to_string(),
            }
        }

        fn as_string_n(&self, n: i32) -> String {

            fn as_string_from(link: &List, c: i32) -> String {
                let mut s = link.to_string();
                if c > 1 {
                    s.push_str(", ");
                    s.push_str(as_string_from(link.tail_decrypted().unwrap().borrow().upgrade().unwrap().as_ref(), c - 1).as_str());
                }
                s
            }
            as_string_from(self, n)
        }

        fn as_string_n_rev(&self, n: i32) -> String {

            fn as_string_from(link: &List, c: i32) -> String {
                let mut s = link.to_string();
                if c > 1 {
                    s.push_str(", ");
                    s.push_str(as_string_from(link.previous().unwrap().borrow().upgrade().unwrap().as_ref(), c - 1).as_str());
                }
                s
            }
            as_string_from(self, n)
        }

        // move_left(E, n = 3)
        // before: A -> B -> C -> D -> E -> F
        // after:  A -> E -> B -> C -> D -> F
        // fn move_left_n(&self, n: i128, debug: bool) -> () {
        fn move_left_n(&self, n: i128) -> () {

            if n <= 0 {
                return;
            }


            let d = self.previous().unwrap().borrow().upgrade().unwrap();
            let e = d.tail_decrypted().unwrap().borrow().upgrade().unwrap();
            let f = e.tail_decrypted().unwrap().borrow().upgrade().unwrap();


            let mut a = Rc::clone(&d);
            let mut petla = 0;

            for _ in 0..n {

                let prev = a.previous().unwrap().borrow().upgrade().unwrap();
                a = prev;
            }

            let b = a.tail_decrypted().unwrap().borrow().upgrade().unwrap();

            *a.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&e);
            *e.previous().unwrap().borrow_mut() = Rc::downgrade(&a);
            *e.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&b);
            *b.previous().unwrap().borrow_mut() = Rc::downgrade(&e);

            *d.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&f);
            *f.previous().unwrap().borrow_mut() = Rc::downgrade(&d);


        }

        // move_roght(B, n = 3)
        // before: A -> B -> C -> D -> E -> F
        // after:  A -> C -> D -> E -> B -> F
        fn move_right_n(&self, n: i128) -> () {
            if n <= 0 {
                return;
            }
            let a = self.previous().unwrap().borrow().upgrade().unwrap();
            let b = a.tail_decrypted().unwrap().borrow().upgrade().unwrap();
            let c = b.tail_decrypted().unwrap().borrow().upgrade().unwrap();

            let mut f = Rc::clone(&c);
            for _ in 0..n {
                let prev = f.tail_decrypted().unwrap().borrow().upgrade().unwrap();
                f = prev;
            }
            let e = f.previous().unwrap().borrow().upgrade().unwrap();

            *a.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&c);
            *c.previous().unwrap().borrow_mut() = Rc::downgrade(&a);
            *e.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&b);
            *b.previous().unwrap().borrow_mut() = Rc::downgrade(&e);

            *b.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&f);
            *f.previous().unwrap().borrow_mut() = Rc::downgrade(&b);

        }


        // move_left(C)
        // before: A -> B -> C -> D
        // after:  A -> C -> B -> D
        fn move_left(&self) -> () {
            let b = self.previous().unwrap().borrow().upgrade().unwrap();
            let c = b.tail_decrypted().unwrap().borrow().upgrade().unwrap();
            let a = b.previous().unwrap().borrow().upgrade().unwrap();
            let d = self.tail_decrypted().unwrap().borrow().upgrade().unwrap();

            *a.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&c);
            *c.previous().unwrap().borrow_mut() = Rc::downgrade(&a);
            *c.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&b);
            *b.previous().unwrap().borrow_mut() = Rc::downgrade(&c);
            *b.tail_decrypted().unwrap().borrow_mut() = Rc::downgrade(&d);
            *d.previous().unwrap().borrow_mut() = Rc::downgrade(&b);
        }

        // move_right(B)
        // before: A -> B -> C -> D
        // after:  A -> C -> B -> D
        fn move_right(&self) -> () {
            let c = self.tail_decrypted().unwrap().borrow().upgrade().unwrap();
            c.move_left();
        }



        fn original_as_string(&self) -> String {

            fn tail_to_string(from: &List, link: &RefCell<Rc<List>>) -> String {
                if link.borrow().tail_original().is_none() {
                    return "".to_string();
                } else {
                    let mut s = link.borrow().to_string();
                    let s2 = tail_to_string(from, link.borrow().tail_original().unwrap());
                    if s2 != "" {
                        s.push_str(", ");
                        s.push_str(s2.as_str());
                    }
                    s
                }

            }

            match &*self {
                Cons(n, _, t, t2) => {
                    let mut s = "[".to_string();
                    s.push_str(n.to_string().as_str());
                    let s2 = tail_to_string(self, t2);
                    if s2 != "" {
                        s.push_str(", ");
                        s.push_str(s2.as_str());
                    }
                    s.push_str("]");
                    s
                },
                Nil => "[]".to_string(),
            }
        }


    }

}

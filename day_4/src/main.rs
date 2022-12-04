use aoctools::read_file;

fn main() {
    let mut fully_contains_ranges = 0;
    let mut overlap_ranges = 0;
    for line_res in read_file("input.txt") {
        match line_res {
            Ok(line) => {
                let (r1, r2) = create_ranges(&line);
                if r1.fully_contains(&r2) || r2.fully_contains(&r1) {
                    fully_contains_ranges += 1;
                }
                if r1.overlap(&r2) {
                    overlap_ranges += 1;
                }
            },
            Err(e) => panic!("Problem when reading a file: {:?}", e),
        }
    }
    println!("Part one: {}", fully_contains_ranges);
    println!("Part two: {}", overlap_ranges);
}

// Part one: 569
// Part two: 936

fn create_ranges(line: &String) -> (Range, Range) {
    let split = line.split(",").flat_map(|s| s.split("-")).map(|s| parse(s)).collect::<Vec<u32>>();
    let r1 = Range::new(split[0], split[1]);
    let r2 = Range::new(split[2], split[3]);
    (r1, r2)
}

fn parse(s: &str) -> u32 {
    match s.parse::<u32>() {
        Ok(num) => num,
        Err(e) => panic!("Error parsing number: {}", e),
    }
}

struct Range {
    left: u32,
    right: u32,
}

impl Range {
    fn new(left: u32, right: u32) -> Range {
        Range {
            left: left,
            right: right,
        }
    }
    fn fully_contains(&self, that: &Range) -> bool {
        self.left <= that.left && self.right >= that.right
    }
    fn overlap(&self, that: &Range) -> bool {
        let dont_overlap = self.right < that.left || self.left > that.right;
        !dont_overlap
    }
}

#[cfg(test)]
mod tests {
    use crate::Range;

    #[test]
    fn check_fully_contains() {
        let r1 = Range {
            left: 6,
            right: 6
        };
        let r2 = Range {
            left: 4,
            right: 6
        };
        assert!(r2.fully_contains(&r1));

        let r1 = Range {
            left: 2,
            right: 8
        };
        let r2 = Range {
            left: 3,
            right: 7
        };
        assert!(r1.fully_contains(&r2));

        let r1 = Range {
            left: 2,
            right: 6
        };
        let r2 = Range {
            left: 4,
            right: 8
        };
        assert!(!r1.fully_contains(&r2));
        assert!(!r2.fully_contains(&r1));
    }

    #[test]
    fn check_overlap() {
        let r1 = Range::new(2, 4);
        let r2 = Range::new(6, 8);
        assert!(!r2.overlap(&r1));

        let r1 = Range::new(2, 3);
        let r2 = Range::new(4, 5);
        assert!(!r2.overlap(&r1));

        let r1 = Range::new(5, 7);
        let r2 = Range::new(7, 9);
        assert!(r2.overlap(&r1));

        let r1 = Range::new(2, 8);
        let r2 = Range::new(3, 7);
        assert!(r2.overlap(&r1));

        let r1 = Range::new(6, 6);
        let r2 = Range::new(4, 6);
        assert!(r2.overlap(&r1));

        let r1 = Range::new(2, 6);
        let r2 = Range::new(4, 8);
        assert!(r2.overlap(&r1));
    }

}

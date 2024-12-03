use std::io::BufRead as _;

fn p1() {
    let input = include_bytes!("input.txt");

    let num_safe: u32 = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let values = line
                .split(' ')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let asc = values[1] > values[0];

            if values.as_slice().windows(2).fold(true, |s, win| {
                s && (win[1] > win[0]) == asc && (1..=3).contains(&(win[1] - win[0]).abs())
            }) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("num safe: {num_safe}")
}

fn p2() {
    let input = include_bytes!("input.txt");

    let num_safe: u32 = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut values = line.split(' ').map(|v| v.parse::<i32>().unwrap());

            let checker = checker(&mut values);

            if checker(&mut values) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("num safe p2: {num_safe}")
}

fn checker<Tail>(iter: &mut impl Iterator<Item = i32>) -> Box<dyn Fn(&mut Tail) -> bool>
where
    Tail: Iterator<Item = i32>,
{
    let mut prev = iter.next().unwrap();
    let mut cur = iter.next().unwrap();
    let mut err_count = 0;
    let diff_ok = |cur: i32, prev: i32| (1..=3).contains(&(cur - prev).abs());
    if !diff_ok(cur, prev) {
        err_count += 1;
        let next = iter.next().unwrap();
        //check if it's the first or second that's the problem
        if diff_ok(next, prev) {
            cur = next;
        } else if diff_ok(next, cur) {
            prev = cur;
        } else {
            return Box::new(|_| false);
        }
    }
    let asc = prev < cur;

    Box::new(move |i| {
        let mut prev = cur;
        let mut errs = err_count;
        for cur in i {
            if (prev < cur) == asc && (1..=3).contains(&(cur - prev).abs()) {
                prev = cur;
            } else {
                errs += 1;
            }
            if errs > 1 {
                return false;
            }
        }
        true
    })
}

fn main() {
    p1();
    p2();
}

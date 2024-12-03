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

//ok if some, contains direction
fn check(ahead: i32, behind: i32) -> Option<bool> {
    (1..=3)
        .contains(&(ahead - behind).abs())
        .then_some(ahead > behind)
}

fn p2() {
    let input = include_bytes!("input.txt");
    let num_safe: u32 = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let values = line
                .split(' ')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let mut acc = 0;

            let mut windows = values.as_slice().windows(3);

            while let Some(v) = windows.next() {
                let behind = v[0];
                let cur = v[1];
                let next = v[2];

                print!("{behind},{cur},{next} - into ");

                match check(cur, behind) {
                    Some(asc) => match (check(next, behind), check(next, cur)) {
                        (Some(asc_nb), _) => {
                            println!("1");
                            if asc != asc_nb {
                                acc += 1;
                            }
                        }
                        (None, Some(_)) => {
                            println!("2.1");
                            //acc += 1;
                        }
                        (None, None) => {
                            println!("2.2");
                            acc += 1;
                        }
                    },
                    None => match (check(next, behind), check(next, cur)) {
                        (Some(_), _) => {
                            println!("3");
                            acc += 1;
                        }

                        (None, Some(_)) => {
                            println!("4.1");
                            acc += 2;
                            break;
                        }
                        (None, None) => {
                            println!("4.2");
                            acc += 2;
                            break;
                        }
                    },
                }

                println!("{behind},{cur},{next}, {acc}");
            }
            println!("{line}: {acc}");
            if acc < 2 {
                1
            } else {
                0
            }
        })
        .sum();

    println!("num safe p2: {num_safe}")
}
fn main() {
    p1();
    p2();
}

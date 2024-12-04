fn p1(input: &str) {
    let res: i32 = input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            let start = &input[i + 4..];
            let end = start.find(')')?;

            let (f, l) = &start[..end].split_once(',')?;

            let num1 = f.parse::<i32>().ok()?;
            let num2 = l.parse::<i32>().ok()?;

            Some(num1 * num2)
        })
        .sum();
    println!("{res}")
}

fn p2(input: &str) {
    let mut dos = input.match_indices("do()").map(|(i, _)| i + 4);
    let mut donts = input.match_indices("don't()").map(|(i, _)| i);

    let mut valid_ranges: Vec<(usize, usize)> = vec![];
    let mut end_of_last_range = donts.next().unwrap();

    valid_ranges.push((0, end_of_last_range));

    loop {
        let Some(next_do) = (loop {
            let Some(next_do) = dos.next() else {
                break None;
            };
            if next_do > end_of_last_range {
                break Some(next_do);
            }
        }) else {
            break;
        };
        let Some(next_dont) = (loop {
            let Some(next_dont) = donts.next() else {
                break None;
            };
            if next_dont > next_do {
                break Some(next_dont);
            }
        }) else {
            valid_ranges.push((next_do, input.len()));
            break;
        };

        valid_ranges.push((next_do, next_dont));
        end_of_last_range = next_dont;
    }

    let mut range_iter = valid_ranges.into_iter();
    let mut cur_range = range_iter.next().unwrap();

    let res: i32 = input
        .match_indices("mul(")
        .filter_map(|(mut i, _)| {
            i += 4;

            let start = &input[i..];
            let end = start.find(')')?;

            let (f, l) = &start[..end].split_once(',')?;

            if i > cur_range.1 {
                cur_range = range_iter.next()?;
                return None;
            }
            if i < cur_range.0 {
                return None;
            }

            let num1 = f.parse::<i32>().ok()?;
            let num2 = l.parse::<i32>().ok()?;

            Some(num1 * num2)
        })
        .sum();
    println!("{res}")
}

fn main() {
    let input = std::str::from_utf8(include_bytes!("input.txt")).unwrap();
    p1(input);
    p2(input);
}

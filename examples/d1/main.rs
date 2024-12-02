use std::{collections::HashMap, io::BufRead};

fn p1() {
    let input = include_bytes!("input.txt");

    let (mut l1, mut l2) = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (f, s) = line.split_once("   ").unwrap();
            (f.parse::<i32>().unwrap(), s.parse::<i32>().unwrap())
        })
        .collect::<(Vec<i32>, Vec<i32>)>();

    l1.sort();
    l2.sort();

    let sum = l1
        .into_iter()
        .zip(l2)
        .fold(0, |s, (i1, i2)| s + (i1 - i2).abs());

    println!("p1 {sum}");
}

fn p2() {
    let input = include_bytes!("input.txt");

    let (l1, l2) = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (f, s) = line.split_once("   ").unwrap();
            (f.parse::<i32>().unwrap(), s.parse::<i32>().unwrap())
        })
        .collect::<(Vec<i32>, Vec<i32>)>();

    let mut l2h: HashMap<i32, u32> = HashMap::new();
    for i2 in l2 {
        let v = l2h.entry(i2).or_default();
        *v += 1;
    }

    let sum: u32 = l1
        .into_iter()
        .map(|i1| {
            let amt = l2h.get(&i1).copied().unwrap_or_default();

            i1 as u32 * amt
        })
        .sum();
    println!("p2 {sum}");
}

fn main() {
    p1();
    p2();
}

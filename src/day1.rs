use std::{error::Error, fs::File, io::Read};

pub fn solve(inp: &str) -> Result<i32, Box<dyn Error>> {
    let mut f = File::open(inp)?;

    let mut left = Vec::new();
    let mut right = Vec::new();

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    buf.lines()
        .map(|line| {
            line.split("   ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|values| {
            assert!(values.len() == 2);
            left.push(values[0]);
            right.push(values[1]);
        });

    let ans = distance(&mut left, &mut right);
    Ok(ans)
}

fn distance(left: &mut [i32], right: &mut [i32]) -> i32 {
    left.sort();
    right.sort();
    left.iter().zip(right).map(|(l, r)| (*l - *r).abs()).sum()
}

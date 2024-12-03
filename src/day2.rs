use std::{error::Error, fs::File, io::Read};

pub fn solve() {
    let reports = read_input("inputs/day2.txt").unwrap();
    println!("Part 1: {}", safe_reports(&reports));
}

fn safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| is_report_safe(r)).count()
}

fn is_report_safe(report: &[i32]) -> bool {
    if report.len() == 1 {
        return true;
    }
    let dir = (report[1] - report[0]).signum();
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.signum() != dir.signum() {
            return false;
        }
        if ![1, 2, 3].contains(&diff.abs()) {
            return false;
        }
    }
    true
}

fn read_input(input_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut f = File::open(input_path)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let reports = buf
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|v| {
                    v.parse::<i32>()
                        .unwrap_or_else(|_| panic!("invalid value: {}", v))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(reports)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_safe_reports() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(safe_reports(&reports), 2);
    }
}

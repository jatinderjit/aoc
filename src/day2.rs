use std::{error::Error, fs::File, io::Read};

pub fn solve() {
    let reports = read_input("inputs/day2.txt").unwrap();
    println!("Part 1: {}", safe_reports(&reports));
    println!("Part 2: {}", safe_reports_2(&reports));
}

fn safe_reports(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| is_report_safe(r)).count()
}

fn safe_reports_2(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|r| is_safe_with_max_one_bad_level(r))
        .count()
}

fn is_report_safe(report: &[i32]) -> bool {
    if report.len() == 1 {
        return true;
    }
    let dir = (report[1] - report[0]).signum();
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.signum() != dir {
            return false;
        }
        if !is_valid_diff(diff) {
            return false;
        }
    }
    true
}

fn is_valid_diff(diff: i32) -> bool {
    [1, 2, 3].contains(&diff.abs())
}

fn is_safe_with_max_one_bad_level(report: &[i32]) -> bool {
    if report.len() <= 2 || is_report_safe(report) {
        return true;
    }
    let mut report = report.to_vec();
    // Try removing one item at a time, starting from the end.
    let mut removed = report.pop().unwrap();
    for i in (0..report.len()).rev() {
        if is_report_safe(&report) {
            return true;
        }
        std::mem::swap(&mut removed, &mut report[i]);
    }
    is_report_safe(&report)
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
            vec![7, 6, 4, 2, 1], // Safe
            vec![1, 2, 7, 8, 9], // Unsafe
            vec![9, 7, 6, 2, 1], // Unsafe
            vec![1, 3, 2, 4, 5], // Unsafe
            vec![8, 6, 4, 4, 1], // Unsafe
            vec![1, 3, 6, 7, 9], // Safe
        ];
        assert!(is_report_safe(&reports[0]));
        assert!(!is_report_safe(&reports[1]));
        assert!(!is_report_safe(&reports[2]));
        assert!(!is_report_safe(&reports[3]));
        assert!(!is_report_safe(&reports[4]));
        assert!(is_report_safe(&reports[5]));

        assert_eq!(safe_reports(&reports), 2);
    }

    #[test]
    fn test_safe_reports_2() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],  // Safe
            vec![1, 2, 7, 8, 9],  // Unsafe
            vec![9, 7, 6, 2, 1],  // Unsafe
            vec![1, 3, 2, 4, 5],  // Safe (remove "3")
            vec![8, 6, 4, 4, 1],  // Safe (remove "4")
            vec![1, 3, 6, 7, 9],  // Safe
            vec![10, 3, 6, 7, 9], // Safe (remove "10")
            vec![1, 3, 6, 7, 19], // Safe (remove "19")
            vec![4, 3, 6, 7, 19], // Unsafe
        ];
        assert!(is_safe_with_max_one_bad_level(&reports[0]));
        assert!(!is_safe_with_max_one_bad_level(&reports[1]));
        assert!(!is_safe_with_max_one_bad_level(&reports[2]));
        assert!(is_safe_with_max_one_bad_level(&reports[3]));
        assert!(is_safe_with_max_one_bad_level(&reports[4]));
        assert!(is_safe_with_max_one_bad_level(&reports[5]));
        assert!(is_safe_with_max_one_bad_level(&reports[6]));
        assert!(is_safe_with_max_one_bad_level(&reports[7]));
        assert!(!is_safe_with_max_one_bad_level(&reports[8]));

        assert_eq!(safe_reports_2(&reports), 6);
    }
}

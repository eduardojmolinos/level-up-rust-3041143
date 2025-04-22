fn weeks_between(a: &str, b: &str) -> i32 {
    let parts1: Vec<&str> = a.split('-').collect();
    let parts2: Vec<&str> = b.split('-').collect();

    let day1 = parts1[2].parse::<i32>();
    let day2 = parts2[2].parse::<i32>();
    return (day2.unwrap() - day1.unwrap()) / 7;
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}

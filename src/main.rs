use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let mut returnok = true;
    let parts: Vec<&str> = text.split('-').collect();
    if parts.len() != 3 {
        returnok = false;
    } else {
        if parts[0].len() != 4 && parts[1].len() != 2 && parts[2].len() != 2 {
            returnok = false;
        } else {
            let y: i32 = parts[0].parse().unwrap();
            let m: u32 = parts[1].parse().unwrap();
            let d: u32 = parts[2].parse().unwrap();
            return Some(NaiveDate::from_ymd(y, m, d));
        }
    }
    returnok = true;

    let parts2: Vec<&str> = text.split('/').collect();
    if parts2.len() != 3 {
        returnok = false;
    } else {
        if parts2[0].len() != 4 && parts2[1].len() != 3 && parts2[2].len() != 2 {
            returnok = false;
        } else {
            let mut m: u32 = 0;
            match parts2[1] {
                "Jan" => m = 1,
                "Feb" => m = 2,
                "Mar" => m = 3,
                "Apr" => m = 4,
                "May" => m = 5,
                "Jun" => m = 6,
                "Jul" => m = 7,
                "Aug" => m = 8,
                "Sep" => m = 9,
                "Oct" => m = 10,
                "Nov" => m = 11,
                "Dec" => m = 12,
                _ => m = 0,
            }
            if m != 0 {
                let y: i32 = parts2[0].parse().unwrap();
                let d: u32 = parts2[2].parse().unwrap();

                return Some(NaiveDate::from_ymd(y, m, d));
            } else {
                returnok = false;
            }
        }
    }

    returnok = true;

    let parts3: Vec<&str> = text.split('.').collect();
    if parts3.len() != 3 {
        returnok = false;
    } else {
        if parts3[0].len() != 2 && parts3[1].len() != 3 && parts3[2].len() != 4 {
            returnok = false;
        } else {
            let mut m: u32 = 0;
            match parts3[1] {
                "Jan" => m = 1,
                "Feb" => m = 2,
                "Mar" => m = 3,
                "Apr" => m = 4,
                "May" => m = 5,
                "Jun" => m = 6,
                "Jul" => m = 7,
                "Aug" => m = 8,
                "Sep" => m = 9,
                "Oct" => m = 10,
                "Nov" => m = 11,
                "Dec" => m = 12,
                _ => m = 0,
            }
            if m != 0 {
                let y: i32 = parts3[2].parse().unwrap();
                let d: u32 = parts3[0].parse().unwrap();

                return Some(NaiveDate::from_ymd(y, m, d));
            } else {
                returnok = false;
            }
        }
    }

    returnok = true;

    let parts4: Vec<&str> = text.split('.').collect();
    if parts4.len() != 3 {
        returnok = false;
    } else {
        if parts4[0].len() != 3 && parts4[1].len() != 2 && parts4[2].len() != 4 {
            returnok = false;
        } else {
            let mut m: u32 = 0;
            match parts4[0] {
                "Jan" => m = 1,
                "Feb" => m = 2,
                "Mar" => m = 3,
                "Apr" => m = 4,
                "May" => m = 5,
                "Jun" => m = 6,
                "Jul" => m = 7,
                "Aug" => m = 8,
                "Sep" => m = 9,
                "Oct" => m = 10,
                "Nov" => m = 11,
                "Dec" => m = 12,
                _ => m = 0,
            }
            if m != 0 {
                let y: i32 = parts3[2].parse().unwrap();
                let d: u32 = parts3[1].parse().unwrap();

                return Some(NaiveDate::from_ymd(y, m, d));
            } else {
                returnok = false;
            }
        }
    }

    if !returnok {
        return None;
    } else {
        return None;
    }
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        Some(NaiveDate::from_ymd(2010, 12, 11))
    )
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        Some(NaiveDate::from_ymd(1999, 3, 2))
    )
}

#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        Some(NaiveDate::from_ymd(2021, 3, 1))
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        Some(NaiveDate::from_ymd(2021, 4, 5))
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}

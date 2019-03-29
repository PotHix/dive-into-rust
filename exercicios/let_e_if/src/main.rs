fn main() {
    /* you may leave it blank */
}

/// A leap year happens every four years
/// except in year that are multiple of 100 and not multiple of 400.
/// Return `true` if `year` is a leap year and `false` otherwise
fn is_leap_year(year: i32) -> bool {
    false
}


// Tests

#[test]
fn test_vanilla_leap_year() {
    assert_eq!(is_leap_year(1996), true);
}

#[test]
fn test_any_old_year() {
    assert_eq!(is_leap_year(1997), false);
}

#[test]
fn test_century() {
    assert_eq!(is_leap_year(1900), false);
}

#[test]
fn test_exceptional_centuries() {
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(2400), true);
}

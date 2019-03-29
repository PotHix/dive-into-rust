fn main() {
    /* you can leave it blank*/
}

/// Given a positive number x:
///     If x is even, divide it by 2 and repeat
///     If x is odd, do x equal to 3x + 1 and repeat
/// Return how many times it's necessary to repeat the process until x = 1
fn collatz(mut x: u32) -> u32 {

}


// Testes

#[test]
fn test_0() {
    assert_eq!(collatz(0), 0);
}

#[test]
fn test_1() {
    assert_eq!(collatz(1), 0);
}

#[test]
fn test_2() {
    assert_eq!(collatz(2), 1);
}

#[test]
fn test_9() {
    assert_eq!(collatz(9), 19);
}

#[test]
fn test_97() {
    assert_eq!(collatz(97), 118);
}

#[test]
fn test_77_031() {
    assert_eq!(collatz(77_031), 350);
}

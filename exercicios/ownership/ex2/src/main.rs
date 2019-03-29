// Make the code compile
// Is the compiler right to complain about your code?
fn main() {
    let mut vector = vec![1, 3, 2];
    let mut biggest = &vector[0];
    for x in &vector {
        if x > biggest {
            biggest = x;
        }
    }
    vector.push(biggest + 1);
    println!("The biggest one is: {}", biggest);
}

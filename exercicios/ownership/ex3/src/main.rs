// Make the code compile
// Is the compiler right to to complain about your code?O compilador tem razão em reclamar do seu código?
fn main() {
    let vetor = vec![1, 2, 3, 4];
    println!("Half of it is: {:?}", half(&vetor));
}

fn half(vetor: &Vec<i32>) -> &[i32] {
    let middle = vetor.len()/2 + 1;
    let mut half = Vec::new();
    for i in 0..middle {
        half.push(vetor[i]);
    }
    &half
}

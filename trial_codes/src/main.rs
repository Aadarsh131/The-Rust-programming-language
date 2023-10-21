fn main() {
    let s1 = String::from("hello");
    removes_ownership(s1);

    let (s2, len) = calculate_length(s1);

    println!(
        "The length of '{}' is {}. and value of s1 is {}",
        s2, len, s1
    );
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
fn removes_ownership(_s: String) {
    let x = 5;
    print!("{}", x);
} //s is `drop` from the function and will never be accessible as we have not retured the ownership of s

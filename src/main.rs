fn main() {
    fizzbuzz(1, 100);
}
fn fizzbuzz(lower: usize, upper: usize) {
    for n in lower..=upper {
        let mut output: String = String::new();
        if n % 3 == 0 {
            output.push_str("fizz")
        }
        if n % 5 == 0 {
            output.push_str("buzz")
        }
        if output.is_empty() {
            output.push_str(&format!("{}", n))
        }

        println!("{}", output)
    }
}

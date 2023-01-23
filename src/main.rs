fn main() {
    let n = String::from("Hello");
    let n2 = append_str(&n);
    println!("Original {} Function value {}", n, n2);
}

fn append_str(string: &String) -> String {
    string.to_lowercase()
}

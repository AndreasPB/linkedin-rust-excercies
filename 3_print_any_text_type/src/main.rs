fn main() {
    let a: &str = "What am I doing?";
    let b: String = String::from("What am I doing? - Different string");
    let c: &String = &b;

    info(a);
    info(c);
    info(b);
}

fn info<T: AsRef<str>>(text: T) {
    println!("{:?}", text.as_ref());
}

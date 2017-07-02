// declare some globals; still not totally sure why we're
// marking LANGUAGE with `&` or `'static` or `str` instead of String
// but whatever, let's keep on rollin'

// static is a "possibly `mut`able variable with 'static lifetime"
static LANGUAGE: &'static str = "Rust";

// this one is const, unchangeable. no surprises here
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // cannot modify const
    // THRESHOLD = 5;
}
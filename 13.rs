fn main() {
    // variable bindings are immutable by default
    // can be overriden with `mut` modifier
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // won't work
    // _immutable_binding += 1;
}
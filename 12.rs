fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy an integer
    let copied_integer = an_integer;

    println!("an integer: {:?}", copied_integer);
    println!("a boolean: {:?}", a_boolean);
    println!("a unit value: {:?}", unit);

    let _unused_variable = 3u32;
    let _noisy_unused_variable = 2u32;
}
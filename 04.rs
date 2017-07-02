// derives fmt::Debug
#[derive(Debug)]
struct Structure(i32);

// nest Structure in yet another struct
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // Similar to dumb replacement
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("And the nested {:?} will print!", Deep(Structure(8)));
}
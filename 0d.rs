#![allow(dead_code)]

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 }
}

fn inspect(p: Person) {
    // usage of an enum must cover *all* cases
    match p {
        Person::Engineer => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        // here we destructure `i`
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // destructure `info` info `name` and `height`
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    };
}

fn main() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    // to_owned() creates an owned String from a string slice
    let dave = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
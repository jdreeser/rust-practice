use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

// Here we attach methods to an enum
impl List {
    // creates an empty list
    fn new() -> List {
        Nil
    }

    // prepends values to list
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // returns length of list
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self { // prefer to match on concrete type, not reference.
            // self here is borrowed, so we can't take ownership of tail.
            // i'm not really sure what any of that means, but let's just
            // roll with it.
            // i guess we're actually destructuring Cons here.
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
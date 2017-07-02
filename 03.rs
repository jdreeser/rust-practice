fn main() {
    // General dumb replacement
    println!("{} days", 31);
    // Positional replacement
    println!("{0} is the first element. {1} is the second", "first", "second");
    // Named arguments
    println!("{subject} {verb} {object}",
    object="the lazy dog",
    verb="jumps over",
    subject="the quick brown fox");
    // Special formatting
    println!("{} of {:b} people know binary, the other half doesn't.", 1, 2);
    // Right aligned. Weird formatting sugar
    println!("{number:>width$}", number=1, width=6);
    // Padded with zeroes
    println!("{number:>0width$}", number=1, width=6);
    // Positional elements must be valid
    println!("My name is {0}, {1} {0}", "Bond", "James");
    #[allow(dead_code)]
    struct Structure(i32);
    // Defined structures need to implement some debug stuff to be printed
    // println!("This struct `{}` won't print...", Structure(3));
}
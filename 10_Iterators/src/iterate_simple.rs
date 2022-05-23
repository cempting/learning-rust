
pub fn show() {
    // Anything that is iterable can be used in a for in loop.
    // We've just seen a range being used, but it also works with a Vec:
    {
        for i in vec![52, 49, 21] {
            println!("I like the number {}", i);
        }
    }

    // Or a slice:
    {
        for i in &[52, 49, 21] {
            println!("I like the number {}", i);
        }
    }

    // Or an actual iterator:
    {
        // note: `&str` also has a `.bytes()` iterator.
        // Rust's `char` type is a "Unicode scalar value"
        for c in "rust".chars() {
            println!("Give me a {}", c);
        }        
    }

    // Even if the iterator items are filtered and mapped and flattened:
    {
        for c in "SuRPRISE INbOUND"
            .chars()
            .filter(|c| c.is_lowercase())
            .flat_map(|c| c.to_uppercase())
        {
            print!("{}", c);
        }
        println!();        
    }
}
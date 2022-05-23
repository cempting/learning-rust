fn for_each_planet<F>(f: F)
    where F: Fn(&'static str)
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}
 
pub fn show() {
    for_each_planet(|planet| println!("Hello, {}", planet));
}

// prints:
// Hello, Earth
// Hello, Mars
// Hello, Jupiter
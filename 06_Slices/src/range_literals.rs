// The .. syntax is just range literals. Ranges are just a few structs defined in the standard library.
// They can be open-ended, and their rightmost bound can be inclusive, if it's preceded by =.

pub fn show() {
    // 0 or greater
    println!("{:?}", (0..).contains(&100)); // true
    // strictly less than 20
    println!("{:?}", (..20).contains(&20)); // false
    // 20 or less than 20
    println!("{:?}", (..=20).contains(&20)); // true
    // only 3, 4, 5
    println!("{:?}", (3..6).contains(&4)); // true
}
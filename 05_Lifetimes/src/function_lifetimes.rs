/*
Functions with reference arguments can be called with borrows that have different lifetimes, so:

All functions that take references are generic
Lifetimes are generic parameters
Lifetimes' names start with a single quote, ':
*/

// elided (non-named) lifetimes:
fn print_elided(x: &i32) {
    // `x` is borrowed (from the outside) for the
    // entire time this function is called.
    println!("Elided print x={}",x);
}

// named lifetimes:
fn print_named<'a>(x: &'a i32) {
    println!("Named print x={}",x);
}

struct Number {
    value: i32,
}

fn number_value<'a>(num: &'a Number) -> &'a i32 {
    &num.value
}


pub fn show() {
    //References in function arguments also have lifetimes:
    {
        let x=42;
        print_elided(&x);
        print_named(&x);
    }

    // This allows returning references whose lifetime depend on the lifetime of the arguments:
    {
        let n = Number { value: 47 };
        let v = number_value(&n);
        // `v` borrows `n` (immutably), thus: `v` cannot outlive `n`.
        // While `v` exists, `n` cannot be mutably borrowed, mutated, moved, etc.
    }

    // When there is a single input lifetime, it doesn't need to be named, 
    // and everything has the same lifetime, so the two functions below are equivalent:
    {
        /*
        fn number_value<'a>(num: &'a Number) -> &'a i32 {
            &num.value
        }
        
        fn number_value(num: &Number) -> &i32 {
            &num.value
        }
        */
    }

}

fn for_each_planet<F>(f: F)
    where F: Fn(&'static str) + 'static // `F` must now have "'static" lifetime
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

pub fn show() {

    // this wont work
    {
        let _greeting = String::from("Good to see you");
//        for_each_planet(|planet| println!("{}, {}", greeting, planet));
        // error: closure may outlive the current function, but it borrows
        // `greeting`, which is owned by the current function
    }

    // This works
    {
        let greeting = String::from("You're doing great");
        for_each_planet(move |planet| println!("{}, {}", greeting, planet));
        // `greeting` is no longer borrowed, it is *moved* into
        // the closure.
    }
}
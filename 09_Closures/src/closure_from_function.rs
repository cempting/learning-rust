
fn make_tester(answer: String) -> impl Fn(&str) -> bool {
    move |challenge| {
        challenge == answer
    }
}

fn make_named_tester<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| {
        challenge == answer
    }
}

fn make_elided_tester(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| {
        challenge == answer
    }
}

pub fn show() {
    // You can return a closure from a function:
    {
        // you can use `.into()` to perform conversions
        // between various types, here `&'static str` and `String`
        let test = make_tester("hunter2".into());
        println!("{}", test("******"));
        println!("{}", test("hunter2"));
    }

    // You can even move a reference to some of a function's arguments, into a closure it returns:
    {
        let test = make_named_tester("hunter2");
        println!("{}", test("*******"));
        println!("{}", test("hunter2"));
    }

    // Or, with elided lifetimes:
    {
        let test = make_elided_tester("hunter2");
        println!("{}", test("*******"));
        println!("{}", test("hunter2"));
    }
}

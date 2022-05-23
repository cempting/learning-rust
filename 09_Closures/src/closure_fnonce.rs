// FnOnce closures can only be called once. They exist because some closure move out variables that have been moved when captured:

fn foobar<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f()); 
}

fn foobar_illegal<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f()); 
//    println!("{}", f()); 
    // error: use of moved value: `f`
}
 
pub fn show() {

    // This is enforced naturally, as FnOnce closures need to be moved in order to be called.
    {
        let s = String::from("alright");
        foobar(move || s);
        // `s` was moved into our closure, and our
        // closures moves it to the caller by returning
        // it. Remember that `String` is not `Copy`.
    }

    // And, if you need convincing that our closure does move s, this is illegal too:
    {
        let s = String::from("alright");
        foobar(move || s);
//        foobar(move || s);
        // use of moved value: `s`        
    }

    // But this is fine:
    {
        let s = String::from("alright");
        foobar(|| s.clone());
        foobar(|| s.clone());
    }
    
    
}

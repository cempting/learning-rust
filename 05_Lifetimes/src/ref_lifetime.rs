pub fn show() {
    // Variables bindings have a "lifetime"
    // `x` doesn't exist yet
    {
        let x = 42; // `x` starts existing
        let x_ref = &x; // `x_ref` starts existing - it borrows `x`
        println!("x_ref = {}", x_ref);
        // `x_ref` stops existing
        // `x` stops existing
    }
    // `x` no longer exists


    // The lifetime of a reference cannot exceed the lifetime of the variable binding it borrows:
    /*
    {
        let x_ref = {
            let x = 42;
            &x
        };
        println!("x_ref = {}", x_ref);
        // error: `x` does not live long enough        
    }
    */
}

fn bubbleup_error() -> Result<(), std::str::Utf8Error> {
    match std::str::from_utf8(&[240, 159, 141, 137]) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    }
    Ok(())
}

fn bubbleup_error_concise() -> Result<(), std::str::Utf8Error> {
    let s = std::str::from_utf8(&[240, 159, 141, 137])?;
    println!("{}", s);
    Ok(())
}

pub fn show() {

    // Functions that can fail typically return a Result:
    {
        let s = std::str::from_utf8(&[240, 159, 141, 137]);
        println!("{:?}", s);
        // prints: Ok("🍉")

        let s = std::str::from_utf8(&[195, 40]);
        println!("{:?}", s);
        // prints: Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })
    }

    // If you want to panic in case of failure, you can .unwrap():
    {
        let s = std::str::from_utf8(&[240, 159, 141, 137]).unwrap();
        println!("{:?}", s);
        // prints: "🍉"
    
        let _s = std::str::from_utf8(&[195, 40]).unwrap();
        // prints: thread 'main' panicked at 'called `Result::unwrap()`
        // on an `Err` value: Utf8Error { valid_up_to: 0, error_len: Some(1) }',
        // src/libcore/result.rs:1165:5
    }

    // Or .expect(), for a custom message:
    {
        let _s = std::str::from_utf8(&[195, 40]).expect("valid utf-8");
        // prints: thread 'main' panicked at 'valid utf-8: Utf8Error
        // { valid_up_to: 0, error_len: Some(1) }', src/libcore/result.rs:1165:5        
    }

    // Or, you can match:
    {
        match std::str::from_utf8(&[240, 159, 141, 137]) {
            Ok(s) => println!("{}", s),
            Err(e) => panic!("{}", e),
        }
        // prints 🍉        
    }
    
    // Or you can if let:
    {
        if let Ok(s) = std::str::from_utf8(&[240, 159, 141, 137]) {
            println!("{}", s);
        }
        // prints 🍉        
    }

    // Or you can bubble up the error:
    {
        let _ = bubbleup_error();
    }

    // Or you can use ? to do it the concise way:
    {
        let _ = bubbleup_error_concise();
    }

}

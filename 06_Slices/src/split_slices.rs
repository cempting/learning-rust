fn file_ext(name: &str) -> Option<&str> {
    // this does not create a new string - it returns
    // a slice of the argument.
    name.split(".").last()
}

pub fn show() {

    // &str values are really slices.
    {
        let name = "Read me. Or don't.txt";
        if let Some(ext) = file_ext(name) {
            println!("file extension: {}", ext);
        } else {
            println!("no file extension");
        }
    }

    // ..so the borrow rules apply here too:
    {
        let ext = {
            let name = String::from("Read me. Or don't.txt");
//            file_ext(&name).unwrap_or("")
            // error: `name` does not live long enough
        };
        println!("extension: {:?}", ext);
    }
}
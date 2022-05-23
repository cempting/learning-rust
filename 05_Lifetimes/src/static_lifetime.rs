/*
There is a special lifetime, named 'static, which is valid for the entire program's lifetime.

String literals are 'static:
*/

struct PersonStatic {
    name: &'static str,
}

struct PersonNonStatic<'a> {
    name: &'a str,
}

struct PersonOwnership {
    name: String,
}

pub fn show() {
    {
        let p = PersonStatic {
            name: "fasterthanlime",
        };
    }

    {
        /*
        In that last example, the local name is not a &'static str, it's a String. 
        It's been allocated dynamically, and it will be freed. 
        Its lifetime is less than the whole program (even though it happens to be in main).
        */
        let name = format!("fasterthan{}", "lime");
//        let p = PersonStatic { name: &name };
        // error: `name` does not live long enough
    }

    // To store a non-'static string in Person, it needs to either:
    //  A) Be generic over a lifetime:
    {
        let name = format!("fasterthan{}", "lime");
        let p = PersonNonStatic { name: &name };
        // `p` cannot outlive `name`
    }

    //  B) Take ownership of the string
    {
        let name = format!("fasterthan{}", "lime");
        let p = PersonOwnership { name: name };
        // `name` was moved into `p`, their lifetimes are no longer tied.
/*
        For many types in Rust, there are owned and non-owned variants:

        Strings: String is owned, &str is a reference
        Paths: PathBuf is owned, &Path is a reference
        Collections: Vec<T> is owned, &[T] is a reference
*/
    }
    
}
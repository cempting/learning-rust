
// Rust has slices - they're a reference to multiple contiguous elements.
//You can borrow a slice of a vector, for example:

fn tail_elided(s: &[u8]) -> &[u8] {
    &s[1..] 
}

fn tail_named<'a>(s: &'a [u8]) -> &'a [u8] {
    &s[1..] 
}

pub fn show() {

    {
        let v = vec![1, 2, 3, 4, 5];
        let v2 = &v[2..4];
        println!("v2 = {:?}", v2);
        // The above is not magical. The indexing operator (foo[index]) is overloaded with the Index and IndexMut traits.
    }

    // Borrowing rules apply to slices. elided vrsion
    {
        let x = &[1, 2, 3, 4, 5];
        let y = tail_elided(x);
        println!("y = {:?}", y);        
    }

    // named version
    {
        let x = &[1, 2, 3, 4, 5];
        let y = tail_named(x);
        println!("y = {:?}", y);        
    }

    // this is legal, ...but only because [1, 2, 3, 4, 5] is a 'static array.
    {
        let y = {
            let x = &[1, 2, 3, 4, 5];
            tail_named(x)
        };
        println!("y = {:?}", y);
    }

    // this is illegal, ...because a vector is heap-allocated, and it has a non-'static lifetime.
    {
        let y = {
            let v = vec![1, 2, 3, 4, 5];
//            tail_named(&v)
            // error: `v` does not live long enough
        };
        println!("y = {:?}", y);        
    }
}


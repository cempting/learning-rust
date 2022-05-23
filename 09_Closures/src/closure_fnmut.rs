fn foobar<F>(f: F)
    where F: Fn(i32) -> i32
{
    println!("{}", f(f(2))); 
}

fn mutable_foobar_illegal<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    //println!("{}", f(f(2))); 
    // error: cannot borrow `f` as mutable more than once at a time
}

fn mutable_foobar_legal<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    let tmp = f(2);
    println!("{}", f(tmp)); 
}
 
pub fn show() {
    // this is legal
    {
        foobar(|x| x * 2);
    }

    // this is not
    {
        mutable_foobar_illegal(|x| x * 2);
    }

    // this is legal again
    {
        mutable_foobar_legal(|x| x * 2);
    }

    // FnMut exists because some closures mutably borrow local variables:
    {
        let mut acc = 2;
        mutable_foobar_legal(|x| {
            acc += 1;
            x * acc
        });
    }

    // Those closures cannot be passed to functions expecting Fn:
    {
        let mut _acc = 2;
        foobar(|x| {
//            _acc += 1;
            // error: cannot assign to `acc`, as it is a
            // captured variable in a `Fn` closure.
            // the compiler suggests "changing foobar
            // to accept closures that implement `FnMut`"
            x * _acc
        });
    }
}
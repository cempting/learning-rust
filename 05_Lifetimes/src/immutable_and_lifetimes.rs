pub fn show() {
    {
        //A variable binding can be immutably borrowed multiple times:
        let x = 42;
        let x_ref1 = &x;
        let x_ref2 = &x;
        let x_ref3 = &x;
        println!("{} {} {}", x_ref1, x_ref2, x_ref3);
    }

    {
        // While immutably borrowed, a variable cannot be mutably borrowed:
        let mut x = 42;
        let x_ref1 = &x;
//        let x_ref2 = &mut x;
        // error: cannot borrow `x` as mutable because it is also borrowed as immutable
        println!("x_ref1 = {}", x_ref1);
    }
}
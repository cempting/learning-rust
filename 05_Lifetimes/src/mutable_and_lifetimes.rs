
pub fn show() {
    //While borrowed, a variable binding cannot be mutated:
    let mut x = 42;
    let x_ref = &x;
//    x = 13;
    println!("x_ref = {}", x_ref);
    // error: cannot assign to `x` because it is borrowed    
}

fn foobar( x : &u8) {
    println!("Borrowed x={}", x);
}

fn barfoo( x : &mut i8) {
    *x = -*x;
}

fn uninitialized_borrow() {
    let x;
//    foobar(&x); // error: borrow of possibly-uninitialized variable: `x`
    x = 42;
    println!("x={}", x );
}

fn using_after_borrow() {
    let x = 0;
    foobar(&x); // 
    println!("x={}", x );
}

fn changing_borrowed() {
    let mut x = 0;
    println!("x={}", x );
    barfoo(&mut x);
    println!("x={}", x );
}

fn swap<T : Clone + Copy> (l : &mut T, r : &mut T)
{
    let tmp = *l;
    *l = *r;
    *r = tmp.clone();
    println!("swapping");
}

use std::fmt::Debug;
fn compare<T: Debug + PartialEq>(l : T, r : T) {
    println!("{:?} {} {:?}", l, if l==r {"=="} else {"!="}, r);
}

fn try_comparing_and_swapping() {
    let mut x = "x";
    let mut y = "y";
    println!("x={}, y={}", x, y);
    compare(x,y);
    swap( &mut x, &mut y);
    println!("x={}, y={}", x, y);
    compare(x,y);
}

fn main() {
    uninitialized_borrow();
    using_after_borrow();
    changing_borrowed();
    try_comparing_and_swapping();
}


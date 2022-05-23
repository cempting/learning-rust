
fn foobar<F>(x: i32, y: i32, is_greater: F)
    where F: Fn(i32, i32) -> bool
{
    let (greater, smaller) = if is_greater(x, y) {
        (x, y)
    } else {
        (y, x)
    };
    println!("{} is greater than {}", greater, smaller);
}

fn countdown<F>(count: usize, tick: F)
    where F: Fn(usize)
{
    for i in (1..=count).rev() {
        tick(i);
    }
}

pub fn show() {

    // Here's a closure with two arguments:
    {
        foobar(32, 64, |x, y| x > y);
    }

    // Here's a closure ignoring both its arguments:
    {
        foobar(32, 64, |_, _| panic!("Comparing is futile!"));
    }
}
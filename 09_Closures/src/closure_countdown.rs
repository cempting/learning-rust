fn countdown<F>(count: usize, tick: F)
    where F: Fn(usize)
{
    for i in (1..=count).rev() {
        tick(i);
    }
}

pub fn show() {
    // Here's a slightly worrying closure:
    {
        countdown(3, |i| println!("tick {}...", i));
    }

    // And here's a toilet closure: Called thusly because |_| () looks like a toilet.
    {
        countdown(3, |_| ());
    }
}
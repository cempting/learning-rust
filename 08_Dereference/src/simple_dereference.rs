// The * operator can be used to dereference, but you don't need to do that to access fields or call methods:

struct Point {
    x: f64,
    y: f64,
}

fn _negate(p: Point) -> Point {
    Point {
        x: -p.x,
        y: -p.y,
    }
}

// now `Point` is `Copy`
#[derive(Clone, Copy)]
struct CopyablePoint {
    x: f64,
    y: f64,
}


fn copyable_negate(p: CopyablePoint) -> CopyablePoint {
    CopyablePoint {
        x: -p.x,
        y: -p.y,
    }
}

pub fn show() {
    {
        let p = Point { x: 1.0, y: 3.0 };
        let p_ref = &p;
        println!("({}, {})", p_ref.x, p_ref.y);
    }

    // And you can only do it if the type is Copy:
    {
        let p = Point { x: 1.0, y: 3.0 };
        let _p_ref = &p;
//        negate(*p_ref);
        // error: cannot move out of `*p_ref` which is behind a shared reference
    }

    // works for copyable points
    {
        let p = CopyablePoint { x: 1.0, y: 3.0 };
        let p_ref = &p;
        copyable_negate(*p_ref); // ...and now this works
    }
}

// prints `(1, 3)`
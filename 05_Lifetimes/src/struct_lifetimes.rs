struct NumRef<'a> {
    _x: &'a i32,
}

impl<'a> NumRef<'a> {
    fn as_i32_ref(&'a self) -> &'a i32 {
        self._x
    }

    fn as_i32_ref_elided(&self) -> &i32 {
        self._x
    }
}

impl NumRef<'_> {
    fn as_i32_ref_elided2(&self) -> &i32 {
        self._x
    }
}

fn as_num_ref<'a>(x: &'a i32) -> NumRef<'a> {
    NumRef { _x: &x }
}

fn as_num_ref_elided(x: &i32) -> NumRef<'_> {
    NumRef { _x: &x }
}

pub fn show() {
    // Structs can also be generic over lifetimes, which allows them to hold references:
    { 
        let x: i32 = 99;
        let _x_ref = NumRef { _x: &x };
        // `x_ref` cannot outlive `x`, etc.
    }

    // The same code, but with an additional function:
    {
        let x: i32 = 99;
        let _x_ref = as_num_ref(&x);
        // `x_ref` cannot outlive `x`, etc.
    }

    // The same code, but with "elided" lifetimes:
    {
        let x: i32 = 99;
        let _x_ref = as_num_ref_elided(&x);
        // `x_ref` cannot outlive `x`, etc.
    }   

    // impl blocks can be generic over lifetimes too:
    {
        let x: i32 = 99;
        let x_num_ref = NumRef { _x: &x };
        let _x_i32_ref = x_num_ref.as_i32_ref();
    }

    // But you can do elision ("to elide") there too:
    {
        let x: i32 = 99;
        let x_num_ref = NumRef { _x: &x };
        let _x_i32_ref = x_num_ref.as_i32_ref_elided();
    }

    // You can elide even harder, if you never need the name:
    {
        let x: i32 = 99;
        let x_num_ref = NumRef { _x: &x };
        let _x_i32_ref = x_num_ref.as_i32_ref_elided2();
    }
}
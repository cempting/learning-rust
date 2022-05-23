mod simple_lifetime;
mod ref_lifetime;
mod immutable_and_lifetimes;
mod mutable_and_lifetimes;
mod function_lifetimes;
mod struct_lifetimes;
mod impl_lifetimes;
mod static_lifetime;

fn main() {
    simple_lifetime::show();
    ref_lifetime::show();
    immutable_and_lifetimes::show();
    mutable_and_lifetimes::show();
    function_lifetimes::show();
    struct_lifetimes::show();
    impl_lifetimes::show();
    static_lifetime::show();
}

mod closure_fn;
mod closure_fn_with_borrows;
mod closure_fn_with_staticlifetime;
mod closure_fnmut;
mod closure_fnonce;
mod closure_with_multiple_arguments;
mod closure_countdown;
mod closure_from_function;

fn main() {
    closure_fn::show();
    closure_fn_with_borrows::show();
    closure_fn_with_staticlifetime::show();
    closure_fnmut::show();
    closure_fnonce::show();
    closure_countdown::show();
    closure_from_function::show();
    closure_with_multiple_arguments::show();
}

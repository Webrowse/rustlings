// TODO: Fix the compiler error by adding one or two characters.
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($a:ty, $b:ty) => {
        println!("Type is implemented");
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(i32, i32);
}

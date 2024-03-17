
fn main() {
    print_backtrace();
}

fn print_backtrace() {
    println!("{:?}", backtrace::Backtrace::new());
}
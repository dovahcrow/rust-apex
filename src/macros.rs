#[macro_export]
macro_rules! lambda_entry {
    ($func: ident) => (
        fn main() {
            ::rust_apex::run($func)
        }
    )
}
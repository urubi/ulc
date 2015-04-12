// unicode literal-ish: Will not live past the place it is used
#[macro_export]
macro_rules! ul {
    ($e: expr) => (
        <Vec<char> as AsRef<[char]>>::as_ref(&$e.chars().collect::<Vec<char>>());
    )
}
#[macro_export]
macro_rules! uv {
    ($e: expr) => (
        $e.chars().collect::<Vec<char>>()
    )
}        


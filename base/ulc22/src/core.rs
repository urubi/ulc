#[macro_export]
macro_rules! if_ {
    ($e: expr, $then: expr, $el: expr) => (
        if $e {
            $then
        }
        else {
            $el
        }
    )
}
#[macro_export]
macro_rules! ifn {
    ($e: expr, $then: expr) => (
        if !$e {
            $then
        }
    )
}
#[test]
fn test_if() {
    assert!(if_!(true, true, panic!("tehe")));
    assert!(if_!(false, panic!("tehe"), true));
    ifn!(true, panic!());
}

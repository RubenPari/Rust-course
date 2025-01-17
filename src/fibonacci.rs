///
///
/// # Arguments
///
/// * `n`: u32
///
/// returns: u32
///
pub(crate) fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

#[test]
fn test_fibonacci() {
    let n: u32 = 20;

    assert_eq!(fibonacci(n), 6765)
}

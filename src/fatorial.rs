pub(crate) fn fatorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * fatorial(n - 1)
    }
}
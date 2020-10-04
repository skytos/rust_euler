#[path = "primes.rs"] mod primes;

pub fn p3(n: usize) -> usize {
    let mut x = n;
    for p in  primes::new() {
        while x % p == 0 {
            x = x / p;
            if x == 1 { return p }
        }
    }
    return 0; // will not happen
}

#[test]
fn test_p3() {
    assert_eq!(p3(600_851_475_143), 6857);
}
pub fn p2(max: u32) -> u32 {
    rec_while(
        (1, 2, 0),
        |&(c, n, t)| { (n, c + n, if c & 1 == 0 { t + c} else { t } ) },
        |&(c, _, _)| { c <= max }
    ).2
}

fn rec_while<T, F, P>(x: T, next: F, pred: P) -> T
    where F: Fn(&T) -> T,
    P: Fn(&T) -> bool,
{
    if pred(&x) { rec_while(next(&x), next, pred) }
    else { x }
}

#[test]
fn test_p2() {
    assert_eq!(p2(4_000_000), 4613732);
}
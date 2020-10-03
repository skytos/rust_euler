fn main() {
    println!("{}", p1(1000));
}

fn p1(max: u32) -> u32 {
    (1..max).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

#[test]
fn test_p1() {
    assert_eq!(p1(1000), 233168)
}
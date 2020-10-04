mod p1;
mod p2;
mod p3;
fn main() {
    println!("{}", p1::p1(1_000));
    println!("{}", p2::p2(4_000_000));
    println!("{}", p3::p3(600_851_475_143));
}
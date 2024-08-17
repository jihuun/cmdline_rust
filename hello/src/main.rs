fn hi(a: u32) -> u32 {
    a + 1
}
fn main() {
    println!("Hello, world!");
}

#[test]
fn unittest1() {
    
    assert_eq!(hi(4), 5);
}

#[derive(Debug, PartialEq)]
pub struct Poui(pub u32);

impl std::ops::Add for Poui {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Poui(self.0.wrapping_add(rhs.0))
    }
}

#[test]
fn basic_arithmetic() {
    let a = Poui(1);
    let b = Poui(2);
    assert_eq!(a + b, Poui(3));
}

#[test]
fn wraparound() {
    let a = Poui(u32::MAX);
    let b = Poui(1);
    assert_eq!(a + b, Poui(0));
}

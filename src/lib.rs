
#[derive(Debug, PartialEq)]
pub struct Poui(pub i32);

impl std::ops::Add for Poui {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Poui(self.0 + rhs.0)
    }
}

#[test]
fn basic_arithmetic() {
    let a = Poui(1);
    let b = Poui(2);
    assert_eq!(a + b, Poui(3));
}
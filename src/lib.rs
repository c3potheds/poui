use num_traits::Num;
use num_traits::WrappingAdd;

/// A point on the unit interval (POUI).
/// 
/// The unit interval for our purposes is the interval [0, 1). Note that 1 is
/// not included in the interval.
#[derive(Debug, PartialEq)]
pub struct Poui<N: Num>(N);

impl<N: Num + WrappingAdd> std::ops::Add for Poui<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Poui(self.0.wrapping_add(&rhs.0))
    }
}

#[test]
fn basic_arithmetic_u8() {
    let a = Poui(1u8);
    let b = Poui(2u8);
    assert_eq!(a + b, Poui(3u8));
}

#[test]
fn basic_arithmetic_u16() {
    let a = Poui(1u16);
    let b = Poui(2u16);
    assert_eq!(a + b, Poui(3u16));
}

#[test]
fn basic_arithmetic_u32() {
    let a = Poui(1u32);
    let b = Poui(2u32);
    assert_eq!(a + b, Poui(3u32));
}

#[test]
fn basic_arithmetic_u64() {
    let a = Poui(1u64);
    let b = Poui(2u64);
    assert_eq!(a + b, Poui(3u64));
}

#[test]
fn basic_arithmetic_u128() {
    let a = Poui(1u128);
    let b = Poui(2u128);
    assert_eq!(a + b, Poui(3u128));
}

#[test]
fn wraparound_u8() {
    let a = Poui(u8::MAX);
    let b = Poui(1u8);
    assert_eq!(a + b, Poui(0u8));
}

#[test]
fn wraparound_u16() {
    let a = Poui(u16::MAX);
    let b = Poui(1u16);
    assert_eq!(a + b, Poui(0u16));
}

#[test]
fn wraparound_u32() {
    let a = Poui(u32::MAX);
    let b = Poui(1u32);
    assert_eq!(a + b, Poui(0u32));
}

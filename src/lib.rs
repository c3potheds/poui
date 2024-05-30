use num_traits::Num;
use num_traits::WrappingAdd;

/// A point on the unit interval.
///
/// The unit interval for our purposes is the interval [0, 1). Note that 1 is
/// not included in the interval.
///
/// A point on the unit interval is useful in many contexts:
///
/// * representing probabilities
/// * representing a fraction of a whole, which can be tacked onto an integer
///   to represent the "fractional part" of a fixed-point rational number
/// * representing a point on a circle, where 0 is 0 degrees and 1 is 360
///   degrees
/// * representing a point on a line segment, where 0 is the start and 1 is the
///   end
///
/// This type is generic over the number type used to represent the point. This
/// allows you to choose the level of precision you need for your application.
/// For example, a `u8` can represent 256 points on the unit interval, which may
/// be sufficient for some applications like representing a color gradient. A
/// `u64` can represent 2^64 points on the unit interval, which offers a very
/// high level of precision.
///
/// The purpose of `Poui` is to provide a type that is specifically designed to
/// replace the common practice of using floating-point numbers to represent
/// points on the unit interval. Floating-point numbers are not well-suited to
/// this task because:
///
/// * they are imprecise, leading to rounding errors
/// * they are slow to compute with
/// * there are many edge cases that can lead to bugs, such as NaNs, infinities,
///   and negative zeros
/// * they can take values outside the unit interval, which can lead to bugs,
///   such as when you forget to normalize a value or when you lerp with a
///   t-value greater than 1 or less than 0
///
/// `Poui` uses *fixed-point arithmetic* to represent points on the unit
/// interval to avoid these problems.
///
/// # Examples
///
/// `Poui` arithmetic is just like regular integer arithmetic:
///
/// ```rust
/// use poui::Poui;
///
/// // half-way between 0 and 1
/// let a = Poui(128u8);
/// let b = Poui(64u8);
/// let c = a + b;
/// assert_eq!(c, Poui(192u8));
/// ```
///
/// `Poui` wraps around when it reaches the end of the unit interval:
///
/// ```rust
/// use poui::Poui;
///
/// // 128 + 128 = 0
/// let a = Poui(128u8);
/// let b = Poui(128u8);
/// let c = a + b;
/// assert_eq!(c, Poui(0u8));
/// ```
///
/// If the underlying number type is a signed integer, then `Poui` represents
/// points on the interval [-0.5, 0.5) instead of [0, 1). This can be useful for
/// representing cartesian coordinates of points on a unit circle, or for
/// representing angles in a symmetric way.
///
/// ```rust
/// use poui::Poui;
///
/// // 0.25 + 0.25 = -0.5
/// let a = Poui(64i8);
/// let b = Poui(64i8);
/// let c = a + b;
/// assert_eq!(c, Poui(-128i8));
/// ```
#[derive(Debug, PartialEq)]
pub struct Poui<N: Num + WrappingAdd>(pub N);

impl<N: Num + WrappingAdd> std::ops::Add for Poui<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Poui(self.0.wrapping_add(&rhs.0))
    }
}

#[test]
fn basic_arithmetic_i8() {
    let a = Poui(1i8);
    let b = Poui(2i8);
    assert_eq!(a + b, Poui(3i8));
}

#[test]
fn basic_arithmetic_i16() {
    let a = Poui(1i16);
    let b = Poui(2i16);
    assert_eq!(a + b, Poui(3i16));
}

#[test]
fn basic_arithmetic_i32() {
    let a = Poui(1i32);
    let b = Poui(2i32);
    assert_eq!(a + b, Poui(3i32));
}

#[test]
fn basic_arithmetic_i64() {
    let a = Poui(1i64);
    let b = Poui(2i64);
    assert_eq!(a + b, Poui(3i64));
}

#[test]
fn basic_arithmetic_i128() {
    let a = Poui(1i128);
    let b = Poui(2i128);
    assert_eq!(a + b, Poui(3i128));
}

#[test]
fn wraparound_i8() {
    let a = Poui(i8::MAX);
    let b = Poui(1i8);
    assert_eq!(a + b, Poui(i8::MIN));
}

#[test]
fn wraparound_i16() {
    let a = Poui(i16::MAX);
    let b = Poui(1i16);
    assert_eq!(a + b, Poui(i16::MIN));
}

#[test]
fn wraparound_i32() {
    let a = Poui(i32::MAX);
    let b = Poui(1i32);
    assert_eq!(a + b, Poui(i32::MIN));
}

#[test]
fn wraparound_i64() {
    let a = Poui(i64::MAX);
    let b = Poui(1i64);
    assert_eq!(a + b, Poui(i64::MIN));
}

#[test]
fn wraparound_i128() {
    let a = Poui(i128::MAX);
    let b = Poui(1i128);
    assert_eq!(a + b, Poui(i128::MIN));
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

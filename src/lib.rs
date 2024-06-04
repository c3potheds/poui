use num_traits::AsPrimitive;
use num_traits::Bounded;
use num_traits::Float;
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
/// interval to avoid these problems. Points outside the unit interval are
/// *unrepresentable* in `Poui`, which means that you can't accidentally
/// go "out of bounds" and introduce bugs. The arithmetic operations are
/// well-defined and consistent with basic integer arithmetic.
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
/// points on the interval [-1.0, 1.0) instead of [0, 1). This can be useful for
/// representing cartesian coordinates of points on a unit circle, or for
/// representing angles in a symmetric way.
///
/// ```rust
/// use poui::Poui;
///
/// // 0.5 + 0.5 = -1.0
/// let a = Poui(64i8);
/// let b = Poui(64i8);
/// let c = a + b;
/// assert_eq!(c, Poui(-128i8));
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Poui<N: Num + WrappingAdd>(pub N);

impl<N: Num + WrappingAdd> std::ops::Add for Poui<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Poui(self.0.wrapping_add(&rhs.0))
    }
}

pub trait Widen {
    type Widened;
    fn widen(self) -> Self::Widened;
}

impl Widen for u8 {
    type Widened = u16;
    fn widen(self) -> Self::Widened {
        self as u16
    }
}

impl Widen for u16 {
    type Widened = u32;
    fn widen(self) -> Self::Widened {
        self as u32
    }
}

impl Widen for u32 {
    type Widened = u64;
    fn widen(self) -> Self::Widened {
        self as u64
    }
}

impl Widen for u64 {
    type Widened = u128;
    fn widen(self) -> Self::Widened {
        self as u128
    }
}

impl Widen for i8 {
    type Widened = i16;
    fn widen(self) -> Self::Widened {
        self as i16
    }
}

impl Widen for i16 {
    type Widened = i32;
    fn widen(self) -> Self::Widened {
        self as i32
    }
}

impl Widen for i32 {
    type Widened = i64;
    fn widen(self) -> Self::Widened {
        self as i64
    }
}

impl Widen for i64 {
    type Widened = i128;
    fn widen(self) -> Self::Widened {
        self as i128
    }
}

impl Widen for u128 {
    type Widened = u128;
    fn widen(self) -> Self::Widened {
        self
    }
}

impl Widen for i128 {
    type Widened = i128;
    fn widen(self) -> Self::Widened {
        self
    }
}

pub trait Shorten {
    type Shortened;
    fn shorten(self) -> Self::Shortened;
}

impl Shorten for u16 {
    type Shortened = u8;
    fn shorten(self) -> Self::Shortened {
        (self >> 8) as u8
    }
}

impl Shorten for u32 {
    type Shortened = u16;
    fn shorten(self) -> Self::Shortened {
        (self >> 16) as u16
    }
}

impl Shorten for u64 {
    type Shortened = u32;
    fn shorten(self) -> Self::Shortened {
        (self >> 32) as u32
    }
}

impl Shorten for u128 {
    type Shortened = u64;
    fn shorten(self) -> Self::Shortened {
        (self >> 64) as u64
    }
}

impl Shorten for i16 {
    type Shortened = i8;
    fn shorten(self) -> Self::Shortened {
        (self >> 8) as i8
    }
}

impl Shorten for i32 {
    type Shortened = i16;
    fn shorten(self) -> Self::Shortened {
        (self >> 16) as i16
    }
}

impl Shorten for i64 {
    type Shortened = i32;
    fn shorten(self) -> Self::Shortened {
        (self >> 32) as i32
    }
}

impl Shorten for i128 {
    type Shortened = i64;
    fn shorten(self) -> Self::Shortened {
        (self >> 64) as i64
    }
}

impl Shorten for u8 {
    type Shortened = u8;
    fn shorten(self) -> Self::Shortened {
        self
    }
}

impl Shorten for i8 {
    type Shortened = i8;
    fn shorten(self) -> Self::Shortened {
        self
    }
}

/// Multiplication of `Poui` values.
///
/// This is implemented using fixed-point arithmetic. The product of two `Poui`
/// values is itself a `Poui` value, but multiplying the underlying numbers
/// together could overflow the underlying number type. To avoid this, the
/// underlying numbers are widened to a larger type before multiplication, e.g.
/// `u8` is widened to `u16`. After multiplying the widened numbers, the result
/// is shortened back to the original type, e.g. `u16` is shortened to `u8`,
/// taking the 8 most significant bits of the result.
///
/// This method provably avoids overflow, but it may lose precision. For
/// example, multiplying `Poui(1u8)` by `Poui(1u8)` results in `Poui(0u8)`,
/// because the product is `1/512`, which is rounded down to `0`.
impl<N, M> std::ops::Mul for Poui<N>
where
    N: Num + WrappingAdd + Widen<Widened = M>,
    M: Num + WrappingAdd + std::ops::Mul + Shorten<Shortened = N>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Poui((self.0.widen() * rhs.0.widen()).shorten())
    }
}

impl<F, N> AsPrimitive<F> for Poui<N>
where
    F: Float + 'static,
    N: Num + WrappingAdd + AsPrimitive<F> + Bounded,
{
    /// Converts the point on the unit interval to a floating-point number.
    ///
    /// This is useful for converting a `Poui` to a floating-point number for
    /// use in floating-point arithmetic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use poui::Poui;
    /// use num_traits::AsPrimitive;
    ///
    /// let a = Poui(128u8);
    /// let f: f32 = a.as_();
    /// assert_eq!(f, 0.5);
    /// ```
    fn as_(self) -> F {
        N::as_(self.0) / (N::as_(N::max_value()) + N::as_(N::one()))
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

#[test]
fn wraparound_u64() {
    let a = Poui(u64::MAX);
    let b = Poui(1u64);
    assert_eq!(a + b, Poui(0u64));
}

#[test]
fn wraparound_u128() {
    let a = Poui(u128::MAX);
    let b = Poui(1u128);
    assert_eq!(a + b, Poui(0u128));
}
#[test]
fn half_times_half_u8() {
    let a = Poui(128u8);
    let b = Poui(128u8);
    assert_eq!(a * b, Poui(64u8));
}

#[test]
fn half_times_half_u16() {
    let a = Poui(32768u16);
    let b = Poui(32768u16);
    assert_eq!(a * b, Poui(16384u16));
}

#[test]
fn half_times_half_u32() {
    let a = Poui(2147483648u32);
    let b = Poui(2147483648u32);
    assert_eq!(a * b, Poui(1073741824u32));
}

#[test]
fn half_times_half_u64() {
    let a = Poui(9223372036854775808u64);
    let b = Poui(9223372036854775808u64);
    assert_eq!(a * b, Poui(4611686018427387904u64));
}

#[test]
fn zero_times_one_u8() {
    let a = Poui(0u8);
    let b = Poui(255u8);
    assert_eq!(a * b, Poui(0u8));
}

#[test]
fn zero_times_one_u16() {
    let a = Poui(0u16);
    let b = Poui(65535u16);
    assert_eq!(a * b, Poui(0u16));
}

#[test]
fn zero_times_one_u32() {
    let a = Poui(0u32);
    let b = Poui(4294967295u32);
    assert_eq!(a * b, Poui(0u32));
}

#[test]
fn zero_times_one_u64() {
    let a = Poui(0u64);
    let b = Poui(18446744073709551615u64);
    assert_eq!(a * b, Poui(0u64));
}

#[test]
fn one_sixteenth_times_one_sixteenth_u8() {
    let a = Poui(16u8);
    let b = Poui(16u8);
    assert_eq!(a * b, Poui(1u8));
}

#[test]
fn one_sixteenth_times_one_sixteenth_u16() {
    let a = Poui(4096u16);
    let b = Poui(4096u16);
    assert_eq!(a * b, Poui(256u16));
}

#[test]
fn one_sixteenth_times_one_sixteenth_u32() {
    let a = Poui(268435456u32);
    let b = Poui(268435456u32);
    assert_eq!(a * b, Poui(16777216u32));
}

#[test]
fn one_sixteenth_times_one_sixteenth_u64() {
    let a = Poui(1152921504606846976u64);
    let b = Poui(1152921504606846976u64);
    assert_eq!(a * b, Poui(72057594037927936u64));
}

#[test]
fn epsilon_times_epsilon_u8() {
    let a = Poui(1u8);
    let b = Poui(1u8);
    assert_eq!(a * b, Poui(0u8));
}

#[test]
fn epsilon_times_epsilon_u16() {
    let a = Poui(1u16);
    let b = Poui(1u16);
    assert_eq!(a * b, Poui(0u16));
}

#[test]
fn epsilon_times_epsilon_u32() {
    let a = Poui(1u32);
    let b = Poui(1u32);
    assert_eq!(a * b, Poui(0u32));
}

#[test]
fn epsilon_times_epsilon_u64() {
    let a = Poui(1u64);
    let b = Poui(1u64);
    assert_eq!(a * b, Poui(0u64));
}

#[test]
fn epsilon_times_epsilon_i8() {
    let a = Poui(1i8);
    let b = Poui(1i8);
    assert_eq!(a * b, Poui(0i8));
}

#[test]
fn epsilon_times_epsilon_i16() {
    let a = Poui(1i16);
    let b = Poui(1i16);
    assert_eq!(a * b, Poui(0i16));
}

#[test]
fn epsilon_times_epsilon_i32() {
    let a = Poui(1i32);
    let b = Poui(1i32);
    assert_eq!(a * b, Poui(0i32));
}

#[test]
fn epsilon_times_epsilon_i64() {
    let a = Poui(1i64);
    let b = Poui(1i64);
    assert_eq!(a * b, Poui(0i64));
}

#[test]
fn convert_to_f32() {
    let a = Poui(128u8);
    let f: f32 = a.as_();
    assert_eq!(f, 0.5);
}

#[test]
fn convert_to_f64() {
    let a = Poui(128u8);
    let f: f64 = a.as_();
    assert_eq!(f, 0.5);
}

#[test]
fn convert_to_f32_signed() {
    let a = Poui(64i8);
    let f: f32 = a.as_();
    assert_eq!(f, 0.5);
}

#[test]
fn convert_to_f64_signed() {
    let a = Poui(64i8);
    let f: f64 = a.as_();
    assert_eq!(f, 0.5);
}

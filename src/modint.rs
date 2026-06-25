use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// The prime modulus used by the overwhelming majority of LeetCode problems.
pub const MODULO: u32 = 1_000_000_007;

/// A `u32`-backed integer with arithmetic taken modulo [`MODULO`]
/// (`1_000_000_007`).
///
/// Every value is kept reduced into `0..MODULO`, so equality and hashing match
/// mathematical congruence classes. Products use a `u64` intermediate, which is
/// always wide enough because `(MODULO - 1)^2 < u64::MAX`. The modulus is fixed
/// at compile time; this type intentionally does **not** support an arbitrary
/// runtime modulus.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord, Debug)]
pub struct ModInt(u32);

impl ModInt {
    /// The additive identity, `0`.
    pub const ZERO: Self = Self(0);
    /// The multiplicative identity, `1`.
    pub const ONE: Self = Self(1);

    /// Returns the reduced representative in `0..MODULO`.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Modular exponentiation by squaring, computing `self^exp mod MODULO`.
    #[must_use]
    pub fn pow(self, mut exp: u64) -> Self {
        let mut base = self;
        let mut acc = Self::ONE;
        while exp > 0 {
            if exp & 1 == 1 {
                acc *= base;
            }
            base *= base;
            exp >>= 1;
        }
        acc
    }
}

impl From<u32> for ModInt {
    fn from(value: u32) -> Self {
        Self(value % MODULO)
    }
}

impl From<i32> for ModInt {
    fn from(value: i32) -> Self {
        Self(((value % MODULO as i32 + MODULO as i32) % MODULO as i32) as u32)
    }
}

impl From<ModInt> for u32 {
    fn from(value: ModInt) -> Self {
        value.0
    }
}

impl From<usize> for ModInt {
    fn from(value: usize) -> Self {
        Self((value % MODULO as usize) as u32)
    }
}

impl From<u64> for ModInt {
    fn from(value: u64) -> Self {
        Self((value % MODULO as u64) as u32)
    }
}


impl From<ModInt> for i32 {
    fn from(value: ModInt) -> Self {
        // value.0 < MODULO < i32::MAX, so the reduced representative always fits.
        value.0 as Self
    }
}

impl Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // Each operand is < MODULO, so the sum is < 2 * MODULO < 2^31 and fits in u32.
        Self::from(self.0 + rhs.0)
    }
}

impl Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        // self.0 + MODULO < 2 * MODULO < 2^31 fits in u32, so this never wraps.
        Self::from(self.0 + MODULO - rhs.0)
    }
}

impl Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from(u64::from(self.0) * u64::from(rhs.0) % u64::from(MODULO))
    }
}

impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Sum for ModInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl Product for ModInt {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

#[cfg(test)]
mod tests {
    use super::{ModInt, MODULO};

    #[test]
    fn reduces_on_construction() {
        assert_eq!(0, ModInt::from(MODULO).get());
        assert_eq!(1, ModInt::from(MODULO + 1).get());
    }

    #[test]
    fn add_wraps() {
        let a = ModInt::from(MODULO - 1);
        assert_eq!(0, (a + ModInt::ONE).get());
        assert_eq!(MODULO - 2, (a + a).get());
    }

    #[test]
    fn sub_wraps() {
        assert_eq!(MODULO - 1, (ModInt::ZERO - ModInt::ONE).get());
    }

    #[test]
    fn mul_uses_wide_intermediate() {
        let a = ModInt::from(MODULO - 1);
        assert_eq!(1, (a * a).get());
    }

    #[test]
    fn pow_matches_repeated_mul() {
        let base = ModInt::from(7u32);
        let mut expected = ModInt::ONE;
        for _ in 0..20 {
            expected *= base;
        }
        assert_eq!(expected, base.pow(20));
        assert_eq!(1, base.pow(0).get());
    }

    #[test]
    fn sum_and_product() {
        let sum: ModInt = (1..=5).map(ModInt::from).sum();
        assert_eq!(15, sum.get());
        let product: ModInt = (1..=5).map(ModInt::from).product();
        assert_eq!(120, product.get());
    }
}

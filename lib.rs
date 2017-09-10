#![no_std]
//! Shift `lhs` left by `rhs` bits, returning `None` instead
//! if that drops any nonzero bits.

pub trait SafeShl: Sized {
  /// Shift `self` left by `n` bits, returning `None` instead
  /// if that drops any nonzero bits.
  fn safe_shl(self, n: u32) -> Option<Self>;
}

macro_rules! ty {
  ($ty:ident) => {
    impl SafeShl for $ty {
      fn safe_shl(self, n: u32) -> Option<$ty> {
        if n <= self.leading_zeros() {
          self.checked_shl(n)
        } else {
          None
        }
      }
    }
    pub fn $ty(lhs: $ty, rhs: u32) -> Option<$ty> {
      lhs.safe_shl(rhs)
    }
  }
}

ty!(u8);
ty!(u16);
ty!(u32);
ty!(u64);
ty!(i8);
ty!(i16);
ty!(i32);
ty!(i64);

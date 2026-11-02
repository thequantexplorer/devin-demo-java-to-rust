//! Bit-for-bit port of `java.util.Random` (the 48-bit LCG), as far as `BillingRun` needs it.

const MULTIPLIER: i64 = 0x5DEE_CE66D;
const ADDEND: i64 = 0xB;
const MASK: i64 = (1 << 48) - 1;

#[derive(Debug, Clone)]
pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    /// `new Random(seed)`
    pub fn new(seed: i64) -> JavaRandom {
        JavaRandom {
            seed: (seed ^ MULTIPLIER) & MASK,
        }
    }

    fn next(&mut self, bits: u32) -> i32 {
        self.seed = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND) & MASK;
        (self.seed >> (48 - bits)) as i32
    }

    /// `nextInt()`
    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    /// `nextInt(bound)`
    ///
    /// # Panics
    /// If `bound <= 0` (Java throws `IllegalArgumentException`).
    pub fn next_int_bounded(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "bound must be positive");
        if bound & bound.wrapping_neg() == bound {
            return ((i64::from(bound) * i64::from(self.next(31))) >> 31) as i32;
        }
        loop {
            let bits = self.next(31);
            let val = bits % bound;
            if bits.wrapping_sub(val).wrapping_add(bound - 1) >= 0 {
                return val;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference values printed by Java 8's java.util.Random.
    #[test]
    fn matches_java_util_random() {
        let mut r = JavaRandom::new(42);
        let ints: Vec<i32> = (0..5).map(|_| r.next_int()).collect();
        assert_eq!(
            ints,
            [-1170105035, 234785527, -1360544799, 205897768, 1325939940]
        );
        assert_eq!(r.next_int_bounded(5000), 2525);
        assert_eq!(r.next_int_bounded(8), 2);
        assert_eq!(r.next_int_bounded(50000), 46918);
        assert_eq!(r.next_int_bounded(i32::MAX), 1429255519);
        assert_eq!(r.next_int_bounded(1 << 30 | 1), 196118093);

        let mut n = JavaRandom::new(-7);
        assert_eq!(n.next_int_bounded(900), 562);
        assert_eq!(n.next_int_bounded(5), 2);
        assert_eq!(n.next_int_bounded(1), 0);
    }
}

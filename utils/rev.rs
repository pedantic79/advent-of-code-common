use std::{
    hash::{Hash, Hasher},
    ops::Add,
};

pub struct Rev<T>(pub T);

impl<T: num::Zero> num::Zero for Rev<T> {
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: Add<Output = T>> std::ops::Add for Rev<T> {
    type Output = Rev<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Ord + PartialOrd> Ord for Rev<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: PartialEq> PartialEq for Rev<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Eq for Rev<T> {}

impl<T: Hash> Hash for Rev<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Clone> Clone for Rev<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy> Copy for Rev<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for Rev<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Rev").field(&self.0).finish()
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Rev<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;
    use num::Zero;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_rev_ordering() {
        let a = Rev(10);
        let b = Rev(20);

        // a should be greater than b because the underlying order is reversed
        assert!(a > b);
        assert!(b < a);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_rev_equality_and_traits() {
        let a = Rev(10);
        let b = Rev(10);
        let c = Rev(20);

        assert_eq!(a, b);
        assert_ne!(a, c);

        // Zero
        let z: Rev<i32> = num::Zero::zero();
        assert_eq!(z.0, 0);
        assert!(z.is_zero());

        // Add
        let sum = a + c;
        assert_eq!(sum.0, 30);

        // Clone & Copy
        let cloned = a.clone();
        assert_eq!(cloned.0, 10);
        let copied = a;
        assert_eq!(copied.0, 10);

        // Hash
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        a.hash(&mut h1);
        10.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_rev_formatting() {
        let r = Rev(42);
        assert_eq!(format!("{:?}", r), "Rev(42)");
        assert_eq!(format!("{}", r), "42");
    }
}

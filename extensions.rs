pub trait MinMaxIterator: Iterator {
    fn min_max<'a, T>(mut self) -> Option<(&'a T, &'a T)>
    where
        T: Ord,
        Self: Iterator<Item = &'a T> + Sized,
    {
        self.next()
            .map(|x| self.fold((x, x), |(min, max), num| (min.min(num), max.max(num))))
    }
}

impl<T: ?Sized> MinMaxIterator for T where T: Iterator {}

pub trait AddIsize
where
    Self: Sized + PartialOrd,
{
    fn checked_add_isize(self, rhs: isize) -> Option<Self>;
    fn checked_add_isize_clamp(self, rhs: isize, max: Self) -> Option<Self> {
        self.checked_add_isize(rhs).filter(|x| x < &max)
    }
}

impl AddIsize for usize {
    fn checked_add_isize(self, rhs: isize) -> Option<Self> {
        let amount = Self::try_from(rhs.abs()).ok()?;
        if rhs < 0 {
            self.checked_sub(amount)
        } else {
            self.checked_add(amount)
        }
    }
}

pub trait GetMutTwice {
    type Output;

    fn get_mut_twice(
        &mut self,
        index0: usize,
        index1: usize,
    ) -> (&mut Self::Output, &mut Self::Output);
}

impl<T> GetMutTwice for [T] {
    type Output = T;

    fn get_mut_twice(
        &mut self,
        index0: usize,
        index1: usize,
    ) -> (&mut Self::Output, &mut Self::Output) {
        crate::common::utils::slice_get_mut_twice(self, index0, index1)
    }
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_iterator() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.iter().min_max(), None);

        let single = vec![42];
        assert_eq!(single.iter().min_max(), Some((&42, &42)));

        let multiple = vec![10, 5, 20, 15, 5];
        assert_eq!(multiple.iter().min_max(), Some((&5, &20)));
    }

    #[test]
    fn test_add_isize() {
        let base: usize = 10;
        assert_eq!(base.checked_add_isize(5), Some(15));
        assert_eq!(base.checked_add_isize(-5), Some(5));
        assert_eq!(base.checked_add_isize(-10), Some(0));
        assert_eq!(base.checked_add_isize(-11), None);

        // Clamping logic
        assert_eq!(base.checked_add_isize_clamp(5, 20), Some(15));
        assert_eq!(base.checked_add_isize_clamp(5, 15), None); // Not less than max (15 is not < 15)
        assert_eq!(base.checked_add_isize_clamp(-5, 10), Some(5));
    }

    #[test]
    fn test_get_mut_twice() {
        let mut slice = vec![1, 2, 3, 4];
        let (a, b) = slice.get_mut_twice(1, 3);
        assert_eq!(*a, 2);
        assert_eq!(*b, 4);
        *a = 20;
        *b = 40;
        assert_eq!(slice, vec![1, 20, 3, 40]);
    }

    #[test]
    #[should_panic]
    fn test_get_mut_twice_same_index() {
        let mut slice = vec![1, 2, 3, 4];
        let _ = slice.get_mut_twice(2, 2);
    }
}

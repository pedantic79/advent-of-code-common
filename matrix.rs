pub fn rotate_right<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    let len = a.len();
    for i in 0..(len / 2) {
        for j in i..(len - i - 1) {
            let temp = a[i].as_mut()[j];
            a[i].as_mut()[j] = a[len - 1 - j].as_mut()[i];
            a[len - 1 - j].as_mut()[i] = a[len - 1 - i].as_mut()[len - 1 - j];
            a[len - 1 - i].as_mut()[len - 1 - j] = a[j].as_mut()[len - 1 - i];
            a[j].as_mut()[len - 1 - i] = temp;
        }
    }
}

pub fn rotate_left<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    let len = a.len();
    for i in 0..(len / 2) {
        for j in i..(len - i - 1) {
            let temp = a[i].as_mut()[j];
            a[i].as_mut()[j] = a[j].as_mut()[len - 1 - i];
            a[j].as_mut()[len - 1 - i] = a[len - 1 - i].as_mut()[len - 1 - j];
            a[len - 1 - i].as_mut()[len - 1 - j] = a[len - 1 - j].as_mut()[i];
            a[len - 1 - j].as_mut()[i] = temp;
        }
    }
}

pub fn rotate_bottom<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    let len = a.len();

    if len % 2 == 1 {
        for j in 0..(len / 2) {
            a[len / 2].as_mut().swap(j, len - j - 1);
        }
    }

    for i in 0..(len / 2) {
        for j in 0..len {
            let temp = a[i].as_mut()[j];
            a[i].as_mut()[j] = a[len - i - 1].as_mut()[len - j - 1];
            a[len - i - 1].as_mut()[len - j - 1] = temp;
        }
    }
}

pub fn flip<T, A>(a: &mut [A])
where
    T: Default + Copy,
    A: AsMut<[T]>,
{
    assert_eq!(a.len(), a[0].as_mut().len(), "not a square matrix");
    for row in a.iter_mut() {
        row.as_mut().reverse();
    }
}

pub fn rotate_right_m_n<T, A>(a: &[A]) -> Vec<Vec<T>>
where
    T: Default + Copy,
    A: AsRef<[T]>,
{
    let rows = a.len();
    let cols = a[0].as_ref().len();

    let mut rotated: Vec<Vec<T>> = vec![vec![T::default(); rows]; cols];
    for i in 0..rows {
        for (j, &v) in a[rows - 1 - i].as_ref().iter().enumerate() {
            rotated[j][i] = v;
        }
    }

    rotated
}

pub fn rotate_left_m_n<T, A>(a: &[A]) -> Vec<Vec<T>>
where
    T: Default + Copy,
    A: AsRef<[T]>,
{
    let rows = a.len();
    let cols = a[0].as_ref().len();

    let mut rotated = vec![vec![T::default(); rows]; cols];
    for (i, row) in a.iter().enumerate() {
        for j in 0..cols {
            rotated[cols - 1 - j][i] = row.as_ref()[j];
        }
    }

    rotated
}

pub fn rotate_bottom_m_n<T, A>(a: &[A]) -> Vec<Vec<T>>
where
    T: Default + Copy,
    A: AsRef<[T]>,
{
    let rows = a.len();
    let cols = a[0].as_ref().len();

    let mut rotated = vec![vec![T::default(); cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            rotated[rows - 1 - i][cols - 1 - j] = a[i].as_ref()[j];
        }
    }

    rotated
}

#[cfg(feature = "common_test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_rotations_and_flip() {
        // Let's create a 3x3 square matrix
        // 1 2 3
        // 4 5 6
        // 7 8 9
        #[rustfmt::skip]
        let mut mat = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ];

        // Flip (reverse each row)
        // 3 2 1
        // 6 5 4
        // 9 8 7
        flip(&mut mat);
        #[rustfmt::skip]
        assert_eq!(mat, [
            [3, 2, 1],
            [6, 5, 4],
            [9, 8, 7]
        ]);

        // Flip back
        flip(&mut mat);

        // Rotate Right (90 deg clockwise)
        // 7 4 1
        // 8 5 2
        // 9 6 3
        rotate_right(&mut mat);
        #[rustfmt::skip]
        assert_eq!(mat, [
            [7, 4, 1],
            [8, 5, 2],
            [9, 6, 3]
        ]);

        // Rotate Left (90 deg counter-clockwise) back to original
        rotate_left(&mut mat);
        #[rustfmt::skip]
        assert_eq!(mat, [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);

        // Rotate Bottom (180 deg)
        // 9 8 7
        // 6 5 4
        // 3 2 1
        rotate_bottom(&mut mat);
        #[rustfmt::skip]
        assert_eq!(mat, [
            [9, 8, 7],
            [6, 5, 4],
            [3, 2, 1]
        ]);
    }

    #[test]
    fn test_rectangular_rotations() {
        // Create a 2x3 matrix (M x N)
        // 1 2 3
        // 4 5 6
        #[rustfmt::skip]
        let mat = [
            [1, 2, 3],
            [4, 5, 6],
        ];

        // rotate_right_m_n: should be 3x2
        // 4 1
        // 5 2
        // 6 3
        let r_right = rotate_right_m_n(&mat);
        #[rustfmt::skip]
        assert_eq!(r_right, vec![
            vec![4, 1],
            vec![5, 2],
            vec![6, 3],
        ]);

        // rotate_left_m_n: should be 3x2
        // 3 6
        // 2 5
        // 1 4
        let r_left = rotate_left_m_n(&mat);
        #[rustfmt::skip]
        assert_eq!(r_left, vec![
            vec![3, 6],
            vec![2, 5],
            vec![1, 4],
        ]);

        // rotate_bottom_m_n: should be 2x3
        // 6 5 4
        // 3 2 1
        let r_bottom = rotate_bottom_m_n(&mat);
        #[rustfmt::skip]
        assert_eq!(r_bottom, vec![
            vec![6, 5, 4],
            vec![3, 2, 1],
        ]);
    }
}

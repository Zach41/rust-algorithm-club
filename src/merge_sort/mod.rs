fn merge_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let n = array.len();

    let mut z = vec![array.to_vec(), array.to_vec()];
    let mut d = 0;
    let mut width = 1;

    while width < n {
        let mut i = 0;

        while i < n {
            let mut idx = i;
            let mut left = i;
            let mut right = i + width;

            let lmax = ::std::cmp::min(left + width, n);
            let rmax = ::std::cmp::min(right + width, n);

            while left < lmax && right < rmax {
                if z[d][left] < z[d][right] {
                    z[1 - d][idx] = z[d][left].clone();
                    left += 1
                } else {
                    z[1 - d][idx] = z[d][right].clone();
                    right += 1;
                }
                idx += 1;
            }

            while left < lmax {
                z[1 - d][idx] = z[d][left].clone();
                left += 1;
                idx += 1;
            }

            while right < rmax {
                z[1 - d][idx] = z[d][right].clone();
                right += 1;
                idx += 1;
            }

            i += width * 2;
        }
        width *= 2;
        d = 1 - d;
    }
    z[d].clone()
}

#[cfg(test)]
mod test {
    use super::merge_sort;

    #[test]
    fn test_merge_sort() {
        let numbers = vec![5, 3, 2, 98, 34, 23, 45, 234, 54, 3, 4, 2, 1];
        assert_eq!(merge_sort(&numbers),
                   vec![1, 2, 2, 3, 3, 4, 5, 23, 34, 45, 54, 98, 234])
    }

    #[test]
    fn test_merge_sort2() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(merge_sort(&numbers), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let numbers2 = vec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(merge_sort(&numbers2), vec![1, 1, 1, 1, 1, 1, 1]);
    }
}

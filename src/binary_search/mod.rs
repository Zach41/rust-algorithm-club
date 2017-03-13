pub fn binary_search<T: Ord>(a: &[T], key: T, left: usize, right: usize) -> Option<usize> {
    if left >= right {
        return None;
    }

    let middle_idx = left + (right - left) / 2;
    if a[middle_idx] > key {
        binary_search(a, key, left, middle_idx)
    } else if a[middle_idx] < key {
        binary_search(a, key, middle_idx + 1, right)
    } else {
        Some(middle_idx)
    }
}

pub fn binary_search2<T: Ord>(a: &[T], key: T) -> Option<usize> {
    let mut left = 0;
    let mut right = a.len();
    while left < right {
        let middle_idx = left + (right - left) / 2;
        if a[middle_idx] == key {
            return Some(middle_idx)
        } else if a[middle_idx] < key {
            left = middle_idx + 1;
        } else {
            right = middle_idx;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        let numbers = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67];
        assert_eq!(binary_search(&numbers, 43, 0, numbers.len()), Some(13));
        assert_eq!(binary_search(&numbers, 7, 0, numbers.len()), Some(3));
        assert_eq!(binary_search(&numbers, 14, 0, numbers.len()), None);
    }

    #[test]
    fn test_search2() {
        let numbers = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67];
        assert_eq!(binary_search2(&numbers, 43), Some(13));
        assert_eq!(binary_search2(&numbers, 7), Some(3));
        assert_eq!(binary_search2(&numbers, 14), None);
    }
}

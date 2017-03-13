pub fn cnt_occurrences<T: Ord>(array: &[T], key: T) -> usize {
    let low_idx = || {
        let mut low = 0;
        let mut high = array.len();

        while low < high {
            let middle = low + (high - low) / 2;
            if array[middle] < key {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        low
    };

    let high_idx = || {
        let mut low = 0;
        let mut high = array.len();

        while low < high {
            let middle = low + (high - low) / 2;
            if array[middle] > key {
                high = middle;
            } else {
                low = middle + 1;
            }
        }
        low
    };

    high_idx() - low_idx()
}

#[cfg(test)]
mod test {
    use super::cnt_occurrences;

    #[test]
    fn test_sorted_cnt() {
        let numbers = vec![1, 2, 3, 4, 5, 5, 6, 6, 6, 6, 8, 8, 8, 10];
        assert_eq!(cnt_occurrences(&numbers, 1), 1);
        assert_eq!(cnt_occurrences(&numbers, 6), 4);
        assert_eq!(cnt_occurrences(&numbers, 5), 2);
        assert_eq!(cnt_occurrences(&numbers, 9), 0);
    }

    #[test]
    fn test_same() {
        let same = vec![1, 1, 1, 1, 1, 1, 1];
        assert_eq!(cnt_occurrences(&same, 1), 7);
    }
}

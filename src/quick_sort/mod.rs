pub fn quick_sort<T: Ord>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition(array, low, high);
        quick_sort(array, low, p-1);
        quick_sort(array, p+1, high);
    }
}

fn partition<T: Ord>(array: &mut [T], low: usize, high: usize) -> usize {
    let mut cur = low;
    for idx in low..high {
        if array[idx] <= array[high] {
            array.swap(idx, cur);
            cur += 1;
        }
    }
    array.swap(cur, high);
    cur
}

#[cfg(test)]
mod test{
    use super::quick_sort;

    #[test]
    fn test_quick_sort() {
        let mut numbers = [1, 5, 2, 4, 3, 9, 7, 8, 6];
        quick_sort(&mut numbers, 0, 8);
        assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7, 8, 9])
    }

    #[test]
    fn test_quick_sort2() {
        let mut numbers = [1, 1, 1, 1, 1, 1];
        quick_sort(&mut numbers, 0, 5);
        assert_eq!(numbers, [1, 1, 1, 1, 1, 1]);
    }
}

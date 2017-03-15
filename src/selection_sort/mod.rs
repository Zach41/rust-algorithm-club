pub fn selection_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let mut a = array.to_vec();

    for idx in 0..a.len() {
        let mut lowest = idx;
        for y in lowest+1..a.len() {
            if a[y] < a[lowest] {
                lowest = y;
            }
        }
        if lowest != idx {
            a.swap(lowest, idx)
        }
    }
    a
}

#[cfg(test)]
mod test {
    use super::selection_sort;

    #[test]
    fn test_selection_sort() {
        let numbers = [1, 7, 4, 3, 5, 2, 6];
        assert_eq!(selection_sort(&numbers), [1, 2, 3, 4, 5, 6, 7]);
    }
}

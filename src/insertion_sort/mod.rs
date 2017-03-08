pub fn insertion_sort<T, F>(array: &mut [T], sort: F)
    where F: Fn(&T, &T) -> bool {
    for idx in 1..array.len() {
        
        let mut y = idx;

        while y > 0 && sort(&array[y], &array[y-1]) {
            array.swap(y, y-1);
            y -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::insertion_sort;

    #[test]
    fn test_insertion_sort() {
        let mut array = vec![2, 5, 3, 2, 1, 10, 7, 5];
        insertion_sort(&mut array, |a, b| a < b);
        assert_eq!(array, vec![1, 2, 2, 3, 5, 5, 7, 10]);
    }
}    


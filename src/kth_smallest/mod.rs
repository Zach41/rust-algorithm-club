pub fn kth_smallest<T: Ord + Clone>(array: &[T], k: usize) -> T {
    assert!(k < array.len());

    let mut arr = array.to_vec();
    let len = arr.len();
    select(&mut arr, 0, len - 1, k)
}

fn get_pivot<T: Clone>(array: &mut [T], low: usize, high: usize) -> T {
    assert!(array.len() > 0 && high < array.len());
    assert!(high >= low);
    
    let middle = low + (high - low) / 2;
    array.swap(middle, high);
    array[high].clone()
}

fn partition<T: Ord + Clone>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = get_pivot(array, low, high);

    let mut i = low;
    for j in low..high {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, high);
    i
}

fn select<T: Ord + Clone>(array: &mut [T], low: usize, high: usize, order: usize) -> T {
    if low < high {
        let p = partition(array, low, high);
        if p == order {
            array[p].clone()
        } else if p < order {
            select(array, p + 1, high, order)
        } else {
            select(array, low, p - 1, order)
        }
    } else {
        array[low].clone()
    }
}

#[cfg(test)]
mod test {
    use super::kth_smallest;

    #[test]
    fn test_kth_smallest() {
        let numbers = vec![2, 5, 4, 3, 1, 8, 7, 6];
        assert_eq!(kth_smallest(&numbers, 0), 1);
        assert_eq!(kth_smallest(&numbers, 1), 2);
        assert_eq!(kth_smallest(&numbers, 2), 3);
        assert_eq!(kth_smallest(&numbers, 3), 4);
        assert_eq!(kth_smallest(&numbers, 4), 5);
        assert_eq!(kth_smallest(&numbers, 5), 6);
        assert_eq!(kth_smallest(&numbers, 6), 7);
        assert_eq!(kth_smallest(&numbers, 7), 8);        
    }

    #[test]
    fn test_duplicates() {
        let numbers = vec![1, 2, 1, 2, 3, 2, 3, 3];
        assert_eq!(kth_smallest(&numbers, 0), 1);
        assert_eq!(kth_smallest(&numbers, 1), 1);
        assert_eq!(kth_smallest(&numbers, 3), 2);
        assert_eq!(kth_smallest(&numbers, 2), 2);
        assert_eq!(kth_smallest(&numbers, 4), 2);
        assert_eq!(kth_smallest(&numbers, 5), 3);
        assert_eq!(kth_smallest(&numbers, 6), 3);
        assert_eq!(kth_smallest(&numbers, 7), 3);

        let same = [1, 1, 1, 1];
        assert_eq!(kth_smallest(&same, 0), 1);
        assert_eq!(kth_smallest(&same, 1), 1);
        assert_eq!(kth_smallest(&same, 2), 1);
        assert_eq!(kth_smallest(&same, 3), 1);
    }    
}

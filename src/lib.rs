pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 3, 5, 7, 9, 11, 13];
        assert_eq!(binary_search(&arr, &5), Some(2));
        assert_eq!(binary_search(&arr, &9), Some(4));
        assert_eq!(binary_search(&arr, &13), Some(6));
        assert_eq!(binary_search(&arr, &2), None);
    }
}

// 地精排序

use std::cmp::{PartialEq, PartialOrd};

pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    let mut arr = arr.to_vec();
    let mut i = 1;
    let mut j = 2;

    while i < arr.len() {
        if arr[i - 1] <= arr[i] {
            i = j;
            j += 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

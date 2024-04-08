// 插入排序
pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn insertion_sort_binary_search<T: Ord>(arr: &mut [T]) {
    // 从第二个元素开始排序
    for i in 1..arr.len() {
        // 利用二分查找获取 arr[i] 应该插入的位置
        let pos = arr[..i].binary_search(&arr[i]).unwrap_or_else(|x| x);
        let mut j = i;
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

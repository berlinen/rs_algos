// 冒泡排序
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let size = arr.len();

    if size <= 1 {
        return;
    }

    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;

        // 最后i个元素已经排好了顺序，不需要再比较
        for j in 1..(size - i) {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}

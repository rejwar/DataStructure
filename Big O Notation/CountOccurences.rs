fn lower_bound(arr: &[i32], target: i32) -> usize {
    let (mut left, mut right) = (0usize, arr.len());
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn upper_bound(arr: &[i32], target: i32) -> usize {
    let (mut left, mut right) = (0usize, arr.len());
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn count_occurrence(arr: &[i32], target: i32) -> usize {
    upper_bound(arr, target) - lower_bound(arr, target)
}

fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 4, 4, 4, 9];
    arr.sort(); // নিশ্চিত করো যে sorted

    let target = 4;
    let count = count_occurrence(&arr, target);
    println!("{} number comes {} times ", target, count);
}

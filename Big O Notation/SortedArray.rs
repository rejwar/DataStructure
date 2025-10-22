fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, arr.len());

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    let arr = [1, 2, 3, 5, 7, 9, 11];
    let target = 9;

    match binary_search(&arr, target) {
        Some(index) => println!("{} got it in index {}", target, index),
        None => println!("{} didn't get the value ", target),
    }
}

fn last_occurence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    let mut result = None;

    while left < right {
        let mid = (left + right) / 2;

        if arr[mid] == target {
            result = Some(mid);

            left = mid + 1;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    result
}

fn main() {
    let arr = [1, 2, 4, 4, 4, 4, 5, 6, 7];
    let target = 4;

    match last_occurence(&arr, target) {
        Some(index) => println!("{} last index is {}", target, index),
        None => println!("{} didn't get the number ", target),
    }
}

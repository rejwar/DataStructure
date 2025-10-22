fn first_occurence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    let mut result = None;

    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            result = Some(mid);
            right = mid;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    result
}

fn main() {
    let arr = [1, 2, 3, 4, 4, 4, 5, 6];
    let target = 4;

    match first_occurence(&arr, target) {
        Some(index) => println!("First time {}  got in index {}", target, index),
        None => println!("{} we didn't get it ", target),
    }
}

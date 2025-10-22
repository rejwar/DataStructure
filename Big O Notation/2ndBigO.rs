use core::num;
use std::ops::Index;

fn BinarySearch(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right: i32 = arr.len().wrapping_sub(1);
    while left <= right {
        let mid = (left + right) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }
    None
}

fn main() {
    let numbers = [2, 3, 5, 8, 12, 16, 18, 23, 38, 56, 72, 91];
    let target = 23;

    match BinarySearch(&numbers, target) {
        Some(Index) => println!("{} we get it in {}", target, Index),
        None => println!("X {} we did not get it", target),
    }
}

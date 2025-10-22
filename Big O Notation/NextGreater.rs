// File: NextGreater.rs
// Problem 6: Find the first element strictly greater than target (upper bound)
// Time Complexity: O(log n)

// UpperBound: প্রথম index যেখানে arr[idx] > target
fn UpperBound(arr: &[i32], target: i32) -> usize {
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

// NextGreater: UpperBound দিয়ে মান ফেরত দেয়
fn NextGreater(arr: &[i32], target: i32) -> Option<i32> {
    let i = UpperBound(arr, target);
    if i < arr.len() {
        Some(arr[i])
    } else {
        None
    }
}

fn Main() {
    let arr = [1, 3, 4, 4, 6, 8, 10];

    let t1 = 4;
    println!("NextGreater({}): {:?}", t1, NextGreater(&arr, t1)); // → Some(6)

    let t2 = 7;
    println!("NextGreater({}): {:?}", t2, NextGreater(&arr, t2)); // → Some(8)

    let t3 = 10;
    println!("NextGreater({}): {:?}", t3, NextGreater(&arr, t3)); // → None
}

// Rust বাইনারি চালাতে main ফাংশন দরকার, তাই এখানে Main() কে কল করছি:
fn main() {
    Main();
}

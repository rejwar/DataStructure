fn search_insert_position(arr: &[i32], target: i32) -> usize {
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

fn main() {
    let arr = [1, 3, 5, 7];
    let target = 5;
    let pos1 = search_insert_position(&arr, target);
    println!("{} got in the inbox no {}", target, pos1);

    let target2 = 2;
    let pos2 = search_insert_position(&arr, target);
    println!("{} will sit on {} no index", target2, pos2);
}

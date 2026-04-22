fn left_rotate_by_one(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let first = arr[0]; // Save the first element
    for i in 1..n {
        arr[i - 1] = arr[i]; // Shift left
    }
    arr[n - 1] = first; // Move first element to the end
}

fn main() {
    let mut my_array = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", my_array);

    left_rotate_by_one(&mut my_array);
    println!("After left rotate by one: {:?}", my_array);
}

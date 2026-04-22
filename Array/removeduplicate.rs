fn remove_duplicate(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut i = 0;
    for j in 1..arr.len() {
        if arr[j] != arr[i] {
            i += 1;

            arr[i] = arr[j];
        }
    }

    i + 1
}

fn main() {
    let mut my_vec: Vec<i32> = vec![1 - 3];
    println!(" {:?}", my_vec);

    let unique_count = remove_duplicate(&mut my_vec);
    println!(" Number of unique elements {}", unique_count);

    println!(" Modified Vector {:?}", &my_vec[..unique_count])
}

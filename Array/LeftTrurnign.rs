fn left_rotate_by_d(arr: &mut [i32], d: usize) {
    let n = arr.len();

    if n <= 1 {
        return;
    }

    let d = d % n;

    if d == 0 {
        return;
    }

    arr[0..d].reverse();

    arr[0..n].reverse();

    arr.reverse();
}

fn main() {
    let mut my_array = [2 - 8];
    let d = 3;

    println!(" Original Array {:?}", my_array);

    left_rotate_by_d(&mut my_array, d);

    println!(" {} turing into left side {:?}", d, my_array);
}

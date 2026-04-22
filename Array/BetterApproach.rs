fn second_largest(arr: [i32]) -> i32 {
    let mut largest = arr;
    let mut second_largest = -1;

    for &num in arr.iter().skip(1) {
        if num > largest {
            second_largest = largest;

            largest = num;
        } else if num < largest && num > second_largest {
            second_largest = num;
        }
    }

    second_largest
}

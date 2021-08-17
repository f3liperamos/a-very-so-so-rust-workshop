/// In this simple exercise, you will create a program that will take two lists of integers, a and b.
/// Each list will consist of 3 positive integers above 0, representing the dimensions of cuboids a and b.
/// You must find the difference of the cuboids' volumes regardless of which is bigger.
///
/// For example, if the parameters passed are ([2, 2, 3], [5, 4, 1]), the volume of a is 12 and the volume of b is 20.
/// Therefore, the function should return 8.
///

fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    todo!()
}

#[test]
fn tests() {
    assert_eq!(find_difference(&[3, 2, 5], &[1, 4, 4]), 14);
    assert_eq!(find_difference(&[9, 7, 2], &[5, 2, 2]), 106);
    assert_eq!(find_difference(&[11, 2, 5], &[1, 10, 8]), 30);
    assert_eq!(find_difference(&[4, 4, 7], &[3, 9, 3]), 31);
    assert_eq!(find_difference(&[15, 20, 25], &[10, 30, 25]), 0);
}

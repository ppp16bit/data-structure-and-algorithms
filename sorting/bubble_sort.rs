fn bubble_sort<T: Ord>(array: &mut [T]) {
    let n = array.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut array);
    println!("{:?}", array);
}
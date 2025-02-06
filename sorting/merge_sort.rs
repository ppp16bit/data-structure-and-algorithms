fn merge_sort(array: &mut [i32]) {
    if array.len() <= 1 {
        return;
    }

    let mid = array.len() / 2;
    let (left, right) = array.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let merged = merge(left, right);
    array.copy_from_slice(&merged);
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut result = Vec::with_capacity(left.len() + right.len());

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] { result.push(left[i]); i += 1; 
        } else {
            result.push(right[j]); j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

fn main() {
    let mut array = [12, 11, 13, 5, 6, 7];
    merge_sort(&mut array);
    println!("{:?}", array);
}
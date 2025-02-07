fn binary_search(array: &[i32], target: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = array.len();

    while start < end {
        let mid = start + (end - start) / 2;

        // usando match para comparar a posicao mid do array com o target
        match array[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => start = mid + 1,
            std::cmp::Ordering::Greater => end = mid,
        }
    }
    None
}

fn main() {
    let array = vec![1, 3, 5, 7, 9, 11, 13];
    let target = 7;

    if let Some(index) = binary_search(&array, target) {
        println!("{}", array[index]);
    }
}
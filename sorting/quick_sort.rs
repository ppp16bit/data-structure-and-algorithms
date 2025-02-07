fn quicksort(array: &mut [i32]) {
    let len = array.len();
    if len <= 1 {
        return;
    }

    let mut stack = Vec::with_capacity(len);
    stack.push((0, len - 1));

    while let Some((low, high)) = stack.pop() {
        let pivot_index = partition(array, low, high);
        
        if pivot_index + 1 < high {
            stack.push((pivot_index + 1, high));
        }
    }
}

fn partition(array: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = array[high];
    let mut i = low as isize - 1;

    for j in low..high {
        if array[j] <= pivot {
            i += 1;
            array.swap(i as usize, j);
        }
    }
    array.swap((i + 1) as usize, high);
    (i + 1) as usize
}

fn main() {
    let mut array = [11, 54, 98, 24, 21, 50];
    quicksort(&mut array);
    println!("{:?}", array);
}
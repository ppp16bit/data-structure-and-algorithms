fn heapify(array: &mut [i32], n: usize, i: usize) {
    let (left, right) = (2 * i + 1, 2 * i + 2);
    let largest = [left, right]
        .iter()
        .filter(|&&c| c < n)
        .fold(i, |acc, &c| if array[c] > array[acc] {c} else { acc });
    if largest != i {
        array.swap(i, largest);
        heapify(array, n, largest);
    }
}

fn heap_sort(array: &mut [i32]) {
    let n = array.len();
    (0..n / 2).rev().for_each(|i| heapify(array, n, i));
    (1..n).rev().for_each(|i| {
        array.swap(0, i);
        heapify(array, i, 0);
    });
}

fn main() {
    let mut array = [12, 11, 13, 5, 6, 7];
    heap_sort(&mut array);
    println!("{:?}", array);
}

/* tentei escrever algo mais "enxuto" e que mantesse a complexidade 
O(n) na construção do heap e O(n log n) na ordenação */
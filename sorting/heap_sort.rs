// paradigma imperativo
fn heapify<T: Ord>(array: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && array[right] > array[largest] { // se o esquerdo for maior que o node atual att o idx do maior
        largest = left;
    }

    if right < n && array[right] > array[largest] { // se o direito for maior que o node atual att o idx do maior
        largest = right;
    }

    if largest != i { // continua o heapfy recursivamente se o node atual nao for o maior
        array.swap(i, largest);
        heapify(array, n, largest);
    }
}

fn heap<T: Ord>(array: &mut [T]) {
    let n = array.len();

    for i in (0..n /2).rev() {
        heapify(array, n, i);
    }

    for i in (1..n).rev() {
        array.swap(0, i);
        heapify(array, i, 0);
    }
}

// a complexidade continua O(n log n) porem sofre com overhead de memoria maior
// tambem nao me livrei 100% da mutabilidade :p
/*fn heapify(array: &mut [i32], n: usize, i: usize) {
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
}*/

fn main() {
    let mut array = [12, 11, 13, 5, 6];
    heap(&mut array);
    println!("{:?}", array);
}
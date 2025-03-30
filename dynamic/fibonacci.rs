fn lader(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    let mut a = 1;
    let mut b = 1;

    for _ in 2..=n {
        let c = a + b;

        a = b;
        b = c;
    }
    b
}

fn main() {
    println!("{}", lader(1));
    println!("{}", lader(2));
    println!("{}", lader(3));
    println!("{}", lader(4));
}
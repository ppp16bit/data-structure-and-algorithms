#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    // ex
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("stack size: {}", stack.size());
    println!("top stack: {:?}", stack.peek());

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!("empty?: {}", stack.is_empty());
}
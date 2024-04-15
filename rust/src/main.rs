trait Strategy {
    fn execute(&self, a: i32, b: i32) -> i32;
}

struct AddStrategy {}
impl Strategy for AddStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct SubtractStrategy {}
impl Strategy for SubtractStrategy {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

struct Multplication {}
impl Strategy for Multplication {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    fn execute(&self, a: i32, b: i32) -> i32 {
        self.strategy.execute(a, b)
    }

    fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }
}

fn main() {
    let mut context = Context::new(Box::new(AddStrategy {}));
    println!("Result: {}", context.execute(5, 3));

    context.set_strategy(Box::new(SubtractStrategy {}));
    println!("Result: {}", context.execute(5, 3));

    context.set_strategy(Box::new(Multplication {}));
    println!("Result: {}", context.execute(2, 4));
}

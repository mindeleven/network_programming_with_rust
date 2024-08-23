// cargo run --bin ex08_trait_example

/// traits are like blueprints for shared behaviors
/// traits define a set of methods that a type must implement to exhibit a certain trait
///
/// a trait is essentially a collection of method signatures w/o implementations

// defining a trait
trait Animal {
    fn make_sound(&self);
    fn eat(&mut self, food: &str);
}

// implementing the trait
struct Dog;

// to use the trait you need to implement it for specific types
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Wooooofffff!");
    }

    fn eat(&mut self, food: &str) {
        println!("The dog eats {}.", food);
    }
}

fn main() {
    println!("Rust's trait system: the secret sauce of reusability and flexibility");
}

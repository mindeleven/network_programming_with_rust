// cargo run --bin ex08_trait_example

/// traits are like blueprints for shared behaviors
/// traits define a set of methods that a type must implement to exhibit a certain trait
///
/// a trait is essentially a collection of method signatures w/o implementations

// defining a trait
trait Animal {
    fn make_sound(&self) -> String;
    fn eat(&mut self, food: &str);
}

// implementing the trait
#[allow(dead_code)]
#[derive(Debug)]
struct Dog {
    breed: String,
    name: String
}

// to use the trait you need to implement it for specific types
impl Animal for Dog {
    fn make_sound(&self) -> String {
        return "Wooooofffff!".to_string();
    }

    fn eat(&mut self, food: &str) {
        println!("The dog eats {}.", food);
    }
}

// using traits: polymorphism and generics
// polymorphism
//    -> writing code that works with any type the implements a particular trait
//    -> we don't need to know specific type at compile time
// example: feed_animal() function that is generic over any type T 
// that implements the Animal trait
fn feed_animal<T: Animal>(animal: &mut T, food: &str) {
    animal.eat(food);
}

fn main() {

    println!("### Rust's trait system: the secret sauce of reusability and flexibility ###");

    let mut my_dog = Dog {
        breed: String::from("German Shepherd"),
        name: String::from("Bismark")
    };

    dbg!(&my_dog);

    println!("My dog says: {:?}", my_dog.make_sound());
    feed_animal(&mut my_dog, "sausages");

}

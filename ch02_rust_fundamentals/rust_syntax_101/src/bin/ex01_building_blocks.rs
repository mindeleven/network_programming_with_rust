// cargo run --bin ex01_building_blocks

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("dedicated learner");
    let message = "Let's learn Building Blocks in Rust";
    println!("{}", message);

    let ip_address = "192.168.1.100";
    let port: u16 = 8080; // unsigned 16-bit integer
    let is_active = true; // boolean
    println!("{}, {}, {}", ip_address, port, is_active);
}

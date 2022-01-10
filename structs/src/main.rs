#[derive(Debug)]
struct RedFox {
    name: String,
    id: u64,
}

impl RedFox {
    fn run(self) {
        println!("Hi my name is {} and my id is {}", self.name, self.id);
    }
}
fn main() {
    let red = RedFox {
        name: "Swastik".to_string(),
        id: 23,
    };
    RedFox::run(red);
}

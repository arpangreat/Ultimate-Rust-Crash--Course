trait Noisy {
    fn get_noisy(&self) -> &str;
}

fn print_noise<T>(item: T)
where
    T: Noisy,
{
    println!("{}", item.get_noisy());
}

impl Noisy for u8 {
    fn get_noisy(&self) -> &str {
        "MEOW!"
    }
}

trait Run {
    fn run(&self) {
        println!("I am running");
    }
}

struct Robot {}
impl Run for Robot {}

fn main() {
    print_noise(5_u8);

    let robot = Robot {};
    robot.run();
}

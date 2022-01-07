fn main() {
    let array = [(1, 2), (3, 4)];

    for (x, y) in array.iter() {
        println!("The value of x and y is {} and {}", x, y);
    }
}

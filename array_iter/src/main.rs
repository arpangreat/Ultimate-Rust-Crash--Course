fn main() {
    let array = [(1, 2), (3, 4)];

    for (x, y) in array.iter() {
        println!("The value of x and y is {} and {}", x, y);
    }

    let mut s1 = String::from("Hello");
    println!("{:?}", do_stuff(&mut s1));
}

fn do_stuff(s: &mut String) {
    s.insert_str(0, "hi, ")
}

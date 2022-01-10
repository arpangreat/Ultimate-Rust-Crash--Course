use std::collections::HashMap;

fn main() {
    let mut v = vec![2, 6, 7, 0];
    v.sort();

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    let have_five = h.remove(&5).unwrap();

    println!("{:?}", v);
    println!("{}", have_five);
}

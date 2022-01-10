#[derive(Debug)]
enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

fn main() {
    let empty = DispenserItem::Empty;
    let ammo = DispenserItem::Ammo(69);
    let things = DispenserItem::Things("hat".to_string(), 89);
    let things1 = DispenserItem::Things(String::from("cap"), 90);
    let place = DispenserItem::Place { x: 32, y: 90 };

    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}",
        empty, ammo, things, things1, place
    );
}

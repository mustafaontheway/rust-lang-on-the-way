fn main() {

    let person1 = some_person_info("Ayhan Zirvede", 46, 3);

    println!("{:?}", person1) // (true, true, true)

}

fn some_person_info(name: &'static str, age: u8, nums_of_children: u8) -> (bool, bool, bool) {

    (name.to_ascii_lowercase().contains('a') || name.to_ascii_lowercase().contains('z'), age > 40, nums_of_children > 0)
}

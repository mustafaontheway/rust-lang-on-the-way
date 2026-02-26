use std::collections::HashSet;

fn main() {

    let mut years: HashSet<u16> = HashSet::new();

    years.insert(2000);
    years.insert(2000);
    years.insert(2000);
    years.insert(2000);
    years.insert(2000);
    years.insert(2002);

    println!("{:?}", years) // {2000, 2002}
}

use std::collections::HashSet;

fn main() {

    let mut years: HashSet<u16> = HashSet::new();

    years.insert(2000);
    years.insert(2000);
    years.insert(2000);
    years.insert(2000);
    years.insert(2000);
    years.insert(2002);
    years.insert(2007);
    years.insert(1997);
    
    println!("{:?}", years.len());

    println!("{:?}", years.contains(&1900));  

    years.remove(&1997);  

    println!("{:?}", years) // {2000, 2002}
}

// 4
// false
// {2000, 2002, 2007}

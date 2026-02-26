use std::collections::HashSet;

fn main() {

    let important_years: [u16; 8] = [2000, 2027, 2019, 2011, 1987, 1965, 2000, 2040];

    let set_imp_years = HashSet::from(important_years);

    let mut years: HashSet<u16> = HashSet::new();

    years.insert(2000);
    years.insert(2004);
    years.insert(2005);
    years.insert(2002);
    years.insert(2007);
    years.insert(1997);
    
    println!("Union -> {:?}", set_imp_years.union(&years));

    println!("Diff -> {:?}", set_imp_years.difference(&years));

    println!("{:?}", set_imp_years.symmetric_difference(&years));

    println!("{:?}", set_imp_years.is_disjoint(&years));

    println!("{:?}", set_imp_years.is_subset(&years));

}

// Union -> [2019, 2040, 2000, 1965, 2011, 2027, 1987, 1997, 2005, 2004, 2002, 2007]
// Diff -> [2019, 2040, 1965, 2011, 2027, 1987]
// [2019, 2040, 1965, 2011, 2027, 1987, 1997, 2005, 2004, 2002, 2007]
// false
// false

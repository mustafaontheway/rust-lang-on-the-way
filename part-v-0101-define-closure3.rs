fn main() {

    let current_year: u16 = 2026;

    let age_calculator = |birth_year: u16| {

        println!("Age is: {}", (current_year - birth_year) as u8)
    };

    age_calculator(1912) // Age is: 114
}

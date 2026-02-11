use std::fmt::Debug;

#[derive(Debug)]
enum CustomerTypes {
    Profitable(u32),
    Unprofitable
}

fn main() {

    customer_profit_rates("Ayhan Bilir", "Best");
    customer_profit_rates("Kağan Bilir", CustomerTypes::Unprofitable);
    customer_profit_rates("Sevda Bilir", 0.12);
    customer_profit_rates("Hakan Bilir", CustomerTypes::Profitable(6500));
}

fn customer_profit_rates<T>(name: &str, rate: T)

    where 
        T: Debug
{
    println!("Customer: {name} and last year sales profit rate: {rate:?}");
}

// Customer: Ayhan Bilir and last year sales profit rate: "Best"
// Customer: Kağan Bilir and last year sales profit rate: Unprofitable
// Customer: Sevda Bilir and last year sales profit rate: 0.12
// Customer: Hakan Bilir and last year sales profit rate: Profitable(6500)

// cargo run main.rs

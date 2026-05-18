fn main() {

    let sales_result1 = calculate_profit(7_560_000, 2_412_000);

    println!("{:?}", sales_result1); // Ok(5148000)

    println!("{:?}", sales_result1.unwrap_or_default()); // 5148000

    //println!("{:?}", sales_result1.copied().);

    let sales_result2 = calculate_profit(4_240_000, 6_500_000);

    println!("{:?}", sales_result2); // Err("To make a profit, sales must exceed costs!")

    println!("{:?}", sales_result2.unwrap_or_default()); //0
}

fn calculate_profit(sales_amount: u64, total_cost: u64) -> Result<u64, &'static str> {

    if sales_amount > total_cost {

        Ok(sales_amount - total_cost)
    } 

    else {

        Err("To make a profit, sales must exceed costs!")
    }
}

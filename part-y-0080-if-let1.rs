fn main() {

    let profit_year_2024 = CompanyYearlyResult::profit(440000);

    if let CompanyYearlyResult::profit(p) = profit_year_2024 {
        
        println!("Year 2024 profit amount: ${p}")
    }

}

// Year 2024 profit amount: $440000

enum CompanyYearlyResult {

    profit(u32),
    neutral,
    loss(i32)
}

use std::collections::HashMap;

fn main() {
     
    let mut user_eth_balances: HashMap<&str, f32> = HashMap::new();

    user_eth_balances.insert("ab004269", 3.12);

    user_eth_balances.insert("ku009847", 4.25);

    let user_aybuke_balance = user_eth_balances.get("al004269");

    println!("{user_aybuke_balance:?}"); // None

    let user_ayhan_balance = user_eth_balances.get("ab004269");

    println!("{user_ayhan_balance:?}"); // Some(3.12)

    // let Some(&user_ayhan_balance_actual) = user_eth_balances.get("ab004269") else {
    //     println!("Undefined user balance...");
    //     return;
    // };

    // let user_ayhan_balance_actual = user_eth_balances.get("ab004279").unwrap_or(&0.0);

    // println!("Some math & deref: {:?}", *user_ayhan_balance_actual + 3.45); // Some math & deref: 3.45

    let user_ayhan_balance_actual = user_eth_balances.get("ab004279").copied().unwrap_or(0.0);

    println!("Some math & deref: {:?}", user_ayhan_balance_actual + 13.45); // Some math & deref: 13.45

}


//cargo run

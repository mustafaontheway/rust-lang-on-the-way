fn main() {

    let discount1: Option<u16> = Some(1000);

    let mut discount2: Option<u16> = None;

    let mut total_basket_amount: u16 = 20000;

    if let Some(d) = discount1 {

        total_basket_amount -= d;

        println!("You'll pay: {total_basket_amount} ₺")
    }

    if let Some(d) = discount2 {
        
        total_basket_amount -= d;

        println!("You'll pay: {total_basket_amount} ₺")
    }

    discount2 = Some(800);

    if let Some(d) = discount2 {
        
        total_basket_amount -= d;

        println!("You'll pay: {total_basket_amount} ₺")
    }

}

// You'll pay: 19000 ₺
// You'll pay: 18200 ₺

fn main() {
     
    let mut social_payments = vec![770u16, 950, 440, 800, 350, 640, 390];

    for p in &mut social_payments {

        if *p < 500 {

            *p += 100;
        }
    }

    println!("Updated social payments list: {:?}", social_payments)
}

// Updated social payments list: [770, 950, 540, 800, 450, 640, 490]

fn main() {

    let mut seconds: u8 = 0;

    loop {

        seconds += 5;

        if seconds == 20 {

            println!("We don't like number '20'. Pass it!");

            continue
        }

        println!("Seconds: {seconds}");

        if seconds == 60 {

            println!("One minute complited!");

            break
        }
    }
}

// Seconds: 5
// Seconds: 10
// Seconds: 15
// We don't like number '20'. Pass it!
// Seconds: 25
// Seconds: 30
// Seconds: 35
// Seconds: 40
// Seconds: 45
// Seconds: 50
// Seconds: 55
// Seconds: 60

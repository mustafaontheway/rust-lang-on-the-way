fn main() {

    let greet = String::from("Hi there!");

    let greet_raw_pointer1 = &raw const greet;

    println!("{greet_raw_pointer1:?}");

    let greet_raw_pointer2: *const String = &greet;

    println!("{greet_raw_pointer2:?}");
}

// random address values and they can change when you run the codes

// 0xd76faffb50
// 0xd76faffb50

fn main() {
  
    let add_one = |x: i32| x + 1;

    println!("{}", add_one(5)); // 6

    let mult_positive_five = |a: u64| a * 5;

    let m1 = mult_positive_five(5_550_000);

    println!("Mult result: {m1}") // Mult result: 27750000  
}

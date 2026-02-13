fn main() {

    let mut sentence = "She is awesome".to_string();

    exclamation_marks(&mut sentence);

    println!("{}", sentence); // She is awesome!!!
}

fn exclamation_marks(s: &mut String) {

    s.push_str("!!!");
}

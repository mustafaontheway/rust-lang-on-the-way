fn main() {

    let sentence = "Welcome! How are you?";

    let new_vec = elements_to_vec(sentence);

    println!("New vector with using collect method: {:?}", new_vec); 

    println!("New vector without using collect method: {:?}", elements_to_vec_without_collect(sentence));

}

fn elements_to_vec_without_collect(s: &str) -> Vec<&str> {

    let mut splitted = Vec::new();

    for part in s.split('!') {

        splitted.push(part);
    }

    splitted
}

fn elements_to_vec(s: &str) -> Vec<&str> {

    s.split('!').collect()
}



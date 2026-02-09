fn main() {

    let emp_data = (("ayhan bilir", 26, true), ("bengü kağan", 35, false));

    let (name, age, married) = emp_data.0;

    println!("His name is {name}");

    let (name, _, _) = emp_data.1;

    println!("Her name is {name}");
}

// His name is ayhan bilir
// Her name is bengü kağan

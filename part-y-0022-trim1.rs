fn main() {

    let sentence = "              A rolling stone gathers no moss!   ";

    println!("{}", sentence.len());

    let s1 = sentence.trim();

    println!("{}", s1.len()); // 32

    let s2 = sentence.trim_start();

    println!("{}", s2.len());

    let s3 = sentence.trim_end();

    println!("{}", s3.len());

}

// 49
// 32
// 35
// 46

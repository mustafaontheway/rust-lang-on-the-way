use std::fs::File; // fs -> file system

fn main() {

    let file_content1 = File::open("../dummy.txt").expect("Not a such file or wrong path...");

    println!("{:?}", file_content1);

    //File { handle: 0xa8, path: "\\\\?\\D:\\_rust\\rust-again\\hi\\dummy.txt" }

}



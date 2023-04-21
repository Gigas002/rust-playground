use std::fs::File;

fn main() {
    // panics

    // panic!("crash and burn");
    let v = vec![1, 2, 3];

    // v[99];

    // recoverable errors
    let greeting_file_result = File::open("hello.txt");
}

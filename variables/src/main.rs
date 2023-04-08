
// const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 //this is expression and in doesn't need semicolon
    };

    println!("The value of y is: {}", y);
}


fn another_function(x: i32) {
    println!("X value is:{}", x);
}

fn five() -> i32 {
    5
}

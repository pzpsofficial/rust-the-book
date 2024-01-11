// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = five();
    let z = plus_one(100);
    another_function(z, 'b');
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
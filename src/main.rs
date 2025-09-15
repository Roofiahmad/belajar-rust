fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello Test")
}

#[test]
fn rust_variable(){
    let mut name ="Roofi Ahmad Sidiq";
     println!("Hello {}", name);
    name = "Fatah";

    let age:u8 = 29;
    println!("Hello {}, your age is {}", name, age);

    let floating_number:f32 = 3.1231232;
    println!("{}", floating_number);

    const BIRTH_YEAR : u16 = 1996;
    println!("My birth year is {}, your age is {}", BIRTH_YEAR, age);

    println!("{}", BIRTH_YEAR as u8);
}

#[test]
fn shadowing(){
    let random_var: &str = "I am Roofi";
    println!("{}",random_var);

    let random_var : i32 = 999;
    println!("{}",random_var);
}

#[test]
fn rust_boolean(){
    let age = 29;
    if age >30 {
        println!("You're too old");
    }else {
        println!("You're not that old, enjoy your life buddy");
    };

    if age != 0 {
        println!("You have a age, awesome");
    }

    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);

    let is_old = if age >50 {"Yes you're old"} else {"You're still young and on fire"};
    println!("{}", is_old);
}

#[test]
fn rust_match(){
    let color : &str = "orange";

    match color {
        "white" | "orange" => println!("bright and shining"),
        "black" | "grey" => println!("dark and gorgeous"),
        _ => println!("such as awesome color"),
    };

    let expression = match color {
        "white" | "orange" => "bright and shining",
        "black" | "grey" => "dark and gorgeous",
        _ => "such as awesome color",
    };
    println!("Your clothes is {}",expression)
}

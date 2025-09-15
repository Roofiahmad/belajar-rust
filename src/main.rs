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

    let age:i32 = 29;
    println!("Hello {}, your age is {}", name, age);
}

#[test]
fn shadowing(){
    let random_var = "I am Roofi";
    println!("{}",random_var);

    let random_var = 999;
    println!("{}",random_var);

}
// fn main() {
//     // let mut x:i32 = 1;
//     // x+= 2; 
    
//     // assert_eq!(x, 3);
//     // println!("Success!");
//     define_x();
// }

// fn define_x(){
//     let x: &str = "Hello world!";
//     println!("THIS IS {}",x);
// }`

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // we can make it mutable 
    let mut x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}
fn main() {
    // let mut x:i32 = 1;
    // x+= 2; 
    
    // assert_eq!(x, 3);
    // println!("Success!");
    define_x();
}

fn define_x(){
    let x: &str = "Hello world!";
    println!("THIS IS {}",x);
}
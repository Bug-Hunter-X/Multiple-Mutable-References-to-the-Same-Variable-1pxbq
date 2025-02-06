fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10;
    println!("{}", x);
    //Here we have only one mutable reference at a time
    let z = &mut x; 
    *z = 100; 
    println!("{}", x);
}
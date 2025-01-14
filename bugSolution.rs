fn main() {
    let mut x = 5;
    { //Limit scope of mutable reference
        let y = &mut x; 
        *y = 6; 
    }
    { //Limit scope of mutable reference
        let z = &mut x; 
        *z = 7; 
    }
    println!("x = {}", x); 
}
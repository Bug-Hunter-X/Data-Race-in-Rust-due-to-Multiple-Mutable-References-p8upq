fn main() {
    let mut x = 5;
    { // Limiting the scope of the mutable borrow
        let y = &mut x; 
        *y = 10; 
    }
    { // Limiting the scope of the mutable borrow 
        let z = &mut x; 
        *z = 15; 
    }
    println!("x = {}", x);
}
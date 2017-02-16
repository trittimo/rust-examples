struct Square {
  width: i32,
  height: i32,
  
  // CAN'T DO THIS!
  // mut area: i32

  // mutability is a property of the binding!
}

struct Color(i32, i32, i32);

#[allow(dead_code)]
pub fn main() {
  println!("STRUCTS...");

  
  let mysquare = Square { width: 5, height: 5 };
  println!("Width: {}, Height: {}", mysquare.width, mysquare.height);


  let black = Color(0, 0, 0);
  println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);

  // Mutability property of the binding
  let mut black = black;
  black.0 = 5;
  println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);
}
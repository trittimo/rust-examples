struct Square {
  width: i32,
  height: i32
}

// Be careful how you pass things!
fn get_area(square: &Square) -> i32 {
  return square.width * square.height;
}

impl Square {
  fn new(w: i32, h: i32) -> Square {
    Square { width: w, height: h }
  }

  // Just syntactic sugar
  fn get_area(&self) -> i32 {
    return self.width * self.height;
  }

  fn set_width(&mut self, w: i32) {
    self.width = w;
  }
}

#[allow(dead_code)]
pub fn run() {
  println!("FUNCTIONS...");


  let mysquare = Square { width: 5, height: 5 };
  let area = get_area(&mysquare);
  println!("The area of mysquare is: {}", area);


  // Use the functions declared on the square struct
  let mut new_square = Square::new(15, 15);
  println!("The area of the new square is: {}", new_square.get_area());

  // Note that we can re-use the new_square because we passed the
  // functions a reference rather than the square directly
  new_square.set_width(30);
  println!("The width of our new square is now: {}", new_square.width);
}
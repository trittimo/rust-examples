#[allow(dead_code)]
pub fn run() {
  println!("VARIABLES...");
  let myvar = 3;

  // The below line is a compile time error -- uncomment to see Rust's nice error messages
  // myvar += 1;

  // Interestingly, we can rebind on the fly
  let mut myvar = myvar;

  // Not an error anymore
  myvar += 1;

  println!("myvar += 1 -> {}", myvar);
}
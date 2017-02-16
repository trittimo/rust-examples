#[allow(dead_code)]
fn destroy(val: String) {
  // Here we've moved val into a new scope
  println!("Destroying reference to value: {}", val);

  // val is about to 'die' because it's going out of scope
}

#[allow(dead_code)]
fn no_destroy(val: &String) {
  println!("Just a reference to val, no destroying here: {}", val);
}

#[allow(dead_code)]
pub fn main() {
  println!("OWNERSHIP...");

  // In order to prevent use after free errors, rust has taken a hard stance on how and when
  // you can use your variables
  let myvar = String::from("Hello world");
  // Since we've moved myvar into a new variable, we are unable to use it later
  // This prevents us from having two of the same 'object'
  // let mynewvar = myvar;
  // println!("mynewvar is {}", mynewvar);
  
  println!("myvar after moved into new variable: {}", myvar);


  // The same thing happens with functions
  // destroy(myvar);
  // println!("myvar after it was destroyed by 'destroy': {}", myvar);


  // We can avoid this problem by using references
  // let ref myrefvar = String::from("My ref var!");
  // {
  //   let mylocalref = myrefvar;
  //   println!("I don't own myrefvar: I just have a reference to it! {}", mylocalref);
  // }
  // println!("myrefvar after it was used in a new scope: {}", myrefvar);

  // no_destroy(myrefvar);
  // println!("myrefvar after it was used by no_destroy: {}", myrefvar);

  // The rule is that we can have any number of immutable references
  // at any given time
  // However, there can not be any immutable references if there is a single
  // mutable reference, and there is only one mutable reference allowed at
  // any given time
}
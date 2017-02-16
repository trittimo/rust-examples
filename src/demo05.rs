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
fn main() {
  println!("OWNERSHIP...");

  // In order to prevent use after free errors, rust has taken a hard stance on how and when
  // you can use your variables
  let myvar = String::from("Hello world");
  {

    // Because we've used myvar in this new scope, it's permanently bound to this scope (unless we explicitly give it back)
    // This means that myvar will be permanently dead after the end of this block
    let myvarinst = myvar;
    println!("I own myvar now: {}", myvarinst);

    // Goodbye myvar!
  }

  // println!("myvar after moved into new scope: {}", myvar);


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
}
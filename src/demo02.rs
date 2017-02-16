#[allow(dead_code)]
pub fn run() {
  println!("REFERENCES...");


  // Here we have a mutable reference (i.e. we can change what the reference is looking at)
  // A reference is just a pointer
  let myvar = &mut 42;
  
  // We can also provide type hinting, but it's not usually necessary (except in functions)
  let myvar: &mut i32 = myvar;

  // We can dereference the pointer using *
  *myvar = 7;

  println!("myvar is now: {}", myvar);


  // Here's the same thing, but maybe a little more clear what's going on
  let ref mut my_other_var = 3;

  *my_other_var = 4;
  println!("my_other_var is now: {}", my_other_var);
}
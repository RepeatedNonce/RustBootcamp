// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
   let f = first_name();
   let l = last_name();
  println!("The {} name and the {} name",f,l );
}

fn first_name() -> &'static str  {
  return "First";
}

fn last_name() -> &'static str{
  return "Last";
}

 

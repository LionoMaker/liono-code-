fn main() {
   let tup:(i32,i32,char)=(850,1100,'A');  // this is how to define tuple in rust
   let obtained_marks=tup.0;               // approaching values of tuple e.g. 0 belongs to 850 and so on
   let total_marks=tup.1;                  // tup.1 approaches 1100
   let grade= tup.2;                       // tup.2 approaches 'A'

   // let's start printing one by one 

   println!("student obtained_marks are: {}",obtained_marks);  // prints obtained marks
   println!("student total_marks are: {}",total_marks);          // prints total marks defined in tuple
   println!("student Grade is:{}",grade);                     // prints grade 
   

   // to print them open "new terminal " ,just like below window, called terminal.
    // in terminal window we are actually write our command and run/check and test our code.
         // firsty save your code   by pressing "ctrl+s"
}

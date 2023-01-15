fn main() { //fn  to say this is a function
    let  first_var = 10;// let  to declare any tupe of var as const can t be change
    let mut second_var =30 ; //var not const  value can change in the code , : i32 mean a 32 bit  signeed  int  or u  for unsigned  8 16 32 ..
    let floating_var : f64 =127.257; //64 or 32
    println!("Hello, world! \n this is x {} and y is {} and float is {}",first_var,second_var,floating_var); // {} place holder to print the value of x 
    second_var=50 ;
    println!("Hello, world! \n this is x {} and y is {}",first_var,second_var);
    let c = first_var as f64 /floating_var;  //castin  int to float
    println!(" {} ", c);

}

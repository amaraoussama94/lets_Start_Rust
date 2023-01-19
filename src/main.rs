fn main() { //fn  to say this is a function
    let  first_var = 10;// let  to declare any tupe of var as const can t be change
    let mut second_var =30 ; //var not const  value can change in the code , : i32 mean a 32 bit  signeed  int  or u  for unsigned  8 16 32 ..
    let floating_var : f64 =127.257; //64 or 32
    println!("Hello, world! \n this is x {} and y is {} and float is {}",first_var,second_var,floating_var); // {} place holder to print the value of x 
    second_var=50 ;
    println!("Hello, world! \n this is x {} and y is {}",first_var,second_var);
    let c = first_var as f64 /floating_var;  //castin  int to float
    println!(" {} ", c);// using  default  format
    println!(" {:08.3} ", c); //3 digit of precision, totale of 8 caractere 
    print!(" {0:08.3}  not  {1}", c,first_var); //ln for  new line every var  has  position starting  from 0 using this position you can prin t it many tima in ssame line 
    let _first_char = 'a';//create caractere 
    let  finger = '\u{261D}';// use  symbole
    println!("{}",finger);
    //array

    let first_array =['a','b','c'];
    println!("{}",first_array[0]);

    let mut first_array =['a','b','c'];//now we can change element of array
    first_array[0]= 'x';
    println!("{}",first_array[0]);

}

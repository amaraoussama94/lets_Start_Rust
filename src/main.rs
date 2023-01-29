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
   
    let second_array: [i32 ;5] ; //array of 5 integer 32int
   // second_array = [1,2,3,4,5] ; //init the array 
    second_array =[0;5]; // init it with number from 0 to 4
    let length = second_array.len();//get the array length 
    println!("{}",length);

    let twodim_array=[[1,2,3],
                [4,5,6]]; //array of 5 integer 32int
    println!("{}",twodim_array[0][1]);

    //tuple
    let stuff =('a',1,13.54);// can be diff  tye of element 

    println!("{}",stuff.0); //print first element  stuff.1 1st .....
    let mut  stuff2 =('a',1, 13.54);// can be diff  tye of element 
    stuff2.0 ='c';
 

    let(a,b,c) =stuff2; // a= 'a' , b=1 c= 13.54 

    println!("{}  {}  {}",a,b,c);
    say_hello(1,'c');
    let x =square (2);
    println!("{}",x);
    let y = square2(3);
    println!("{:?}",y); //this how to print tuples 

    //if cond 
    if first_var == 10
    {
        println!(" first  condition ");
    }
    else  //elswe if ()
    {
        println!(" second  condition ");
    }
    //condition assignement
    let condition  = true ; 
    let rust_var = if condition {5} else {256};
    println!("{}" , rust_var) ;
    // loops 
    //infinit 
    let mut count =0;

    let result =loop {   //assign the return value aka count  to result 
        count +=1;
        println!(" count value is {}",count);
        if count > 20 
        {
            break count;//end loop //return count  when the loop end
        }
        }; // need ;  as its an expression not statement 

        /*loop {   //assign the return value aka count  to result 
        count +=1;
        println!(" count value is {}",count);
        if count > 20 
        {
            break count;//end loop //return count  when the loop end
        } */
println!(" result = {}",result);
//while loops
let mut var_count =0;
while var_count < 10 
{
    var_count += 1;
    println!(" {}",var_count);
}

let mut i =0;
while i < first_array.len()
{
    
    println!("first_array ;{}",first_array[i]);
    i += 1;
}

//for loop
//for item in first_array 
for (index,item)in first_array .iter().enumerate()//iterator  //.iter().enumerate() to return a tule of  index and elemnt 
{
    println!("for loop ;{}  in postion {}",item,index);
}

for i in 0..5
{
    println!(" hello num {}",i);
}
//shadowing var 
let planet =" earth";
println!(" the planet is {}",planet);
let planet =" march";
println!(" the planet is {}",planet);

//var sco and  shadowing 

{
    let mut planet ="jupiter";
    println!(" the planet is {}",planet);
}
println!(" the planet is {}",planet);
}


fn say_hello( number : i32 , name :char)// var : type
{
    println!("hello {} {}",name,number);
}


fn square (x : i32)-> i32   //-> i32 for return value  
{
    x*x  //make it express mean no ; so th value pass as retun value 
    //return x*x ; other way 

}

fn square2 (x : i32)-> (i32,i32)   //return 2value 
{
    
    return (x,x*x );// return 2 val

}
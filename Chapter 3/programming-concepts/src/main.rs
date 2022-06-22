fn main() {
    // variables and immutability

    let mut x:i32 = 6;
    x=5;
    println!("Hello, world!, {}", x);

    // Shadowing - lets you declare a new varaible with same name name as previous variables
    // it involves transforming a varible from previous state
    let  y = 5;
    let y = y + 1;
    println!("Shadowing var {}",y);

    /* Scalar types contains only a single value; integer,bool,float, character,
        Compound types - tuples and arrays
    */

    //Tuples

    let tup:(i32,f32,i32) = (4,5.0,6);
    println!("Tuple {}", tup.0);
    // destructuring tuple
    let (x,y,z) = tup;
    println!("Destructure tup y {}",y);


    // Arrays arrays are fixed in rust accepting only asingle type 
    let arr = [1,23,4,4];
    println!("first index array {}", arr[0]);

    // fUNCTIONS 
    anotherFunction(5);
    //functions with return value
    let five = five();
    println!("five {}",five);

    //CONTROL FLOW STATEMENT
    let a = 3;

    if a >2 {
        println!("if control - a >2  {}" , a);
    }
    else {
        println!("if control - a less than or equal 2  {}" , a);
    }


    // Let if
    let val = if true {
        10
    }
    else {
        30
    };

    println!("let if {}",val);

    //..LOOPS

    // loop{
    //     println!("infinte loops")
    // }

    let mut num = 3;

    while num !=0 {
        println!("Loop - {}", num);
         num = num - 1;
    }

    //Loop through array

    for elem in arr.iter() {
        println!("val {}",elem);
    }

    // for in with range
    for elem in (1..4).rev(){
        println!("rev val {}",elem);
    }

    //excercise 
   let res = fib(5);
   println!("fib res {}",res);
}


fn anotherFunction(x:i32){
    println!("ANother function {}",x);
}

// dont add semicolon to return value;
fn five() -> u32{
    5
}

//Execrcise Fibonnaci 
fn fib( n:u32)-> u32{
    let mut n1= 0; let mut n2=1;
    let mut nt =0;

    match n {
        0 => 1,
        1=>1,
        _ => {
             //fib(n-1) + fib(n-2)
            for _e in (1..n).rev() {

                // println!("n1 - {}",n1);
                // println!("n2 - {}",n2);
                // println!("nt - {}",nt);
               
                nt = n1 + n2;
                n1 = n2;
                n2 = nt;
                
              
            }
        
             nt
        }
    }


    
   
   
}
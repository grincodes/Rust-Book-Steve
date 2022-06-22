fn main() {
    /* Generics are abstracts stand ins for concrete types*/

    // Generics in functions

    
    fn st<T>(list:i32)->i32{
        1
    }

    //Generics in structs
    struct Point<T>{
        x:T,
        y:T
    }

    //Generics in Enum
    enum Option<T>{
       Some(T),
        None
    }

    enum Result<T,E>{
        Ok(T),
        Err(E)
    }

    //Generics in Method Definition

    impl<T> Point<T> {
        fn x(&self) -> &T{
            &self.x
        }
    }

    let p = Point{x:3,y:4};
    println!("p.x = {}", p.x());

    /* TRAITS  Defining shared behavior  also callled Interfaces in other language 
        way to group set of behaviors that achieve a particular purpose 
    
    */


    pub trait Summary {
        fn summarize(&self)->String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    //implementing traits
    impl Summary for  NewsArticle {
        
        fn summarize(&self) -> String { 
            format!("{}, by {} ({})", self.headline, self.author, self.location)
         }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
       }
       impl Summary for Tweet {
        fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
        }
       }


       let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
       };
       println!("1 new tweet: {}", tweet.summarize());

       // defualt trait implementation 
       pub trait Summarize {
           fn Summarize(&self)->String {
               String::from("(Read More ...)")
           }
       }

       // trait could have multiple methods related
       pub trait Summary_ {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
        }
       }

       /* Traits Bonds is traits on generic type 
            Traits Bonds help Rust know method signature and 
            appropraite return type of Generictypes 
            ahead before run time, it gives idea of
            the type the abstract generic is most likely to be 

            its as simply as <T:Trait>

            we can specify multiple trait using the + sign
            <T: Trait + AnotherTrait>
       
       */

       //PartialOrd is a trait FOR COMPARING STRING 
      fn largest<T: PartialOrd>(list:&[T]) -> &T {
        let mut large = &list[0];
        for item in list.iter()   {
            if item > large {
                large = item
            }
        }

        &large
    }

    fn largest_num <T:PartialOrd + Copy> (list:&[T]) -> T{
        let mut large = list[0];
         for &item  in list.iter(){
             if item > large {
                 large = item;
             }
         }
         large
    }

    // Specifying multiple traits
    fn some_function<T, U>(t: T, u: U) -> i32{
        let test:i32=4;
        test
    }

    // USING TRAITS BOUNDS TO CONDITIONALLY IMPLEMENT METHODS

    use std::fmt::Display;
    struct Pair<T>{
        x:T,
        y:T,
    }

    impl<T> Pair<T>{
        fn new(x:T,y:T) ->Self{
            Self{
                x,y,
            }
        }
    }

    // the compare method can only work if T is of PartialOrd and Display type

    // fn some_function<T, U>(t: T, u: U) -> i32
    // where T: Display + Clone,
    // U: Clone + Debug{}

    impl<T:PartialOrd + Display > Pair<T> {
        fn comp_display (&self) {
            if self.x >= self.y {
                println!("The largest number is {}",self.x)
            }
            else{
                println!("The largest number is {}",self.y)
            }

        }
    }

    // this will cause error because collection deosnot implement PartialOrd and Display
    //let s = Pair::new(&[4],&[5]);
    let s = Pair::new(4,6);
    s.comp_display();



    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    let res = largest_num(&number_list);
    println!("The largest number is {}", result);
    println!("The largest number is {}", res);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    // VALIDATING REFERNCE WITH LIFETIMES
    /*
     * lifetimes help the compiler to dtermine if a refernce is out of scope
     * 
     * lifetime anonotations dont change the lifetime of a reference
     * 
     */


     // throws error about lifetime , the compiler cant determine the lifetime and it needs 
     // our help
    // fn largist (x:&str,y:&str)->&str {
    //         if x.len() > y.len() {
    //             x
    //         }
    //         else{
    //             y
    //         }
    // }

    fn largist<'a> (x:&'a str,y:&'a str)->&'a str {
            if x.len() > y.len() {
                x
            }
            else{
                y
            }
    }

    // example of the advantages of lifetime annotation it helps spot dangling 
    //refernces at before compile time , x here is out of scope
    // {
    //    let x = "mad over your in this life";
    // }

    // let y = "ok , na make we see ";
    // largist(x,y);

    // correct guy
    let x = "mad over your in this life";
    let y = "ok , na make we see ";
    largist(x,y);


    // Lifetimes in struct

    struct Engine <'a>{
        version: &'a str,
    }

   let e =  Engine {version: "v6 bugatti"};

   //Life time illuesion rules 
   /*
        This rule tells us when we dont need to add lifetime to refernces as rust will add them for us

        Rule 1:
        when the method contains one refernce  the output ref is automatically set to the lifetime of 
        input ref 
        when there are two refs parms in a method and one is &self ; self is applied to all refs
        
   */

  //Lifetime for methods

  impl<'a> Engine <'a>{
    fn level(&self)->i32{
        3
    }
  }

  //STATIC LIFETIMES  for the entire duration of the program
  //The text of this string is stored directly in the binary of your program,
  //which is always available. Therefore, the lifetime of all string literals is 'static.
  
  let s: &'static str = "I have a static lifetime.";




   

}

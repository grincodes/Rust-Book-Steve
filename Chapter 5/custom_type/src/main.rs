fn main() {
    /*
      Defining  Struct
    */

    
    #[derive(Debug)]
    struct User {
        name: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }
    let user1 = User{
        name: String::from("bola ige"),
        email:String::from("boalige@gmail.com"),
        sign_in_count:4,
        active:true
    };

    println!("Active {:?}", user1);

    fn create_user(name:String,email:String)->User{
        User {
            name,
            email,
            sign_in_count:5,
            active:false
        }

    }

    let mut user2 = create_user(String::from("sola"),String::from("solamail"));
    user2.email = String::from("sola@email.com");
    println!("{}",user2.email);

    // createuser from another user property
    let user3= User {
        ..user2
    };

    println!("user 3 {:?}", user3);


    //Custom tupel defining tuples with name
    struct Color(i32,i32,i32);

    let black =Color(0,0,0);
    println!("{}",black.0);

    #[derive(Debug)]
    struct Rect {
    height:u32,
    width:u32
    }

    fn Area (rec:&Rect)->u32{
            rec.height * rec.width
    }

    let rec = Rect {height:3,width:4};
    let area = Area(&rec);
    println!("Area {}",area);

    impl Rect {
        fn area(&self)-> u32{
            self.height * self.width
        }
        fn can_hold(&self,other:&Rect)-> bool{
                self.width > other.width && self.height > other.height
        }
    }

    println!("CAn hold {}" , rec.can_hold(&Rect{height:2,width:2}));

    //ASSOCIATED FUNCTION OR STATIC METHODS
    // we can aso have multiple implementations

    impl Rect {
        fn Square(size:u32)->Rect{
            Rect {width:size,height:size}
        }
    }

    println!("square {:#?}", Rect::Square(3));

}

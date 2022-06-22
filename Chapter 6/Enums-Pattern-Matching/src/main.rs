fn main() {
    /*
         Enumerations (Enums) allows you to adefine a type by
         enumaerating its possible values
    */

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    // get val of enums
    let four = IpAddrKind::V4;

    println!("val enum four {:?}", four);

    // enums in action

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let local = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.01"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // you can return value of enums
    #[derive(Debug)]
    enum IpAddrE {
        V4(String),
        V6(String),
    }
    let elocal = IpAddrE::V4(String::from("127.0.0.1"));

    println!("elocal {:?}", elocal);

    enum IpAddrs {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrs::V4(127, 0, 0, 1);
    let loopback = IpAddrs::V6(String::from("::1"));

    // enums for variants of the same type
    // methods can also be implemented on enum
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", self)
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    //OPTION ENUM
    // rust has no null type so it uses
    //option is part of the lib
    // enum option<T>{
    //     Some(T),
    //     None
    // };
    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent_number: Option<i32> = None;

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // using match as a control flow operator
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("state from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(x) => Some(x + 1),
            None => None,
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // marching all  variants of enums can be tiring we can use
    /* (_) called placeholder as default in switch then specify the needed values 
        here we are only intreseted in number 3 from all the possible u8 values
        every other vvalue are matched to the default or olaceholder
    
    */
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    /* match can be ehaustive if let lets us match to just one possible value 
        you also loose the exhaustive type checking that match provides
    */
    if let Some(3) = some_u8_value {
        println!("3")
    }
    else{
        println!("not 3")
    }

    /*Boiler plate code
        
        //match
        match case {
            _ => ()
        }

        // if let 
        if let match_branch = var_to_match {

        }

        else {

        }
        
    
    */


}

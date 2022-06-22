fn main() {
    // error s1 has passed ownership to s2 /or has moved all
    // its propertires to s2 and it has moved out of scope hence the error
    // let  s1 = String::from("hello");
    // let s2 =s1;
    // println!("{}, world!", s1);

    /*
        clone is a temporal solution to the above issue or error
        clone makes an exact copy of the sring variable on the heap
        this can be usually expensive for high data varaibles
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // OWNERSHIP AND FUNCTIONS

    let s = String::from("hello");
    takes_ownership(s); //s value  MOVES into this function and is no longer valid
    let x = 5;
    makes_copy(x); // x moves in to the function but since its i32 and on stack it has aC COPY so its fine

    // trying to use s after wards to see if its still available; it returns an error
    // println!("{}",s);

    /*
      Solutions:
      RETURNING VALUES CAN TRANSFER OWNERSHIP BACK FRM FUNCTION TO MAIN CODE
      USING REFERENCE AND BORROWING

    */

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // creating mutable reference

    let mut s = String::from("hello");
    change(&mut s);

    //Lifetime

    // slices
    // allows you to refernce a contigous block of memory in collection without referncing the whole collection

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{:?}", world);

    // it makes more sense to define a function that accepts string slice rather than string
    // string slice van also be used as string and it makes the function more robost
    // fn takeString( &str) rather than fn takeString(s:&String)

}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" ,world");
}

fn take_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s:&String)-> &str {
    let bytes = s.as_bytes();

    for (i,&item) in  bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..];
}

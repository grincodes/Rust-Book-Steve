use std::collections::HashMap;

fn main() {
    /*
        Data Collections vectors hasp maps strings
    */

    // Vectors store collection of values of the same types
    // they are better thatn array cos you dont need to knw
    // the amount of data before instantiation and they
    // can grow or shrink
    let vec: Vec<i32> = Vec::new();

    let vv = vec![4, 5, 6, 6, 7];

    // updating a vector
    let mut _vec = Vec::new();
    _vec.push(4);
    _vec.push(3);
    _vec.push(5);

    // reading elements vec
    /*
        there are two ways to reference the val stored in a vector
        and both ways have different return types
        one uses square notatino and index and returns a ref to the position

        the other one is through get option and it returns an option of
        reference to vector value
    */

    // let res: &i32 = &vec[2]; // where u use this method  to call out of index this method panic and crash out
    let res2: Option<&i32> = _vec.get(2); // when you use this method to call out of index it returns None

    // iterating over vec
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * deferencing operator to get the actual value of a trfrtnvr ny fr
    }


    //Using Enums to store multiple types 
    /*
        initially we said vectors only store a single type , but we can make them
        store multiply types by using Enums

    */

    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![SpreadSheetCell::Int(3),SpreadSheetCell::Float(4.0),SpreadSheetCell::Text(String::from("hello"))];


    /* Strings 
        to_string,String::from()

        push_str to add string
        format
        .chars(),.bytes(),
    */
    let data = "hello string";
    let mut s = data.to_string();
    s.push_str(" additon ");
    println!("to string - {}",s );

    /* Hash maps : storing key and value pairs */
     
    let mut score: HashMap<_,_> = HashMap::new();

    score.insert(String::from("blue"),30);
    score.insert(String::from("yellow"),20);

    /*
        Hash Maps and ownerships
        Hashmaps use the copy trait to copy ints passed 
        but it moves or takes ownership of strings
    */

    let field_name = String::from("favorite Color");
    let field_value = String::from("Blue");

    let mut map : HashMap<_,_> = HashMap::new();

    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point,
    // println!("{}", field_name);

    // ACCESSING VALUE IN HASHMAP using get
    // get() returns  Option<&V>
    let team_name = String::from("blue");
    let sc = score.get(&team_name);
    
    match sc {
        Some(v)=> println!("gotten {}",v),
        _ => println!("None VAl")
    }
    

    // iteration over hashmap

    for (k,v) in &score {
        println!("{} - {}", k, v)
    }




    /*
        uPDATING HASHMAP - replacing the old value 
        or the new value , checking if the key exists
    */
    score.insert(String::from("blue"),50);
    println!("{:?}", score);

    /*
        Check if  a key exist
        before insert
        .enty(key).._or_insert(value)
    */

    score.entry(String::from("yellow")).or_insert(50);

    //updating a value base on old value

    let text =  "hello world wonderfuk world";

    let mut  map :HashMap<_,_> = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        // the above function returns a reference to value of particular hash map key
        // the deferncing operator is used ot change its value

        /*
            The or_insert
            method actually returns a mutable reference (&mut V) to the value for this
            key. Here we store that mutable reference in the count variable, so in order to 
            assign to that value, we must first dereference count using the asterisk (*). The
            mutable reference goes out of scope at the end of the for loop, so all of these
            changes are safe and allowed by the borrowing rules.
         */ 
        *count +=1;

    }

    println!("{:?}", map);

    

}

use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {
    /*
    Rust has two categories of error
    recoverable and non recoverable
    For a recoverable error, such as a file not found error, it’s reasonable
     to report the problem to the user and retry the operation. Unrecoverable
     errors are always symptoms of bugs, like trying to access a location beyond
     the end of an array

     Result<T,E> for recoverable error ; panic! for non recoverable
     */

    // panic!("crash and burn");

    //Recoverable Error
    // enum Result<T, E> {
    //     Ok(T),
    //     Error(E),
    // }

    let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    let f = match f {
        Ok(file) => file,
        // The is statment after ref error is called match guard
        //This condition must be true for that arm’s code to be run otherwise the next arm code will run 
        // the ref is used insted of & cos ref matches value and returns a refrence  while & matches reference 
        // and returns value
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    /* 
        Shortcuts for panic on Error  : unwrap(), expect()
        Result<T,E>  has some helper functions like unwrap and expect
        this functions help match result tyoe to return the expected result.
        If the Result value is the Ok variant, unwrap will return the value
        inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us

    */

    let m = File::open("hello.txt").unwrap();

    /*
        Another method is  expect("error msg") this function allows us to pass
        customize error message for panic 
    */
    let m = File::open("hello.txt").expect("Failed to open hello.txt");
    
    /*
        PROPAGATING ERRORS
        you can push code to handle error and result to the call stack
        the caller of the function by returning result type Result<T,E>
    */

    fn read_username_frm_file()->Result<String,io::Error>{
        
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file)=> file,
            // the retuen stament will help exit out of the function eith Err
            Err(e)=> return Err(e)
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e)
        }


    }

    /* 
    Shortcut for propagating Error
    Propagating error is returning Existing a function and returning an error 
    to the code that called the error .
    Basically its a fn with return type Result
    instead of using match and a lot of boiler plate code ther is a shortcut for 
    propagating erro which is ?
    the ? after a fucntion that returns result helps to perform a match oprtation
     returns T on Ok and returns Err(e) on e
    */

    // rewrite of the above function
    fn read_user_from_file () -> Result<String,io::Error>{

        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)

    }
}

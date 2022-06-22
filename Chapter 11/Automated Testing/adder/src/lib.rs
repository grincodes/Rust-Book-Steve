
/* 
    Note:
    $ cargo test -- --test-threads=1 (to not run tests in parralel)

    cargo test -- --nocapture (when you want printlin statement to show in test )

     cargo test -- --ignored (run ignored test)

     $ cargo test --test integration_test (running integration test)
*/


pub fn add_two(val:u32) ->u32{
    val+2
}
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self,other:&Rectangle)->bool{
        self.length > other.length && self.width > other.width     
    }
}


pub struct Guess {
    value:u32
}

impl Guess {
    pub fn new(value:u32)->Guess{
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        
        Guess { value }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // i  am ignoring this test
    #[test]
    #[ignore]
    fn make_test_fail() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
       let larger = Rectangle{length:5, width:6};
       let smaller = Rectangle{length:3, width:2};
        assert!(larger.can_hold(&smaller));

    }

    // ading cutom failure message
    // and now i ma ignoring this code
    #[test]
    #[ignore]
    fn custom_error_msg() {
        let err = "- Oga fix this thing";
        assert_eq!(2,4, "There is an arithmetic error here {}",err);
    }

    // expecting a code to panic use should panic

    #[test]
    #[should_panic]
    fn greater_than_100() {
       Guess::new(200);
    }

    // displaying error msessage when a code panics
    #[test]
    #[should_panic(expected = "iGuess value must be less than or equal to 100")]
    #[ignore]
    fn greater_than_n_100() {
       Guess::new(200);
    }
}

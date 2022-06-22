
mod network {
    fn connect(){

    }
}

mod client {
 fn connect() {
 }
}

// modules can be inside modules

mod network2 {
    fn connect(){

    }

    mod client2{
        fn connect(){

        }
    }
}

// moving modules in to thier files 

/*
we are telling rust to lookj for the module in another file */
pub mod client_ext; // this literally means contents of client_ext.rs

pub mod network_ext; // pub is used here to make the mod public 


/*
    Rules of Module Filesystems
Let’s summarize the rules of modules with regard to files:
•	 If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs.
•	 If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs.

*/

#[cfg(test)]
mod tests {
    use super::client_ext;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        //super::client_ext::connect(); we can now call the module becos of use
        client_ext::connect();
    }
}

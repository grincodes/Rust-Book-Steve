pub fn connect(){

}

pub mod server;

/*
    server.rs cant be put simply in the ssrc folde like the others because it is 
    a submodule of network_ext and there is no way for rust to know that its a sub module
    if it is placed in the same place as network ext, moreover ther could be 
    other module that has a server submodule too and this could casue conflict 
    as rust wont know how to map the differnt submodules to their appropriate 
    modules. hence athe parent submodule folder is created with an entry point of mod
    then the sub modules are placed in this folder using the normal concention
    rust defines modules using the normal file/folder hierachy system. 

*/

// mod server{
//     fn connect(){

//     }
// }
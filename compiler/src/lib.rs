pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/iron.rs"));  
}
pub mod ast;

pub use ast::greet_user;
mod  cli;
mod db;

use cli::parse;
use db::engine::Database;
use std::io::{self, Write};

fn main() {
    let mut db= Database::new();

    loop {
         print!("db> ");

         io::stdout().flush().unwrap();

         let mut input= String::new();

         io::stdin().read_line(&mut input).unwrap();

         match parse(input.trim()) {
               cli::Command::Set(k, v) => db.set(k, v),
            cli::Command::Get(k) => {
                match db.get(&k) {
                    Some(v) => println!("{}", v),
                    None => println!("(nil)"),
                }                
            }

            cli::Command::Get(k) => {
                match db.get(&k) {
                    Some(v) => println!("{}", v),
                    None => println!("(nil)"),
                }
            }
            cli::Command::Delete(k) => {
                if db.delete(&k) {
                    println!("deleted");
                } else {
                    println!("not found");
                }
            }
             cli::Command::Keys => {
                for key in db.keys() {
                    println!("{}", key);
                }
            }
            cli::Command::Unknown => {
                println!("unknown command");
            }
         }
    }
}

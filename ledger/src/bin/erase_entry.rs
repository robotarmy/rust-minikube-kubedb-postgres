extern crate ledger;
extern crate diesel;
extern crate bigdecimal;
extern crate uuid;

use self::ledger::*;
use self::models::*;

use self::diesel::prelude::*;
use std::io::{self};

use uuid::Uuid;

fn main() {
    use ledger::schema::ledger_entries::dsl::*;
    let connection = establish_connection();

    println!("What is the unique UUID of the entry?");
    let mut uuid_str = String::new();
    io::stdin().read_line(&mut uuid_str).expect("UUID required entry");
    uuid_str.pop(); // drop newline

    let uuid_result = Uuid::parse_str(&uuid_str);

    if uuid_result.is_ok() {
        let uuid = uuid_result.unwrap();
        let entry = ledger_entries.find(uuid)
            .first::<LedgerEntry>(&connection)
            .expect("Unable to load specified entry");

        println!("{}", entry);

        println!("Type \"erase\" to erase this entry.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Expect Credit Input");
        input.pop(); //newline

        if input == "erase" {
            println!("\nRemoving Entry '{}' with id '{}'", entry.title, entry.id);
            let result = delete_ledger_entry(&connection, &uuid);
            if result.is_ok() {
                println!("Entry Removed")
            } else {
                println!("Error: {}", result.unwrap())
            }
        } else {
            println!("\nOk. I'll Do Nothing.");
        }
    } else {
        println!("\nThat's not a uuid I recognize.");
    }

}

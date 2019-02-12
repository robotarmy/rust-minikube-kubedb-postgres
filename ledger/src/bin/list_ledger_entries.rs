extern crate ledger;
extern crate diesel;

use self::ledger::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use ledger::schema::ledger_entries::dsl::*;

    let connection = establish_connection();
    let results = ledger_entries
        .load::<LedgerEntry>(&connection)
        .expect("Error loading LedgerEntries");

    println!("Displaying {} entries", results.len());
    for entry in results {
        println!("{}", entry);
    }
}

extern crate ledger;
extern crate diesel;

use self::ledger::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use ledger::schema::ledger_entries::dsl::*;

    let connection = establish_connection();
    let results = ledger_entries.limit(5)
        .load::<LedgerEntry>(&connection)
        .expect("Error loading LedgerEntries");

    println!("Displaying {} entries", results.len());
    for entry in results {
        println!("ledger_entry/{}",entry.id);
        println!("\t{}", entry.title);
        println!("\t----------");
        match entry.debit {
            Some(val) =>
                println!("\tDebit:   {}", val),
            None => continue,
        };
        match entry.credit {
            Some(val) =>
                println!("\tCredit:  {}", val),
            None => continue,
        };
        println!("\tCreated: {}", entry.created_at);
        println!("");
    }
}

pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate bigdecimal;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{LedgerEntry,
                   NewLedgerEntry,
                   DebitAmount,
                   CreditAmount};

pub fn create_ledger_entry<'a>(
    conn: &PgConnection,
    title: &'a String,
    credit: &'a CreditAmount,
    debit: &'a DebitAmount
) -> LedgerEntry {
    use schema::ledger_entries;

    let new_entry = NewLedgerEntry {
        title: title,
        credit: credit,
        debit: debit,
    };

    diesel::insert_into(ledger_entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving Ledger Entry")
}

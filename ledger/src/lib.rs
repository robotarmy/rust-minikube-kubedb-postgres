pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate bigdecimal;
extern crate uuid;
use uuid::Uuid;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

use schema::ledger_entries;
use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn delete_ledger_entry(
    conn: &PgConnection,
    uuid: &Uuid
) -> Result<usize, Error> {
    diesel::delete(ledger_entries::table.find(uuid)).execute(conn)
}

pub fn create_ledger_entry<'a>(
    conn:   &PgConnection,
    title:  &'a String,
    credit: &'a CreditAmount,
    debit:  &'a DebitAmount
) -> LedgerEntry {

    let new_entry = NewLedgerEntry {
        title:  title,
        credit: credit,
        debit:  debit,
    };

    diesel::insert_into(ledger_entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving Ledger Entry")
}

extern crate ledger;
extern crate diesel;
extern crate bigdecimal;

use self::ledger::*;
use std::io::{self};

use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;

use self::models::*;

fn main() {
    let connection = establish_connection();

    println!("What is the title of this entry?");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Need a Title");
    title.pop(); // drop newline

    println!("What is the Debit Amount? [0]");
    let mut s_debit = String::new();
    io::stdin().read_line(&mut s_debit).expect("Expect Debit Input");
    s_debit.pop(); // newline

    let debit_result = BigDecimal::from_str(&s_debit); 
    let mut debit = BigDecimal::zero();
    if debit_result.is_ok() {
        debit = debit_result.unwrap();
    }

    println!("What is the Credit Amount? [0]");
    let mut s_credit = String::new();
    io::stdin().read_line(&mut s_credit).expect("Expect Credit Input");
    s_credit.pop(); //newline

    let credit_result = BigDecimal::from_str(&s_credit); 
    let mut credit = BigDecimal::zero();
    if credit_result.is_ok() {
        credit = credit_result.unwrap();
    }

    let entry = create_ledger_entry(&connection, &title, &CreditAmount(credit), &DebitAmount(debit));
    println!("\nCreated Ledger Entry '{}' with id  '{}'", title, entry.id);
}

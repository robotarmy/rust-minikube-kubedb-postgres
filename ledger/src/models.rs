
extern crate uuid;
extern crate bigdecimal;
extern crate chrono;

use self::uuid::Uuid;
use self::bigdecimal::BigDecimal;
use self::chrono::{Utc, DateTime};

use schema::ledger_entries;


// Todo: instead of making an alias
// make a newtype (trait?) so that
// the compiler doesn't allow
// a DebitAmount to be substituted
// for a CreditAmount
//
//https://github.com/diesel-rs/diesel/pull/429/commits/4b94471c584a406c8787671a0f4ceed48842e88c
pub type DebitAmount = BigDecimal;
pub type CreditAmount = BigDecimal;

#[derive(Queryable)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub title: String,
    pub debit: Option<DebitAmount>,
    pub credit: Option<CreditAmount>,
    // DateTimes in UTC Timezone
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Insertable)]
#[table_name = "ledger_entries"]
pub struct NewLedgerEntry<'a> {
    pub title: &'a String,
    pub debit: &'a DebitAmount,
    pub credit: &'a CreditAmount
}

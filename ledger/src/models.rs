extern crate uuid;
extern crate chrono;
extern crate bigdecimal;

use schema::ledger_entries;

use self::uuid::Uuid;
use self::bigdecimal::BigDecimal;
use self::chrono::{Utc, DateTime};



// Todo: instead of making an alias
// make a newtype (trait?) so that
// the compiler doesn't allow
// a DebitAmount to be substituted
// for a CreditAmount
//

macro_rules! define_newtype_for_diesel {
    ($new_type:ident, $wrapped_type:ty, $sql_type:tt, $diesel_db_field_type:ty, $diesel_db_type:ty) => (

        #[derive(Debug, FromSqlRow, AsExpression)]
        #[sql_type=$sql_type]//#[sql_type="Numeric"] // #[sql_type=""]
        pub struct $new_type(pub $wrapped_type);

        impl fmt::Display for $new_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                <$wrapped_type as fmt::Display>::fmt(&self.0, f)
            }
        }
        impl deserialize::FromSql<$diesel_db_field_type, $diesel_db_type> for $new_type {
            fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
                match <$wrapped_type as deserialize::FromSql<$diesel_db_field_type, $diesel_db_type>>::from_sql(bytes) {
                    Ok(val) => Ok($new_type(val)),
                    Err(thing) => Err(thing)
                }
            }
        }
        impl serialize::ToSql<$diesel_db_field_type, $diesel_db_type> for $new_type {
            fn to_sql<W: Write>(
                &self,
                out: &mut serialize::Output<W, $diesel_db_type>) -> serialize::Result {
                <$wrapped_type as serialize::ToSql<$diesel_db_field_type, $diesel_db_type>>::to_sql(
                    &self.0,
                    out
                )
            }
        }

    )
}

// Requirements for Macro define_newtype_for_diesel
use diesel::sql_types::Numeric;
use diesel::pg::Pg;
use diesel::serialize::{self};
use std::io::prelude::{Write};
use diesel::deserialize::{self};
use std::fmt::{self};

define_newtype_for_diesel!(DebitAmount, BigDecimal, "Numeric", Numeric, Pg);
define_newtype_for_diesel!(CreditAmount, BigDecimal, "Numeric", Numeric, Pg);



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

impl fmt::Display for LedgerEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-*-\tledger_entry/{}\n",self.id)?;
        write!(f, "   \t----------\n")?;
        write!(f, "\tTitle:   {}\n", self.title)?;
        match self.debit {
            Some(ref val) => write!(f, "\tDebit:   {}\n", val)?,
            None => (),
        };
        match self.credit {
            Some(ref val) => write!(f, "\tCredit:  {}\n", val)?,
            None => (),
        };
        write!(f, "\tCreated: {}\n", self.created_at)?;
        write!(f, "-*-\t----------\n")
    }
}



#[derive(Insertable, Queryable)]
#[table_name = "ledger_entries"]
pub struct NewLedgerEntry<'a> {
    pub title: &'a String,
    pub debit: &'a DebitAmount,
    pub credit: &'a CreditAmount
}
 

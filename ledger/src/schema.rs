table! {
    ledger_entries (id) {
        id -> Uuid,
        title -> Varchar,
        debit -> Nullable<Numeric>,
        credit -> Nullable<Numeric>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

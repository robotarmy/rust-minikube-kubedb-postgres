-- Your SQL goes here
CREATE TABLE ledger_entries (
id uuid primary key default uuid_generate_v4(),
title VARCHAR(840) NOT NULL,
debit DECIMAL(16,4),
credit DECIMAL(16,4),
created_at timestamp with time zone NOT NULL default current_timestamp,
updated_at timestamp with time zone
)

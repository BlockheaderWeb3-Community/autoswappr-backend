-- This SQL script creates a table to store information about ERC20 tokens,
-- including their contract address, name, symbol, decimal places and if stable.
create table token (
    contract_address varchar(66) primary key not null check (contract_address ~ '^0x[a-fA-F0-9]{40}$'),
    token_name varchar(255) not null,
    token_symbol varchar(10) not null check (length(token_symbol) <= 10),
    token_decimals smallint not null check (token_decimals between 0 and 18),
    is_stable boolean not null default false,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

SELECT trigger_updated_at('"token"');

create index on token(token_symbol);
create index idx_token_stable on token(is_stable) where is_stable = true;
create index idx_token_unstable on token(is_stable) where is_stable = false;

-- This SQL script defines a PostgreSQL table named swap_subscription to track
-- ERC20 token swap subscriptions.
create table swap_subscription(
    wallet_address varchar(42) primary key not null,
    to_token varchar(66) not null check (to_token ~ '^0x[a-fA-F0-9]{40}$'),
    is_active boolean not null default true,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

SELECT trigger_updated_at('"swap_subscription"');

create index on swap_subscription(to_token);

create table swap_subscription_from_token(
    wallet_address varchar(42) not null references swap_subscription(wallet_address) on delete cascade,
    from_token varchar(66) not null check (from_token ~ '^0x[a-fA-F0-9]{40}$'),
    percentage smallint not null check (percentage between 1 and 100),
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    primary key (wallet_address, from_token)
);

SELECT trigger_updated_at('"swap_subscription_from_token"');

-- This PostgreSQL table is designed to store a detailed log of ERC-20 token swap transactions.
-- Each transaction record is uniquely identified by a UUID transaction_id.
create table transactions_log(
    transaction_id uuid primary key default uuid_generate_v1mc(),
    wallet_address varchar(42) not null check (wallet_address ~ '^0x[a-fA-F0-9]{40}$'),
    from_token varchar(66) not null check (from_token ~ '^0x[a-fA-F0-9]{40}$'),
    to_token varchar(66) not null check (to_token ~ '^0x[a-fA-F0-9]{40}$'),
    percentage smallint not null check (percentage between 1 and 100),
    amount_from bigint not null check (amount_from > 0),
    amount_to bigint not null check (amount_to > 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

SELECT trigger_updated_at('"transactions_log"');

create index on transactions_log(wallet_address, amount_to, from_token, to_token, created_at);

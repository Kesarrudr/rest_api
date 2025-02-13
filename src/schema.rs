// @generated automatically by Diesel CLI.

diesel::table! {
    data_table (id) {
        id -> Text,
        data -> Jsonb,
    }
}

diesel::table! {
    _prisma_migrations (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::table! {
    miner (id) {
        id -> Uuid,
        address -> Uuid,
        nickname -> Text,
        hash_rate -> Int4,
        shares_mined -> Int4,
    }
}

diesel::table! {
    wallet (address) {
        address -> Uuid,
        club_name -> Text,
    }
}

diesel::joinable!(miner -> wallet (address));

diesel::allow_tables_to_appear_in_same_query!(data_table, _prisma_migrations, miner, wallet,);

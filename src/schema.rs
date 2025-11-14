// @generated automatically by Diesel CLI.

diesel::table! {
    datapod_events (id) {
        id -> BigSerial,
        event_type -> Varchar,
        datapod_id -> Varchar,
        seller -> Varchar,
        title -> Nullable<Varchar>,
        category -> Nullable<Varchar>,
        price_sui -> Nullable<BigInt>,
        kiosk_id -> Nullable<Varchar>,
        old_price -> Nullable<BigInt>,
        new_price -> Nullable<BigInt>,
        transaction_digest -> Varchar,
        checkpoint_sequence_number -> BigInt,
        event_index -> BigInt,
        timestamp -> BigInt,
        created_at -> Timestamp,
    }
}

diesel::table! {
    smart_contract_objects (id) {
        id -> BigSerial,
        object_id -> Varchar,
        object_type -> Varchar,
        owner -> Nullable<Varchar>,
        version -> BigInt,
        digest -> Varchar,
        content_type -> Nullable<Varchar>,
        data -> Nullable<Jsonb>,
        checkpoint_sequence_number -> BigInt,
        transaction_digest -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    transaction_digests (id) {
        id -> BigSerial,
        tx_digest -> Varchar,
        checkpoint_sequence_number -> BigInt,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    datapod_events,
    smart_contract_objects,
    transaction_digests,
);

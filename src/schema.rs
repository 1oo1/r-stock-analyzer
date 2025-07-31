// @generated automatically by Diesel CLI.

diesel::table! {
    llm_providers (id) {
        id -> Uuid,
        name -> Varchar,
        provider_type -> Varchar,
        api_key_encrypted -> Text,
        api_endpoint -> Nullable<Varchar>,
        model_name -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    llm_usage (id) {
        id -> Uuid,
        provider_id -> Uuid,
        user_id -> Uuid,
        tokens_used -> Int4,
        cost -> Nullable<Numeric>,
        request_type -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(llm_usage -> llm_providers (provider_id));
diesel::joinable!(llm_usage -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(llm_providers, llm_usage, users,);

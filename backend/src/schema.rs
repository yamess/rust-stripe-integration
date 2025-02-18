// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Int4,
        user_id -> Uuid,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        photo_url -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Int4,
        user_id -> Uuid,
        stripe_customer_id -> Varchar,
        stripe_price_id -> Varchar,
        stripe_product_id -> Varchar,
        stripe_subscription_id -> Varchar,
        status -> Varchar,
        has_used_trial -> Bool,
        current_period_end -> Nullable<Timestamptz>,
        cancel_at_period_end -> Bool,
        canceled_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        firebase_id -> Varchar,
        stripe_customer_id -> Nullable<Varchar>,
        status -> Varchar,
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(subscriptions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    profiles,
    subscriptions,
    users,
);

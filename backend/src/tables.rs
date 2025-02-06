use diesel::prelude::*;

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        firebase_id -> Varchar,
        stripe_customer_id -> Varchar,
        status -> Varchar,
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}
//
//
// table! {
//     profiles (id) {
//         id -> Int4,
//         user_id -> Uuid,
//         first_name -> Nullable<Varchar>,
//         last_name -> Nullable<Varchar>,
//         phone -> Nullable<Varchar>,
//         photo_url -> Nullable<Varchar>,
//         created_at -> Timestamptz,
//         updated_at -> Nullable<Timestamptz>,
//     }
// }
//
//
// table! {
//     plans (id) {
//         id -> Int4,
//         name -> Varchar,
//         description -> Nullable<Varchar>,
//         stripe_product_id -> Varchar,
//         price -> Numeric,
//         stripe_price_id -> Varchar,
//         created_at -> Timestamptz,
//         updated_at -> Nullable<Timestamptz>,
//     }
// }
//
// table! {
//     subscriptions (id) {
//         id -> Int4,
//         user_id -> Uuid,
//         plan_id -> Int4,
//         stripe_subscription_id -> Varchar,
//         status -> Varchar,
//         current_period_start -> Timestamptz,
//         current_period_end -> Nullable<Timestamptz>,
//         created_at -> Timestamptz,
//         updated_at -> Nullable<Timestamptz>,
//     }
// }
//
// joinable!(profiles -> users (user_id));
// joinable!(subscriptions -> plans (plan_id));
// joinable!(subscriptions -> users (user_id));
//
// allow_tables_to_appear_in_same_query!(
//     users,
//     profiles,
//     subscriptions,
//     plans,
// );
// //

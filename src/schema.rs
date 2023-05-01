// @generated automatically by Diesel CLI.

diesel::table! {
    product (product_id) {
        product_id -> Bigint,
        product_create_ts -> Nullable<Timestamp>,
        product_description -> Nullable<Varchar>,
        product_name -> Nullable<Varchar>,
        product_price -> Nullable<Decimal>,
        product_quantity -> Nullable<Bigint>,
        product_update_ts -> Nullable<Timestamp>,
    }
}

diesel::table! {
    product_order (order_id) {
        order_id -> Bigint,
        order_create_ts -> Nullable<Timestamp>,
        order_product_id -> Nullable<Bigint>,
        order_quantity -> Nullable<Bigint>,
        order_update_ts -> Nullable<Timestamp>,
        order_user_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    user (user_id) {
        user_id -> Bigint,
        user_name -> Nullable<Varchar>,
        user_address -> Nullable<Varchar>,
        user_alias -> Nullable<Varchar>,
        user_create_ts -> Nullable<Timestamp>,
        user_update_ts -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(product, product_order, user,);

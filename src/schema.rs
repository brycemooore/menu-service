// @generated automatically by Diesel CLI.

diesel::table! {
    item (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        available -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    item_ingredient (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        item_id -> Nullable<Int4>,
        available -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    menu (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    menu_category (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        menu_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    menu_category_item (id) {
        id -> Int4,
        item_id -> Nullable<Int4>,
        menu_category_id -> Nullable<Int4>,
    }
}

diesel::joinable!(item_ingredient -> item (item_id));
diesel::joinable!(menu_category -> menu (menu_id));
diesel::joinable!(menu_category_item -> item (item_id));
diesel::joinable!(menu_category_item -> menu_category (menu_category_id));

diesel::allow_tables_to_appear_in_same_query!(
    item,
    item_ingredient,
    menu,
    menu_category,
    menu_category_item,
);

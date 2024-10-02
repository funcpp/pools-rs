// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Int4,
        #[max_length = 511]
        name -> Varchar,
        parent_id -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        parent_id -> Nullable<Int4>,
        #[max_length = 24]
        author_id -> Bpchar,
        content -> Text,
        created_at -> Timestamptz,
        is_deleted -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        channel_id -> Int4,
        #[max_length = 24]
        author_id -> Bpchar,
        #[max_length = 1023]
        title -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> channels (channel_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    comments,
    posts,
);

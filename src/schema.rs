table! {
    posts (id) {
        id -> Int4,
        post_uuid -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_uuid -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        role -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(posts, users,);
